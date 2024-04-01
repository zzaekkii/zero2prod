#[tokio::test] // 테스팅에선 기능적으로 tokio::main과 동등.
async fn health_check_works() {
    // Arrange
    // spawn_app().await.expect("Failed to spawn our app.");
    // 이제는 .await, .expect를 호출하지 않음.
    spawn_app();

    // 'reqwest'를 이용해 application에 대한 http 요청 수행.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// .await 호출이 없으므로 async가 아니어도 됨.
fn spawn_app() {
    // todo!() // 문제 발생 -> 테스트 실패.
    let server = zero2prod::run().expect("Failed to bind address.");
    // 기존 .await의 문제점: 무한 대기 & 반환 X.

    let _ = tokio::spawn(server);
}
