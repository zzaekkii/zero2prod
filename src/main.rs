use zero2prod::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 주소 바인딩 실패 시 io::Error 발생.
    let address = "127.0.0.1:0";
    let listener = TcpListener::bind(address)?;

    run(listener)?.await // 그렇지 않다면 Server에 대해 .await
}
