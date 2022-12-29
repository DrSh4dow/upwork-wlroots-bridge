use async_process::Command;
use regex::Regex;
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
                .output()
                .await;
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
                .output()
                .await;
            let _ = Command::new("zenity")
                .arg("--warning")
                .arg("--text=\"incoming window capture, make sure to focus the correct window (you have 2 secs once you close this warning)\"")
                .output()
                .await;

            let _ = Command::new("sleep").arg("2").output().await;
        }

        let mut grim = Command::new("grim");
        if include_cursor {
            grim.arg("-c");
        }

        // ask current window coordinates and position
        if let Ok(swaymsg) = Command::new("swaymsg")
            .arg("-t")
            .arg("get_tree")
            .output()
            .await
        {
            if let Ok(tree) = String::from_utf8(swaymsg.stdout) {
                let raw_json: &str = &tree
                    .replace(['\n', '\r', '\t', ' '], "")
                    .split("},{")
                    .filter(|v| v.contains("\"focused\":true"))
                    .collect::<Vec<&str>>()
                    .join("");

                let re = Regex::new(
                    r#""rect".*"x":(\d*),"y":(\d*),"width":(\d*),"height":(\d*).*,"deco_rect":"#,
                )
                .unwrap();
                if let Some(cap) = re.captures(raw_json) {
                    if self.opts.debug {
                        println!(
                            "Focused window coordinates: x: {}, y: {}, w: {}, h: {}",
                            &cap[1], &cap[2], &cap[3], &cap[4]
                        );
                    }
                    grim.arg("-g")
                        .arg(format!("{},{} {}x{}", &cap[1], &cap[2], &cap[3], &cap[4]));
                }
            }
        };

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
                .output()
                .await;
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
