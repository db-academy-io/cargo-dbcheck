#![warn(clippy::all)]
mod args;

use crate::args::Cli;

use clap::Parser;

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}
