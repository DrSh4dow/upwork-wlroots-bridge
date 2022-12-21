use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use async_process::{Command, Stdio};
use futures_lite::{io::BufReader, prelude::*};
use zbus::{dbus_interface, Result};

use crate::opts::Opts;

pub struct Idle {
    pub opts: Opts,
    pub last_active: Arc<Mutex<Instant>>,
}

impl Idle {
    pub async fn start(last_active: Arc<Mutex<Instant>>) -> Result<()> {
        let mut child = Command::new("swayidle")
            .arg("-w")
            .arg("timeout")
            .arg("1")
            .arg("echo timeout")
            .arg("resume")
            .arg("echo resume")
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn swayidle");

        let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
        while let Some(line) = lines.next().await {
            let line = line?;
            if line == *"resume" {
                let mut lock = last_active.lock().unwrap();
                *lock = Instant::now();
            }
        }
        Ok(())
    }
}

#[dbus_interface(name = "org.gnome.Mutter.IdleMonitor")]
impl Idle {
    fn get_idletime(&self) -> u64 {
        let lock = self.last_active.lock().unwrap();
        if self.opts.debug {
            println!("[ org.gnome.Mutter.IdleMonitor ] called get_idletime method");
            println!(
                "[ org.gnome.Mutter.IdleMonitor ] current idle time: {} milliseconds",
                lock.elapsed().as_millis()
            );
        }
        lock.elapsed().as_millis() as u64
    }
}
