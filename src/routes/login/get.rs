use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
struct HelloTemplate<'a> {
    auth_error: Option<&'a str>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct QueryParams {
    pub error_message: String,
}

pub async fn login_form(query: Option<web::Query<QueryParams>>) -> HttpResponse {
    let auth_error = query.as_ref().map(|q| q.error_message.as_str());
    let body = HelloTemplate { auth_error }.render().unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)
}
