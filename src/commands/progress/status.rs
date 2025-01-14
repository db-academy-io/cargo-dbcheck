use clap::Args;
use log::{debug, info};

use crate::{
    commands::{CommandContext, CommandExecutor},
    error::DbCheckError,
};

#[derive(Debug, Args)]
pub struct StatusCommand {
    /// Show completed topics
    #[arg(short, long, default_value_t = false)]
    pub show_completed: bool,
}

impl CommandExecutor for StatusCommand {
    fn execute(&self, context: &mut CommandContext) -> Result<(), DbCheckError> {
        debug!("Status command");
        let status = context.get_course_status()?;

        info!("Course: {0}", status.id);
        info!(
            "Current topic: {0}",
            status
                .current_topic
                .unwrap_or("No current topic".to_string())
        );

        if self.show_completed {
            if status.completed.is_empty() {
                info!("No completed topics");
            } else {
                info!("Completed topics");
                for topic in status.completed {
                    info!("  {0}", topic);
                }
            }
        }
        Ok(())
    }
}
