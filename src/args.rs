use crate::commands::*;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = None,
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
