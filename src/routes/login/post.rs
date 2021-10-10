use crate::auth::{validate_credentials, Credentials};
use crate::routes;
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
        Err(e) => {
            let query_parameters = routes::login::get::QueryParams {
                error_message: e.to_string(),
            };
            let query_string = serde_qs::to_string(&query_parameters).unwrap();
            HttpResponse::SeeOther()
                .insert_header((LOCATION, format!("/login?{}", query_string)))
                .finish()
        }
    }
}
