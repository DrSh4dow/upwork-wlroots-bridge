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
    pub last_line: Arc<Mutex<String>>,
}

impl Idle {
    /// start the IO loop to watch out for swayidle messages and update the
    /// last_active time utilized by get_idletime
    pub async fn start(
        last_active: Arc<Mutex<Instant>>,
        last_line: Arc<Mutex<String>>,
    ) -> Result<()> {
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
                let mut active_lock = last_active.lock().unwrap();
                let mut line_lock = last_line.lock().unwrap();
                *active_lock = Instant::now();
                *line_lock = String::from("resume");
            } else if line == *"timeout" {
                let mut active_lock = last_active.lock().unwrap();
                let mut line_lock = last_line.lock().unwrap();
                *line_lock = String::from("timeout");
                *active_lock = Instant::now();
            }
        }
        Ok(())
    }
}

#[dbus_interface(name = "org.gnome.Mutter.IdleMonitor")]
impl Idle {
    fn get_idletime(&self) -> u64 {
        let lock = self.last_active.lock().unwrap();
        let last_line_lock = self.last_line.lock().unwrap();
        if self.opts.debug {
            println!("[ org.gnome.Mutter.IdleMonitor ] called get_idletime method");
            println!(
                "[ org.gnome.Mutter.IdleMonitor ] current idle time: {} seconds",
                lock.elapsed().as_secs()
            );
        }

        if *last_line_lock == *"resume" {
            0_u64
        } else {
            lock.elapsed().as_millis() as u64
        }
    }
}
