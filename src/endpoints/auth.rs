use time::{Duration, OffsetDateTime};

use rocket::{http::{private::cookie::CookieBuilder, CookieJar}, response::Redirect};
use crate::{auth::{jwt::{sign, verify}, store::exists}};

#[derive(Responder)]
pub enum Response {
    #[response(status = 200)]
    Logged(String),

    #[response(status = 400)]
    Failed(String),

    #[response(status = 304)]
    AlreadyLogged(Redirect)
}


#[post("/login", data = "<payload>")]
pub async fn handle(payload: String, cookies: &CookieJar<'_>) -> Response {
    if cookies.get("token").is_some() && 
        verify(cookies.get("token").unwrap().value()) {
        return Response::AlreadyLogged(Redirect::to("/"));
    }

    let passed = exists(&payload).unwrap();
    if !passed {
        return Response::Failed("no".to_string());
    }

    
    let token = sign().unwrap();
    let cookie = CookieBuilder::new("token", token)
        .domain("localhost")
        .expires(
            OffsetDateTime::now_utc() + Duration::days(7)
        )
        .path("/");

    cookies.add(cookie);

    Response::Logged("ok".to_string())
}