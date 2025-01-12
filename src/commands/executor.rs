use crate::{context::CommandContext, error::DbCheckError};

use super::Command;


pub trait CommandExecutor {
    fn execute(&self, context: &mut CommandContext) -> Result<(), DbCheckError>;
}

// impl CommandExecutor for Command {
//     fn execute(&self, context: &mut CommandContext) -> Result<(), DbCheckError> {
//         let _ = match self {
//             Command::Login(command) => command.execute(context),
//             Command::Logout(command) => command.execute(context),
//             Command::Init(command) => command.execute(context),
//             Command::Courses(command) => command.execute(context),
//             _ => todo!(),
//         };
//         Ok(())
//     }
// }