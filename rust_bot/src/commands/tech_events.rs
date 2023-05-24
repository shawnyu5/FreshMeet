use std::collections::HashMap;

use async_trait::async_trait;
use networking_accumlator::{search, Response, SearchData};

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

        search_result.dedup();
        return vec!["hi".to_string(), "world".to_string()];
        // search_result.iter().for_each(|r| {
        // let reply = meetup::format_search_result(*r);
        // })
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
        c
    }

    async fn handle_component_interaction(
        &mut self,
        interaction: & serenity::model::prelude::interaction::message_component::MessageComponentInteraction,
        ctx: &serenity::prelude::Context,
    ) {
    }

    fn all_component_ids(&self) -> HashMap<String, String> {
        return HashMap::new();
    }
}
