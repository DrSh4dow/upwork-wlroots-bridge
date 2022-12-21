use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author = "Drsh4dow",
    version = "0.1.0",
    about = "Simple tool for making upwork screenshot request to work on wlroots"
)]
pub struct Opts {
    /// Show warning via zenity and pw-play before the screnshoot is taken
    #[clap(short = 'w', long = "warning", default_value_t = false)]
    pub show_warning: bool,

    /// Show debug messages
    #[clap(short = 'D', long = "debug", default_value_t = false)]
    pub debug: bool,
}
