use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};

pub trait Command {
    fn run(options: &[CommandDataOption]) -> String;
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}
