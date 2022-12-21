use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use clap::Parser;
use tokio::task;
use upwork_wl_bridge::{idle::Idle, opts::Opts, screenshot::Screenshot};
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

    task::spawn(Idle::start(last_active.clone()));

    let idle = Idle {
        opts: Opts::parse(),
        last_active,
    };

    let _ = ConnectionBuilder::session()?
        .name("org.gnome.Mutter.IdleMonitor")?
        .serve_at("/org/gnome/Mutter/IdleMonitor/Core", idle)?
        .build()
        .await?;

    loop {
        std::future::pending::<()>().await;
    }
}
