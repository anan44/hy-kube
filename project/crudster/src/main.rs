#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate reqwest;

use rocket::response::NamedFile;
use std::{env, thread, io};
use rocket::State;
use std::path::Path;
use std::time::Duration;
use std::thread::sleep;
use std::fs::File;
use rocket_contrib::serve::StaticFiles;


struct AppState {
    image_path: String,
}

fn main() {
    let image_path = env_or_default("IMAGE_FILE_PATH", "../test_files/daily_image.jpg");
    let state = AppState {
        image_path: image_path.clone(),
    };

    thread::spawn(move || {
        loop {
            let sleep_time = fetch_image(&image_path);
            sleep(sleep_time);
        }
    });


    rocket::ignite()
        .manage(state)
        .mount("/", StaticFiles::from("./frontend"))
        .mount("/", routes![daily_image])
        .launch();
}

#[get("/daily_image")]
fn daily_image(state: State<AppState>) -> Option<NamedFile> {
    let image_path = Path::new(&state.image_path);
    NamedFile::open(image_path).ok()
}

fn fetch_image(image_file_path: &str) -> Duration {
    let mut resp = match reqwest::blocking::get("https://picsum.photos/1200") {
        Ok(resp) => {
            println!("Successfully to loaded new image! Next download in 24h");
            resp
        }
        Err(error) => {
            println!("Failed to load image! Trying again in 5secs");
            println!("{}", error);
            return Duration::from_secs(5);
        }
    };
    let mut out = File::create(image_file_path).expect("Failed to create the new image file");
    return match io::copy(&mut resp, &mut out) {
        Ok(_) => Duration::from_secs(60 * 60 * 24),
        Err(_) => {
            println!("Failed to write image to the file! Trying again in 5secs");
            return Duration::from_secs(15);
        }
    };
}

fn env_or_default(env_name: &str, default: &str) -> String {
    match env::var(env_name)
    {
        Err(_) => String::from(default),
        Ok(value) => value,
    }
}