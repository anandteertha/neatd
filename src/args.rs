use clap::{Parser, Subcommand};

///neatd - automatic folder organizer daemon
#[derive(Parser, Debug)]
#[command(name = "neatd")]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create default config at ~/.neatd/config.toml
    Init,

    /// Start neatd (watch folders and organize)
    Run {
        /// Run once and exit(no watching)
        #[arg(long)]
        once: bool,

        /// Run as a background daemon
        #[arg(long)]
        daemon: bool,
    },

    /// Show what neatd WOULD do without moving files
    DryRun,

    /// Show whether daemon is running and summary stats
    Status,
}
