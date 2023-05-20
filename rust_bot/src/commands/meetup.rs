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
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Display;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// the /meetup command
pub struct Meetup;

#[derive(EnumIter)]
pub enum ComponentId {
    ClickMe,
    Next,
    Previous,
}

impl Hash for ComponentId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ComponentId::ClickMe => {
                "ClickMe".hash(state);
            }
            ComponentId::Next => {
                "Next".hash(state);
            }
            ComponentId::Previous => {
                "Previous".hash(state);
            }
        }
    }
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
        let hashed_component_ids = self.all_component_ids();
        let mut hasher = DefaultHasher::new();
        ComponentId::ClickMe.hash(&mut hasher);
        c.create_action_row(|a| {
            a.create_button(|b| {
                b.label("Click me!").custom_id(
                    hashed_component_ids
                        .get(&ComponentId::ClickMe.to_string())
                        .unwrap(),
                )
            })
            .create_button(|b| {
                b.label("Next").custom_id(
                    hashed_component_ids
                        .get(&ComponentId::Next.to_string())
                        .unwrap(),
                )
            })
        })
    }

    /// handle previous and next page pagination
    async fn handle_component_interaction(
        &self,
        interaction: &MessageComponentInteraction,
        ctx: &Context,
    ) {
        let component_ids = self.all_component_ids();
        let click_me_id = component_ids
            .get(&ComponentId::ClickMe.to_string())
            .unwrap();
        let next_id = component_ids.get(&ComponentId::Next.to_string()).unwrap();
        let previous_id = component_ids
            .get(&ComponentId::Previous.to_string())
            .unwrap();

        dbg!(&interaction.data.custom_id);
        dbg!(next_id);
        match &interaction.data.custom_id {
            value if value == click_me_id => {
                interaction
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|d| d.content("You clicked the button!!!!"))
                    })
                    .await
                    .unwrap();
            }
            value if value == next_id => {
                interaction
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|d| d.content("Next button"))
                    })
                    .await
                    .unwrap();
            }
            value if value == previous_id => {
                interaction
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|d| d.content("previous button"))
                    })
                    .await
                    .unwrap();
            }
            _ => {
                println!("no match");
            }
        }
    }

    fn component_handlers<'a>(
        &self,
        // interaction: &'static MessageComponentInteraction,
        // ctx: &Context,
    ) -> HashMap<
        String,
        Box<
            dyn Fn(
                &'a MessageComponentInteraction,
                &'a Context,
            ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>,
        >,
    > {
        let mut map: HashMap<
            String,
            Box<
                dyn Fn(
                    &'a MessageComponentInteraction,
                    &'a Context,
                ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>,
            >,
        > = HashMap::new();

        map.insert(
            ComponentId::ClickMe.to_string(),
            Box::new(|interaction, ctx| Box::pin(handle_click_me(interaction, ctx))),
        );
        // map.insert(
        // ComponentId::ClickMe.to_string(),
        // Box::new(|interaction, ctx| Box::pin(Meetup::handle_click_me(interaction, ctx))),
        // );
        return map;
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

// pub fn test() {
// let mut map = HashMap::new();
// map.insert(
// ComponentId::ClickMe.to_string(),
// Box::new(|interaction, ctx| (handle_click_me(interaction, ctx)).boxed()),
// );
// // map.insert(
// // ComponentId::ClickMe.to_string(),
// // Box::new(|interaction, ctx| Box::pin(Meetup::handle_click_me(interaction, ctx))),
// // );
// // return map;
// }
