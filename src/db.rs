use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use sysinfo::Pid;

pub type Db = Arc<Mutex<HashMap<String, Pid>>>;

/* pub struct Db {
    mutex: Arc<Mutex<HashMap<String, Pid>>>,
}

impl Db {
    pub fn new() -> Db {
        Db {
            mutex: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Pid> {
        let mut lock = self.mutex.lock().unwrap();
        lock.get(key)
    }

    pub fn set<T: Into<String>>(&self, key: T, value: Pid) {
        let mut lock = self.mutex.lock().unwrap();
        lock.insert(key.into(), value);
    }
} */
