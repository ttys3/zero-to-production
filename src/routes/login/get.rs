use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};
use askama::Template;

pub const AUTH_ERROR_COOKIE_NAME: &str = "auth_error";

#[derive(Template)]
#[template(path = "login.html")]
struct HelloTemplate<'a> {
    auth_error: Option<&'a str>,
}

pub async fn login_form(request: HttpRequest) -> HttpResponse {
    let cookie = request.cookie(AUTH_ERROR_COOKIE_NAME);
    let auth_error = cookie.as_ref().map(|c| c.value());
    let body = HelloTemplate { auth_error }.render().unwrap();
    let mut response = HttpResponse::Ok();
    if let Some(cookie) = cookie {
        response.del_cookie(&cookie);
    }
    response.content_type(ContentType::html()).body(body)
}
