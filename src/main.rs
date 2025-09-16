#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
mod endpoints;
pub mod template;
pub mod auth;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/assets", FileServer::from("./dist/assets"))
        .mount("/", routes![
            endpoints::index::handle,
            endpoints::auth::handle,
            endpoints::files::handle
        ])
}