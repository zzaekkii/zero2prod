use std::net::TcpListener;

// 애플리케이션 인스턴스 새로 실행하고 주소(ex: https://localhost:xxxx) 반환.
// .await 호출이 없으므로 async가 아니어도 됨.
fn spawn_app() -> String {
    // todo!() // 문제 발생 -> 테스트 실패.
    // 0번 포트로 사용 가능한 포트 OS에 요청.
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("랜덤 포트 바인딩 실패!");

    // OS가 할당한 포트 번호.
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address.");
    // 기존 .await의 문제점: 무한 대기 & 반환 X.

    let _ = tokio::spawn(server);
    // _ 언더바는 반환 결과를 무시. 의도적으로 사용하지 않음을 명시.

    // 애플리케이션 주소 반환.
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test] // 테스팅에선 기능적으로 tokio::main과 동등.
async fn health_check_works() {
    // Arrange
    // spawn_app().await.expect("Failed to spawn our app.");
    // 이제는 .await, .expect를 호출하지 않음.
    let address = spawn_app();

    // 'reqwest'를 이용해 application에 대한 http 요청 수행.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
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
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=최%20재영&email=kcj1607%40naver.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=최%20재영", "이메일 누락"),
        ("email=kcj1607%40naver.com", "이름 누락"),
        ("", "둘 다 미아"),
    ];

    for (invalid_body, error_msg) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
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