use std::fs;
use super::storage_path;

fn read() -> Result<Vec<String>, ()>{
    let path = storage_path();
    if !path.exists() {
        return Err(());
    }

    let store: Vec<String> = match fs::read(storage_path()) {
        Ok(lines) =>  lines
            .split(|&line| line == b'\n')
            .map(|line| String::from_utf8(line.to_vec()).unwrap())
            .collect(),
        _ => Vec::new()
    };

    Ok(store)
}

#[allow(unused)]
fn write(data: Vec<String>) -> Result<(), ()> {
    let path = storage_path();
    if !path.exists() {
        return Err(());
    }

    match fs::write(path, data.join("\n")) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn exists(pass: &str) -> Result<bool, ()> {
    let store: Vec<String> = read().unwrap();

    for line in store {
        if line.len() != 17 {
            continue;
        }
        if line.eq(pass) {
            return Ok(true)
        }
    }

    Ok(false)
}