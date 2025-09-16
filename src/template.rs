use std::fs::read;

pub fn folder() -> String {
    let bytes = read("./dist/folder/index.html")
        .expect("Failed to load folder/index.html template");

    String::from_utf8(bytes).unwrap()
}

pub fn login() -> String {
    let bytes = read("./dist/login/index.html")
        .expect("Failed to load login/index.html template");

    String::from_utf8(bytes).unwrap()
}

pub fn not_found() -> String {
    let bytes = read("./dist/404/index.html")
        .expect("Failed to load 404/index.html template");

    String::from_utf8(bytes).unwrap()
}