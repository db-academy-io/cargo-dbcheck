#![warn(clippy::all)]
mod args;
mod commands;
mod context;
mod course;
mod error;

use crate::args::Cli;
use commands::CommandExecutor;
use context::CommandContext;
use error::DbCheckError;
use fern::Dispatch;
use log::{error, LevelFilter};
use std::env;

use clap::Parser;

fn main() {
    init_logger().unwrap();

    let mut args: Vec<String> = env::args().collect();

    // In case of calling dbcheck from cargo, cargo will send
    // subcommand name as 1st argument as a parameter
    // which confuses the clap library
    if args.len() > 1 && args[1] == "dbcheck" {
        // Remove the "dbcheck" argument
        args.remove(1);
    }

    let cli = Cli::parse_from(args);
    let config = ".db-academy/config.json";
    let mut context = CommandContext::new(config);
    if let Err(e) = cli.command.execute(&mut context) {
        error!("Error: {}", e);
    }
}

fn init_logger() -> Result<(), DbCheckError> {
    let base_config = Dispatch::new();

    let logfile = fern::log_file("log/output.log").map_err(DbCheckError::IO)?;
    let file_config = Dispatch::new().level(LevelFilter::Debug).chain(logfile);

    let stdout_config = Dispatch::new()
        .level(LevelFilter::Info)
        .chain(std::io::stdout());

    let formatter =
        |out: fern::FormatCallback, message: &std::fmt::Arguments, record: &log::Record| {
            let target = record
                .target()
                .split("::")
                .last()
                .unwrap_or("")
                .to_uppercase();

            out.finish(format_args!(
                "[{target}][{level}] {message}",
                level = record.level(),
                target = target,
                message = message
            ));
        };

    base_config
        .format(formatter)
        .chain(file_config)
        .chain(stdout_config)
        .apply()
        .map_err(|e| DbCheckError::InternalError(e.to_string()))?;
    Ok(())
}
