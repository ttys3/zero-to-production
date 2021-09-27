use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use tera::{Context, Tera};

pub async fn login_form(templates: web::Data<Tera>) -> HttpResponse {
    let body = templates
        .render("login/login.html", &Context::new())
        .unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)
}
