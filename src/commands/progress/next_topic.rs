use clap::Args;
use log::{debug, error, info};

use crate::{
    commands::{CommandContext, CommandExecutor},
    error::DbCheckError,
};

#[derive(Debug, Args)]
pub struct NextTopicCommand {
    /// Mark current topic as completed
    #[arg(short, long, default_value_t = false)]
    pub mark_completed: bool,

    /// Skip completed topics
    #[arg(long, default_value_t = true)]
    pub skip_completed: bool,
}

impl CommandExecutor for NextTopicCommand {
    fn execute(&self, context: &mut CommandContext) -> Result<(), DbCheckError> {
        debug!("Moving to next topic");
        let current_topic_url = context
            .get_current_topic()?
            .expect("No current topic found");
        debug!("Current topic: {0}", current_topic_url);
        let current_chapter = context
            .get_current_chapter()?
            .expect("No current chapter found");
        debug!("Current chapter: {0}", current_chapter.id);

        let current_topic = current_chapter
            .topics
            .into_iter()
            .find(|t| t.url == current_topic_url);
        debug!("Current topic found: {0}", current_topic.is_some());

        if let Some(current_topic) = current_topic {
            if let Some(next_topic_url) = current_topic.next_url {
                let mut status = context.get_course_status()?;
                if self.mark_completed {
                    status.completed.push(current_topic_url);
                }

                status.current_topic = Some(next_topic_url.clone());
                status.current_chapter = context
                    .find_chapter_by_topic_url(&next_topic_url)?
                    .map(|c| c.url);
                context.update_course_status(status)?;
                info!("Moved to next topic: {0}", next_topic_url);
            } else {
                error!("No next topic found, maybe you have completed the course?!");
            }
        } else {
            error!("No current topic found in {0} chapter", current_chapter.id);
        }

        Ok(())
    }
}
