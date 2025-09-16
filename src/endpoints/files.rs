use std::{fs, path::{PathBuf, Path}, env};
use rocket::{http::CookieJar, serde::json::Json};
use serde::Serialize;
use crate::auth::jwt::verify;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Payload {
    path: String,
    files: Vec<(bool, String)>
}

#[derive(Responder)]
pub enum Response {
    #[response(status = 200, content_type = "json")]
    Data(Json<Payload>),

    #[response(status = 401)]
    Unauthorized(String),

    #[response(status = 404)]
    NotFound(String)
}

#[get("/_payload/<path..>", rank = 99)]
pub async fn handle(path: PathBuf, cookies: &CookieJar<'_>) -> Response {
    let explorer_data = env::var("EXPLORER_DATA").unwrap_or(String::from("./data"));
    let explorer_path = Path::new(explorer_data.as_str()).join(&path);

    if !explorer_path.exists() {
        return Response::NotFound("Not Found".to_string());
    }

    let token: Option<&rocket::http::Cookie<'static>> = cookies.get("token");
    if !path.starts_with("public/") {
        if !token.is_some() {
            return Response::Unauthorized(
                "Unauthorized".to_string()
            )
        }
        
        let token = token.unwrap();
        if !verify(token.value()) {
            cookies.remove("token");
            return Response::Unauthorized(
                "Unauthorized".to_string()
            )
        }
    }

    match fs::read_dir(&explorer_path) {
        Ok(data) => {
            let payload: Vec<(bool, String)> = data
                .filter(|entry| entry.is_ok())
                .map(|entry| {
                    let _path = entry.unwrap().path();

                    (_path.is_file(), 
                        path.join(
                            _path.file_name().unwrap()
                        ).to_str().unwrap().to_string())
                })
                .collect();

            Response::Data(Json(Payload {
                path: explorer_path.to_str().unwrap().to_string(),
                files: payload
            }))
        },
        _ => Response::NotFound("Not Found".to_string())
    }
}