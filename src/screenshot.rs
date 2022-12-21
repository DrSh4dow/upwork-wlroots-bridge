use async_process::Command;
use zbus::dbus_interface;

use crate::opts::Opts;

pub struct Screenshot {
    pub opts: Opts,
}

#[dbus_interface(name = "org.gnome.Shell.Screenshot")]
impl Screenshot {
    async fn screenshot(
        &self,
        include_cursor: bool,
        flash: bool,
        filename: &str,
    ) -> (bool, String) {
        if self.opts.debug {
            println!(
                "[ org.gnome.Shell.Screenshot ] called screenshot method with following args:"
            );
            println!(
                "include_cursor: {}, flash: {}, filename: {},",
                include_cursor, flash, filename
            );
        }

        if self.opts.show_warning {
            let _ = Command::new("pw-play")
                .arg("/usr/share/sounds/freedesktop/stereo/screen-capture.oga")
                .spawn();
            let _ = Command::new("zenity")
                .arg("--warning")
                .arg(
                    "--text=\"incoming screenshot, make sure to be on the corresponding worspace\"",
                )
                .output()
                .await;
        }

        let mut grim = Command::new("grim");
        if include_cursor {
            grim.arg("-c");
        }
        let out = grim.arg(filename).output().await.is_ok();

        (out, filename.into())
    }

    async fn screenshot_window(
        &self,
        include_frame: bool,
        include_cursor: bool,
        flash: bool,
        filename: &str,
    ) -> (bool, String) {
        if self.opts.debug {
            println!(
                "[ org.gnome.Shell.Screenshot ] called screenshot_window method with following args:"
            );
            println!(
                "include_frame: {}, include_cursor: {}, flash: {}, filename: {},",
                include_frame, include_cursor, flash, filename
            );
        }

        if self.opts.show_warning {
            let _ = Command::new("pw-play")
                .arg("/usr/share/sounds/freedesktop/stereo/screen-capture.oga")
                .spawn();
            let _ = Command::new("zenity")
                .arg("--warning")
                .arg("--text=\"incoming window capture, make sure to focus the correct window\"")
                .output()
                .await;
        }

        let mut grim = Command::new("grim");
        if include_cursor {
            grim.arg("-c");
        }

        let out = grim.arg(filename).output().await.is_ok();

        (out, filename.into())
    }

    async fn screenshot_area(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        flash: bool,
        filename: &str,
    ) -> (bool, String) {
        if self.opts.debug {
            println!(
                "[ org.gnome.Shell.Screenshot ] called screenshot_window method with following args:"
            );
            println!(
                "x: {}, y: {}, width: {}, height: {} flash: {}, filename: {},",
                x, y, width, height, flash, filename
            );
        }

        if self.opts.show_warning {
            let _ = Command::new("pw-play")
                .arg("/usr/share/sounds/freedesktop/stereo/screen-capture.oga")
                .spawn();
            let _ = Command::new("zenity")
                .arg("--warning")
                .arg("--text=\"incoming Area capture, Go to the correct workspace\"")
                .output()
                .await;
        }

        let out = Command::new("grim")
            .arg("-g")
            .arg(format!("{x},{y} {width}x{height}"))
            .arg(filename)
            .output()
            .await
            .is_ok();

        (out, filename.into())
    }
}
