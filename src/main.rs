#![warn(clippy::all)]
mod args;

use crate::args::Cli;
use std::env;

use clap::Parser;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // In case of calling dbcheck from cargo, cargo will send
    // subcommand name as 1st argument as a parameter
    // which confuses the clap library
    if args.len() > 1 && args[1] == "dbcheck" {
        // Remove the "dbcheck" argument
        args.remove(1);
    }

    let args = Cli::parse_from(args);
    println!("{:?}", args);
}
