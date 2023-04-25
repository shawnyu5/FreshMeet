use super::command::Command;
use async_trait::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

/// the /ping command
pub struct Ping;

#[async_trait]
impl Command for Ping {
    async fn run(_options: &[CommandDataOption]) -> String {
        "Hey, I'm alive!".to_string()
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("ping").description("A ping command")
    }
}
