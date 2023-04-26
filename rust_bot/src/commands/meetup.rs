use std::time::Duration;

use super::command::Command;
use async_trait::async_trait;
use chrono::FixedOffset;
use chrono::{DateTime, Utc};
use networking_accumlator::search;
use networking_accumlator::SearchData;
use serenity::builder::{CreateButton, CreateComponents, EditInteractionResponse};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
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

fn to_iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    return format!("{}", dt.format("%+"));
    // formats like "2001-07-08T00:34:60.026490+09:30"
}

pub fn components(c: &mut CreateComponents) -> &mut CreateComponents {
    c.create_action_row(|a| a.create_button(|b| b.label("Click me!").custom_id("click me")))
    // let mut button = CreateButton::default();
    // button.label("Click me!");
    // return button;
}

// pub async fn handle_button_click() {
// let interaction = match m
// .await_component_interaction(&ctx)
// .timeout(Duration::from_secs(60 * 3))
// .await
// {
// Some(x) => x,
// None => {
// m.reply(&ctx, "Timed out").await.unwrap();
// return;
// }
// };
// }

#[async_trait]
impl Command for Meetup {
    async fn run(
        interaction: &ApplicationCommandInteraction,
        ctx: &Context,
        options: &[CommandDataOption],
    ) -> String {
        // interaction
        // .create_interaction_response(&ctx.http, |r| {
        // r.kind(InteractionResponseType::DeferredChannelMessageWithSource)
        // })
        // .await
        // .unwrap();
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
