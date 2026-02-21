use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short='n', long, default_value="false")]
    pub dry_run: bool,
    #[arg(short, long)]
    pub tags: Vec<String>,
    /// Target directory path new note will be created.
    pub dir: PathBuf
}



