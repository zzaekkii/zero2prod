#[tokio::test] // 테스팅에선 기능적으로 tokio::main과 동등.
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn our app.");

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

// 백그라운드에서 application 구동.
async fn spawn_app() -> std::io::Result<()> {
    // todo!() // 문제 발생 -> 테스트 실패.
    zero2prod::run().await // await -> 무한 대기, 반환 X
}
