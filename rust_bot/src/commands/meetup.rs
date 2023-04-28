use std::collections::HashMap;
use std::fmt::Display;

use super::command::SlashCommand;
use async_trait::async_trait;
use chrono::FixedOffset;
use chrono::{DateTime, Utc};
use networking_accumlator::search;
use networking_accumlator::SearchData;
use serenity::builder::CreateComponents;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::CommandDataOption,
    },
};

/// the /meetup command
pub struct Meetup;

enum ComponentId {
    ClickMe,
    Next,
    Previous,
}

impl Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentId::ClickMe => write!(f, "click me"),
            ComponentId::Next => write!(f, "next"),
            ComponentId::Previous => write!(f, "previous"),
        }
    }
}

fn to_iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    return format!("{}", dt.format("%+"));
    // formats like "2001-07-08T00:34:60.026490+09:30"
}

pub async fn handle_click_me(interaction: &MessageComponentInteraction, ctx: &Context) {
    interaction
        .create_interaction_response(&ctx.http, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| d.content("You clicked the button!"))
        })
        .await
        .unwrap();
}

#[async_trait]
impl SlashCommand for Meetup {
    async fn run(
        &self,
        _interaction: &ApplicationCommandInteraction,
        _ctx: &Context,
        options: &[CommandDataOption],
    ) -> String {
        let query = options.get(0).unwrap().value.as_ref().unwrap().to_string();

        // today's date
        let today = to_iso8601(&std::time::SystemTime::now());

        let result = search(SearchData {
            query: query.as_str(),
            page: 1,
            per_page: 3,
            start_date: today.into(),
        })
        .await;

        let response: String;
        if let Ok(events) = result {
            let mut builder = MessageBuilder::new();
            events.into_iter().for_each(|mut e| {
                builder.push_bold("title: ").push_line(&e.title);
                if &e.description.len() > &250 {
                    e.description = e.description[0..249].to_string();
                    e.description.push_str("...");
                }
                builder.push_bold("description: ").push_line(&e.description);
                let datetime = DateTime::parse_from_str(&e.dateTime, "%Y-%m-%dT%H:%M%z")
                    .unwrap()
                    // TODO: dont use this deprecated function
                    .with_timezone(&FixedOffset::west(4 * 60 * 60)); // convert to EST

                let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                builder
                    .push_bold("date: ")
                    .push_line(formatted_time)
                    .push_bold("link: ")
                    .push_line(String::from("<".to_owned() + &e.eventUrl + ">"))
                    .push_line(" ");
            });

            response = builder.build();
        } else {
            response = MessageBuilder::new().push("failed...").build();
        }

        return response;
    }

    fn register(self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
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

    /// create forward and back buttons
    fn create_components(self, c: &mut CreateComponents) -> &mut CreateComponents {
        c.create_action_row(|a| {
            a.create_button(|b| b.label("Click me!").custom_id(ComponentId::ClickMe as u32))
        })
    }

    /// handle previous and next page pagination
    async fn handle_component_interaction(
        self,
        interaction: &MessageComponentInteraction,
        ctx: &Context,
    ) {
        interaction
            .create_interaction_response(&ctx.http, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|d| d.content("You clicked the button!"))
            })
            .await
            .unwrap();
    }

    fn component_handlers<'a>(
        &self,
        interaction: &MessageComponentInteraction,
        ctx: &Context,
    ) -> HashMap<String, Box<dyn std::future::Future<Output = ()>>> {
        let mut map = HashMap::new();
        map.insert(
            ComponentId::ClickMe.to_string(),
            Box::new(handle_click_me(interaction, ctx)),
        );
        return map;
    }
}
