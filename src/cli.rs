use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "ruby-format")]
#[command(version = "0.1")]
pub struct Cli {
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    pub targets: Vec<PathBuf>,

    #[arg(long)]
    pub display_time: bool,
}
