use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Focus on the next window (cycles)
    ///
    /// Note: cycles the windows, unlike the default "yabai -m window --focus next"
    Next,
    /// Focus on the previous window (cycles)
    Previous,
    /// Swap the current window with the next window (cycles)
    Swap,
    /// Set configuration options.
    Config {
        /// The name of the configuration option, i.e. "shift-size"
        name: String,
        /// The value of the configuration option, i.e. "20"
        value: String,
    },
    /// Resize the window's right border to the left
    Resize { direction: String },
}
