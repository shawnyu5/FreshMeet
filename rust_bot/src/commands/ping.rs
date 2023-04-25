use super::command::Command;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

/// the /ping command
pub struct Ping;

impl Command for Ping {
    fn run(options: &[CommandDataOption]) -> String {
        "Hey, I'm alive!".to_string()
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("ping").description("A ping command")
    }
}
