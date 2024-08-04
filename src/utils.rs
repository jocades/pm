use std::env;
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

pub fn get_dir() -> PathBuf {
    match env::var("HOME") {
        Ok(home) => PathBuf::from(home).join(".pm"),
        Err(_) => {
            eprintln!("Error: $HOME not found");
            process::exit(1);
        }
    }
}

pub fn get_time() -> String {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Start { cmd: String, name: Option<String> },
    Stop { name: String },
    Info { name: String },
}
