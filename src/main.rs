#![warn(clippy::all)]
mod args;
mod commands;

use crate::args::Cli;
use commands::{CommandContext, CommandExecutor};
use std::env;

use clap::Parser;

fn main() -> Result<(), anyhow::Error> {
    let mut args: Vec<String> = env::args().collect();

    // In case of calling dbcheck from cargo, cargo will send
    // subcommand name as 1st argument as a parameter
    // which confuses the clap library
    if args.len() > 1 && args[1] == "dbcheck" {
        // Remove the "dbcheck" argument
        args.remove(1);
    }

    let cli = Cli::parse_from(args);
    let config = ".db-academy-io.json";
    let mut context = CommandContext { _config: config };
    let result = cli.command.execute(&mut context);
    println!("{:?}", result);

    Ok(())
}
