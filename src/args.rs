use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(required = true)]
    /// A list of files to be assembled.
    pub file: Vec<PathBuf>,
}
