use async_trait::async_trait;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOption,
    },
    prelude::Context,
};

#[async_trait]
pub trait Command {
    async fn run(
        interaction: &ApplicationCommandInteraction,
        ctx: &Context,
        options: &[CommandDataOption],
    ) -> String;
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}
