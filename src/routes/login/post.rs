use crate::auth::{validate_credentials, Credentials};
use crate::routes::login::get::AUTH_ERROR_COOKIE_NAME;
use actix_web::cookie::{Cookie, SameSite};
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
    match validate_credentials(
        Credentials {
            username: form.username,
            password: form.password,
        },
        &pool,
    )
    .await
    {
        Ok(_user_id) => HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish(),
        Err(e) => HttpResponse::SeeOther()
            .cookie(
                Cookie::build(AUTH_ERROR_COOKIE_NAME, e.to_string())
                    .path("/login")
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Strict)
                    .max_age(time::Duration::seconds(60 * 5))
                    .finish(),
            )
            .insert_header((LOCATION, "/login"))
            .finish(),
    }
}
