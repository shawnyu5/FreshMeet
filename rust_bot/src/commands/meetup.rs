use async_trait::async_trait;
use chrono::FixedOffset;
use chrono::{DateTime, Utc};
use networking_accumlator::SearchData;
use networking_accumlator::{search, Response};
use serenity::builder::{CreateComponents, CreateInteractionResponse};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::{Context, TypeMapKey};
use serenity::utils::MessageBuilder;
use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::utils::utils;

use super::command::SlashCommand;

/// the /meetup command
#[derive(Clone)]
pub struct Meetup {
    pub search_query: String,
    pub page_number: i32,
}

impl Default for Meetup {
    fn default() -> Self {
        Self {
            page_number: 1,
            search_query: "".to_string(),
        }
    }
}

#[derive(EnumIter)]
pub enum ComponentId {
    Next,
}

impl Hash for ComponentId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ComponentId::Next => {
                "Next".hash(state);
            }
        }
    }
}

impl Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentId::Next => write!(f, "next"),
        }
    }
}

impl TypeMapKey for Meetup {
    type Value = Meetup;
}
#[async_trait]
impl SlashCommand for Meetup {
    async fn run(
        &mut self,
        interaction: &ApplicationCommandInteraction,
        ctx: &Context,
    ) -> Vec<String> {
        self.search_query = interaction
            .data
            .options
            .get(0)
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .to_string();

        // today's date
        let today = utils::to_iso8601(&std::time::SystemTime::now());

        let search_result = search(SearchData {
            query: self.search_query.as_str(),
            page: self.page_number,
            per_page: 3,
            start_date: today.into(),
        })
        .await;

        let response: String;
        if let Ok(events) = search_result {
            response = format_search_result(events);
        } else {
            response = MessageBuilder::new().push("failed...").build();
        }

        return vec![response];
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
    fn create_components<'a>(&self, c: &'a mut CreateComponents) -> &'a mut CreateComponents {
        let hashed_component_ids = self.all_component_ids();
        c.create_action_row(|a| {
            a.create_button(|b| {
                b.label("next").custom_id(
                    hashed_component_ids
                        .get(&ComponentId::Next.to_string())
                        .unwrap(),
                )
            })
        })
    }

    /// handle next page pagination
    async fn handle_component_interaction(
        &mut self,
        interaction: &MessageComponentInteraction,
        ctx: &Context,
    ) {
        interaction.defer(&ctx.http).await.unwrap();

        let component_ids = self.all_component_ids();
        let next_id = component_ids.get(&ComponentId::Next.to_string()).unwrap();
        let today = utils::to_iso8601(&std::time::SystemTime::now());

        match &interaction.data.custom_id {
            value if value == next_id => {
                self.page_number += 1;
                let search_result = search(SearchData {
                    query: self.search_query.as_str(),
                    page: self.page_number,
                    per_page: 3,
                    start_date: today.into(),
                })
                .await
                .unwrap();

                let interaction_response = interaction.get_interaction_response(&ctx.http).await;
                dbg!(&interaction_response);

                let reply = format_search_result(search_result);
                interaction
                    .edit_original_interaction_response(&ctx.http, |m| {
                        m.content(reply).components(|c| self.create_components(c))
                    })
                    .await
                    .unwrap();
            }
            _ => {
                println!("no match");
            }
        }
    }

    /// hashmap of component name to component custom id
    fn all_component_ids(&self) -> HashMap<String, String> {
        // TODO: Could this be auto generated using a macro?
        let mut component_ids = HashMap::new();
        // let mut component_ids = Vec::<String>::new();
        let mut hasher = DefaultHasher::new();

        for component_id in ComponentId::iter() {
            component_id.hash(&mut hasher);
            component_ids.insert(component_id.to_string(), hasher.finish().to_string());
        }
        return component_ids;
    }
}

/// format the search result into a string to be sent to discord
///
/// * `events`: events returned from meetup.com
/// return: formatted string to be sent to discord
pub fn format_search_result(events: Response) -> String {
    let mut builder = MessageBuilder::new();
    events.into_iter().for_each(|mut e| {
        builder.push_bold("title: ").push_line(&e.title);
        // truncate description if it is too long
        if &e.description.len() > &250 {
            e.description = e.description[0..249].to_string();
            e.description.push_str("...");
        }
        // delete bold marks
        e.description = e.description.replace("**", "");

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

    return builder.build();
}
