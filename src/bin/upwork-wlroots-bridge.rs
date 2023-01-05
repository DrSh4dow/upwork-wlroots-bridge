use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use clap::Parser;
use tokio::task;
use upwork_wlroots_bridge::{idle::Idle, opts::Opts, screenshot::Screenshot};
use zbus::{ConnectionBuilder, Result};

#[tokio::main()]
async fn main() -> Result<()> {
    let screenshot = Screenshot {
        opts: Opts::parse(),
    };

    let _ = ConnectionBuilder::session()?
        .name("org.gnome.Shell.Screenshot")?
        .serve_at("/org/gnome/Shell/Screenshot", screenshot)?
        .build()
        .await?;

    let last_active = Arc::new(Mutex::new(Instant::now()));
    let last_line = Arc::new(Mutex::new(String::from("resume")));

    task::spawn(Idle::start(last_active.clone(),last_line.clone()));

    let idle = Idle {
        opts: Opts::parse(),
        last_active,
        last_line
    };

    let _ = ConnectionBuilder::session()?
        .name("org.gnome.Mutter.IdleMonitor")?
        .serve_at("/org/gnome/Mutter/IdleMonitor/Core", idle)?
        .build()
        .await?;

    println!("DBUS Server initialized successfully!");

    loop {
        std::future::pending::<()>().await;
    }
}
