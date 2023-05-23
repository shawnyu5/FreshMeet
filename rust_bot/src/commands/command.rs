use std::collections::HashMap;

use async_trait::async_trait;
use serenity::{
    builder::{CreateApplicationCommand, CreateComponents},
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction,
        message_component::MessageComponentInteraction,
    },
    prelude::Context,
};

#[async_trait]
pub trait SlashCommand: Send + Sync {
    /// runs the slash command
    ///
    /// * `interaction`: the interaction object
    /// * `ctx`: the context
    /// * `options`: command options
    /// * returns a string that will be sent to the user
    async fn run(&mut self, interaction: &ApplicationCommandInteraction, ctx: &Context) -> String;

    /// register a slash command
    ///
    /// * `command`: CreateApplicationCommand object
    /// * returns a mutable reference to the CreateApplicationCommand object
    fn register(self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;

    /// create components for the slash command
    ///
    /// * `c`: create components object
    /// returns a mutable reference to the create components object
    fn create_components<'a>(&self, c: &'a mut CreateComponents) -> &'a mut CreateComponents;

    /// handle the component interaction
    ///
    /// * `interaction_id`: the component interaction id
    /// * `interaction`: component interaction object
    /// * `ctx`: command context
    async fn handle_component_interaction(
        &mut self,
        interaction: &MessageComponentInteraction,
        ctx: &Context,
    );

    // create a hashmap of component names and their component id
    fn all_component_ids(&self) -> HashMap<String, String>;
    // -> HashMap<String, Box<Pin<Box<dyn Future<Output = ()> + Send>>>>;
    // -> HashMap<String, Box<dyn Fn(&MessageComponentInteraction, &Context) -> BoxFuture<'a, ()>>>;
}

// /// return a hashmap of all commands
// /// return: a hashmap of command names to their slash command object
// pub fn all_commands() -> HashMap<String, Box<dyn SlashCommand>> {
// let mut map: HashMap<String, Box<dyn SlashCommand>> = HashMap::new();
// let meetup = meetup::Meetup::default();
// map.insert("meetup".to_string(), Box::new(meetup));
// return map;
// }
