#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
mod handle;
pub mod template;
pub mod auth;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/assets", FileServer::from("./dist/assets"))
        .mount("/", routes![
            handle::handler, 
        ])
}