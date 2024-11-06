use std::fmt::format;
use std::net::TcpListener;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::sync::BarrierWaitResult;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// 애플리케이션 인스턴스 새로 실행하고 주소(ex: https://localhost:xxxx) 반환.
async fn spawn_app() -> TestApp {
    // 0번 포트로 사용 가능한 포트 OS에 요청.
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("랜덤 포트 바인딩 실패!");

    // OS가 할당한 포트 번호.
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string(); // 테스트 편하게.
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address.");
    let _ = tokio::spawn(server);
    // _ 언더바는 반환 결과를 무시. 의도적으로 사용하지 않음을 명시.

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(
        &config.connection_string_without_db()
    )
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}

#[tokio::test] // 테스팅에선 기능적으로 tokio::main과 동등.
async fn health_check_works() {
    // Arrange
    // spawn_app().await.expect("Failed to spawn our app.");
    // 이제는 .await, .expect를 호출하지 않음.
    let app = spawn_app().await;

    // 'reqwest'를 이용해 application에 대한 http 요청 수행.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=최%20재영&email=kcj1607%40naver.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "kcj1607@naver.com");
    assert_eq!(saved.name, "최 재영");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=최%20재영", "이메일 누락"),
        ("email=kcj1607%40naver.com", "이름 누락"),
        ("", "둘 다 미아"),
    ];

    for (invalid_body, error_msg) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "{} API가 400: BAD_REQUEST로 실패한 게 아니고 테스트가 실패.",
            error_msg
        );
    }
}