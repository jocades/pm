use std::fs::{self, File};
use std::time::Instant;

use log::info;
use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};
use std::{thread, time::Duration};

use sysinfo::{
    Components, Disks, Networks, Pid, Process, ProcessesToUpdate, System,
};

fn main() {
    env_logger::init();
    // let content = fs::read("pm.config.json").unwrap();
    // println!("{:?}", &content[..]);
    // let v: Value = serde_json::from_slice(&content).unwrap();
    //
    // println!("{}", v["other"])

    let mut sys = System::new();

    let mut command = Command::new("bun");

    let stdout = File::create("test/pm.out").unwrap();
    let stderr = File::create("test/pm.err").unwrap();

    command
        .args(["run", "server.js"])
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));

    let mut child = command.spawn().unwrap();

    let pid = Pid::from(child.id() as usize);
    println!("PID: {pid}");

    let mut pidf = File::create("test/pid").unwrap();
    writeln!(pidf, "{}", pid).unwrap();

    let alive = Duration::from_secs(10);
    let now = Instant::now();

    loop {
        sys.refresh_processes(ProcessesToUpdate::Some(&[pid]));
        let p = sys.process(pid).unwrap();
        info!(
            "server ({pid}), cpu: {}, mem: {}",
            p.cpu_usage(),
            p.memory()
        );

        thread::sleep(Duration::from_secs(1));

        if now.elapsed() >= alive {
            break;
        }
    }

    child.kill().unwrap();
}
