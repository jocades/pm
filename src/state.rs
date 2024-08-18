#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::{collections::HashMap, path::Path};
use sysinfo::{Pid, Process, ProcessesToUpdate, System};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub command: String,
    pub args: Vec<String>,
    pub log_file: PathBuf,
    pub cpu_usage: f32,
    pub mem_usage: u64,
}

#[derive(Debug)]
pub struct State {
    pub db: HashMap<String, ProcessInfo>,
    pub sys: System,
    pub path: PathBuf,
}

impl State {
    pub fn new(path: &Path) -> crate::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(State {
            db: HashMap::new(),
            sys: System::new(),
            path: path.into(),
        })
    }
}
