use crate::auth::{validate_credentials, Credentials};
use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::HttpResponse;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: String,
}

pub async fn submit_login(pool: web::Data<PgPool>, form: web::Form<FormData>) -> HttpResponse {
    let form = form.0;
    let _user_id = validate_credentials(
        Credentials {
            username: form.username,
            password: form.password,
        },
        &pool,
    )
    .await
    .unwrap_or_default();
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
