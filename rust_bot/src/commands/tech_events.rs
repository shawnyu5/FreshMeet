use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::Display,
    hash::{Hash, Hasher},
};

use async_trait::async_trait;
use networking_accumlator::{search, Response, SearchData};
use serenity::model::prelude::component::ButtonStyle;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::utils::utils;

use super::{
    command::SlashCommand,
    meetup::{self, Meetup},
};

pub struct TechEvents {
    tech_meetups: Meetup,
    programming_meetups: Meetup,
    coding_meetups: Meetup,
}

#[derive(EnumIter, Hash, Display)]
enum ComponentId {
    Next,
}

impl Default for TechEvents {
    fn default() -> Self {
        Self {
            tech_meetups: Meetup {
                search_query: "tech".to_string(),
                page_number: 1,
            },
            programming_meetups: Meetup {
                search_query: "programming".to_string(),
                page_number: 1,
            },
            coding_meetups: Meetup {
                search_query: "coding".to_string(),
                page_number: 1,
            },
        }
    }
}

#[async_trait]
impl SlashCommand for TechEvents {
    async fn run(
        &mut self,
        interaction: & serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction,
        ctx: &serenity::prelude::Context,
    ) -> Vec<String> {
        let today = utils::to_iso8601(&std::time::SystemTime::now());
        let mut reply: Vec<String> = vec![];
        let mut search_result = self.get_search_results().await;

        search_result.dedup();
        search_result.iter().for_each(|r| {
            reply.push(meetup::format_search_result(r.clone()));
        });

        return reply;
    }

    fn register(
        self,
        command: &mut serenity::builder::CreateApplicationCommand,
    ) -> &mut serenity::builder::CreateApplicationCommand {
        command
            .name("tech-events")
            .description("fetch a list of tech events from different sources")
    }

    fn create_components<'a>(
        &self,
        c: &'a mut serenity::builder::CreateComponents,
    ) -> &'a mut serenity::builder::CreateComponents {
        let hashed_component_ids = self.all_component_ids();
        c.create_action_row(|a| {
            a.create_button(|b| {
                b.label("next").style(ButtonStyle::Primary).custom_id(
                    hashed_component_ids
                        .get(&ComponentId::Next.to_string())
                        .unwrap(),
                )
            })
        })
    }

    async fn handle_component_interaction(
        &mut self,
        interaction: & serenity::model::prelude::interaction::message_component::MessageComponentInteraction,
        ctx: &serenity::prelude::Context,
    ) {
        let component_ids = self.all_component_ids();
        let next_id = component_ids.get(&ComponentId::Next.to_string()).unwrap();
        match &interaction.data.custom_id {
            value if value == next_id => {
                self.tech_meetups.page_number += 1;
                self.programming_meetups.page_number += 1;
                self.coding_meetups.page_number += 1;
                let mut reply = Vec::<String>::new();

                let mut search_result = self.get_search_results().await;
                search_result.dedup();
                search_result.iter().for_each(|r| {
                    reply.push(meetup::format_search_result(r.clone()));
                });

                for s in search_result {
                    interaction
                        .channel_id
                        .send_message(&ctx.http, |m| {
                            m.content(meetup::format_search_result(s.clone()))
                        })
                        .await
                        .unwrap();
                }
                interaction
                    .channel_id
                    .send_message(&ctx.http, |m| m.components(|c| self.create_components(c)))
                    .await
                    .unwrap();
            }
            _ => {
                println!("no match");
            }
        }
    }

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

impl TechEvents {
    async fn get_search_results(&self) -> Vec<Response> {
        let today = utils::to_iso8601(&std::time::SystemTime::now());
        let mut search_result: Vec<Response> = Vec::new();

        search_result.push(
            search(SearchData {
                query: self.tech_meetups.search_query.as_str(),
                page: self.tech_meetups.page_number,
                per_page: 3,
                start_date: Some(today.clone()),
            })
            .await
            .unwrap(),
        );

        search_result.push(
            search(SearchData {
                query: self.programming_meetups.search_query.as_str(),
                page: self.programming_meetups.page_number,
                per_page: 3,
                start_date: Some(today.clone()),
            })
            .await
            .unwrap(),
        );

        search_result.push(
            search(SearchData {
                query: self.coding_meetups.search_query.as_str(),
                page: self.coding_meetups.page_number,
                per_page: 3,
                start_date: Some(today),
            })
            .await
            .unwrap(),
        );
        return search_result;
    }
}
