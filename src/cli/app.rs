use std::path::PathBuf;
use clap::{ Parser, Subcommand };

#[derive(Debug, Parser)]
#[command(author, about, version)]
pub struct App {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Install a local Typst package
    Install {
        /// Package directory
        package_dir: PathBuf,
    },
}
