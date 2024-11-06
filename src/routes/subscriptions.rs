use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::types::chrono;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    // async 함수에서 enter()쓰면 망한다고 함.
    // 근데 span을 활성화하려면 enter()로 직접 개입해야 함.
    // let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
        // 'get_ref()'로 web::Data로 감싸진 PgConnection에 대한 불변 참조 획득.
        .execute(pool.get_ref())
        // 인스트루멘테이션을 먼저 붙이고 대기.
        .instrument(query_span)
        .await
        {
        Ok(_) => {
            tracing::info!("request_id {} - Subscribed successfully", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
