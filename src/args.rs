use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    Init {
        /// Set custom path for config file
        #[arg(long)]
        path: Option<PathBuf>,

        /// force to rewrite the config file
        #[arg(long)]
        force: bool,
    },

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

    /// Validate if the config file generated is correct or not and prints a formatted output
    #[command(long_about = "
    Validate your configuration and rules without making changes.
    Checks config syntax, required fields, paths/permissions, and rule conflicts.
    Reports actionable errors and warnings; exits non-zero on failure.")]
    Validate,
}
