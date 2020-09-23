#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use uuid::Uuid;
use std::fs::File;
use std::io::prelude::*;
use rocket::State;
use std::path::Path;
use std::env;

struct UuidState {
    uuid: Uuid,
    time_file_path: String,
    count_file_path: String,
}

fn main() {
    let my_uuid = Uuid::new_v4();
    let time_file_path = env_or_default("TIME_FILE_PATH", "../test_files/time.txt");
    let count_file_path = env_or_default("COUNT_FILE_PATH", "../test_files/count.txt");
    let uuid_state = UuidState {
        uuid: my_uuid,
        time_file_path,
        count_file_path,
    };

    rocket::ignite()
        .mount("/", routes![get_hash])
        .manage(uuid_state)
        .launch();
}

#[get("/")]
fn get_hash(state: State<UuidState>) -> String {
    let time = read_text_from_file(state.inner().time_file_path.as_str());
    let count = read_text_from_file(state.inner().count_file_path.as_str());
    let uuid = state.inner().uuid.to_string();
    return format!("{} - {}\nPing / Pongs: {}", time, uuid, count)
}

fn read_text_from_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(_) => return String::from("--"),
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(_) => return String::from("--"),
        Ok(_) => print!("{} contains: {}", display, content),
    }
    return content;

}

fn env_or_default(env_name: &str, default: &str) -> String {
    match env::var(env_name)
    {
        Err(_) => String::from(default),
        Ok(value) => value,
    }
}