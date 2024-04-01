use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 주소 바인딩 실패 시 io::Error 발생.
    run()?.await // 그렇지 않다면 Server에 대해 .await
}
