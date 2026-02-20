use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short='n', long, default_value="false")]
    dry_run: bool,
    #[arg(short, long)]
    tags: Vec<String>,
    /// Target directory path new note will be created.
    dir: PathBuf
}



