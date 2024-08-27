use crate::cmd::Executor;
use crate::State;
use crate::{Command, Connection, Error, LOCAL_HOST};

use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{env, fs};
use sysinfo::System;
use tokio::net::{TcpListener, TcpStream};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T = Value> {
    pub ok: bool,
    pub data: T,
}

// create e respone builder
impl<T: Serialize> Response<T> {
    pub fn new(ok: bool, data: T) -> Response<T> {
        Response { ok, data }
    }

    pub fn ok(data: T) -> Self {
        Self::new(true, data)
    }

    pub fn err(data: T) -> Self {
        Self::new(false, data)
    }
}

pub async fn run(port: u16) -> crate::Result<()> {
    /* tokio::task::spawn_blocking(|| {
            let mut sys = System::new();
            loop {
                sys.refresh_cpu_usage();
                let cpus: Vec<_> =
                    sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
                // let _ = tx.send(cpus);
                println!("{:?}", cpus);
                std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            }
        });
    */
    let listener = TcpListener::bind(format!("{LOCAL_HOST}:{port}")).await?;

    info!("Server started with pid: {}", std::process::id());
    info!("PM_DIR: {:?}", env::var("PM_DIR"));
    info!("Listening on port {port}");

    let path = Path::new(env!("HOME")).join(".pm");
    let state = Arc::new(Mutex::new(State::new(&path)?));

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        info!("Accepted {addr}");
        tokio::spawn(async move {
            if let Err(e) = handle(stream, state).await {
                error!("Connection error: {e}");
            }
        });
    }
}

async fn handle(stream: TcpStream, s: Arc<Mutex<State>>) -> crate::Result<()> {
    let mut conn = Connection::new(stream);

    match conn.read::<Command>().await {
        Ok(None) => Ok(()),
        Ok(Some(cmd)) => {
            {
                let mut state = s.lock().unwrap();
                state.sys.refresh_all();
                println!("=> system:");
                // RAM and swap information:
                println!("total memory: {} bytes", state.sys.total_memory());
                println!("used memory : {} bytes", state.sys.used_memory());

                let mem = state.sys.used_memory() as f32
                    / state.sys.total_memory() as f32
                    * 100.0;

                println!("used memory : {:.2}%", mem);

                println!("total swap  : {} bytes", state.sys.total_swap());
                println!("used swap   : {} bytes", state.sys.used_swap());

                // Display system information:
                println!("System name:             {:?}", System::name());
                println!(
                    "System kernel version:   {:?}",
                    System::kernel_version()
                );
                println!("System OS version:       {:?}", System::os_version());
                println!("System host name:        {:?}", System::host_name());
            }

            cmd.execute(s, &mut conn).await
        }
        Err(e) => {
            if let Error::Parse(e) = e {
                conn.write(&json!({"error": e})).await
            } else {
                Err(e)
            }
        }
    }
}

async fn restore(s: &mut State) -> crate::Result<()> {
    // get all the pids and run the commands

    /* let pids = fs::read_dir(s.path.join("pids"))?
    .filter_map(|entry| {
        entry.ok().and_then(|e| {
            e.file_name().into_string().ok().and_then(|s| {
                s.strip_suffix(".pid").and_then(|_| s.parse::<u32>().ok())
            })
        })
    })
    .collect::<Vec<_>>(); */

    Ok(())
}
