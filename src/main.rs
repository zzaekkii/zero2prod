use std::net::TcpListener;
use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use tracing_log::LogTracer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // // 환경 변수 없을 시, info 레벨 이상의 로그 출력.
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 모든 log 이벤트를 구독자에게 리다이렉트.
    LogTracer::init().expect("Failed to set logger");

    // RUST_LOG 환경변수 미설정 시, info 레벨 이상 모든 span 출력.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // 포맷이 적용된 span들을 stdout으로 출력.
        std::io::stdout,
    );

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // 애플리케이션에서 span을 처리하기 위해 어떤 subscriber를 사용해야하는지 지정 가능.
    set_global_default(subscriber).expect("Failed to set subscriber");


    // 구성값 읽어오기.
    let configuration = get_configuration().expect("Failed to read configuration.");

    // 포트는 구성값에서 꺼내오기.
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    // db connection.
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    )
    .await
    .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}
