#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::Mutex;
use rocket::State;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::env;

struct AppState {
    count: Mutex<u32>,
    file_path: String,
}

fn main() {
    let app_state = AppState {
        count: Mutex::new(0),
        file_path: env_or_default("COUNT_FILE_PATH", "../test_files/count.txt"),
    };

    rocket::ignite()
        .mount("/", routes![pingpong])
        .manage(app_state)
        .launch();
}

#[get("/")]
fn pingpong(state: State<AppState>) -> String {
    let mut count = state.inner().count.lock().unwrap();
    let file_path = state.inner().file_path.as_str();
    *count += 1;
    write_to_file(file_path, count.to_string());
    count.to_string()
}

fn write_to_file(file_path: &str, text: String) {
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display, why),
        Ok(_) => println!("Successful write to {} -> {}", display, text)
    }
}

fn env_or_default(env_name: &str, default: &str) -> String {
    match env::var(env_name)
    {
        Err(_) => String::from(default),
        Ok(value) => value,
    }
}
