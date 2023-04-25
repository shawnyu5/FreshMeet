use super::command::Command;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::CommandDataOption,
    },
};

/// the /meetup command
pub struct Meetup;

impl Command for Meetup {
    fn run(options: &[CommandDataOption]) -> String {
        // TODO: send query to meetup.com
        options
            .get(0)
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("meetup")
            .description("Search meetup.com for events")
            .create_option(|option| {
                option
                    .name("query")
                    .description("the query to search for in meetup.com")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    }
}
