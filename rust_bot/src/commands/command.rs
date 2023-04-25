use async_trait::async_trait;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};

#[async_trait]
pub trait Command {
    async fn run(options: &[CommandDataOption]) -> String;
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}
