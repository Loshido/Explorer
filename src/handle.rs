use std::{path::{PathBuf, Path}, env};
use rocket::{fs::NamedFile, http::CookieJar};
use crate::{template, auth::jwt::verify};

#[derive(Responder)]
pub enum Response {
    #[response(status = 200, content_type = "html")]
    HTML(String),

    #[response(status = 200)]
    File(Option<NamedFile>),

    #[response(status = 404, content_type = "html")]
    NotFound(String)
}

#[get("/<path..>", rank = 100)]
pub async fn handler(path: PathBuf, cookies: &CookieJar<'_>) -> Response {
    let explorer_data = env::var("EXPLORER_DATA").unwrap_or(String::from("./data"));
    let explorer_path = Path::new(explorer_data.as_str()).join(&path);

    let token = cookies.get("token");
    println!("path: {:?}", path);
    if !path.starts_with("public/") {
        if !token.is_some() {
            return Response::HTML(
                template::login()
            )
        }

        let token = token.unwrap();
        if !verify(token.value()) {
            cookies.remove("token");
            return Response::HTML(
                template::login()
            )
        }
    }

    if explorer_path.is_dir() {
        Response::HTML(template::folder())
    } else if explorer_path.is_file() {
        Response::File(
            NamedFile::open(explorer_path).await.ok()
        )
    } else {
        Response::NotFound(template::not_found())
    }
}