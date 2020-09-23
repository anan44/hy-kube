use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use chrono::Utc;
use std::thread::sleep;
use std::time::Duration;
use std::env;

fn main() {
    let file_path = env_or_default("TIME_FILE_PATH", "../test_files/time.txt");
    loop {
        let utc = Utc::now();
        write_to_file(file_path.as_str(), format!("{:?}", utc));
        sleep(Duration::from_secs(5));
    }
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