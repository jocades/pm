#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::{collections::HashMap, path::Path};
use sysinfo::{Pid, Process, ProcessesToUpdate, System};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Clone, Tabled)]
pub struct Task {
    pub name: String,
    pub pid: u32,
    pub command: String,
    pub cpu_usage: f32,
    pub mem_usage: u64,
}

#[derive(Debug)]
pub struct State {
    pub db: HashMap<String, Task>,
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
