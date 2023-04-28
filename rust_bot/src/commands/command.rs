use std::collections::HashMap;

use async_trait::async_trait;
use serenity::{
    builder::{CreateApplicationCommand, CreateComponents},
    model::prelude::interaction::{
        application_command::{ApplicationCommandInteraction, CommandDataOption},
        message_component::MessageComponentInteraction,
    },
    prelude::Context,
};

pub type AsyncFunc = fn(
    interaction: &MessageComponentInteraction,
    ctx: &Context,
) -> dyn std::future::Future<Output = ()>;

#[async_trait]
pub trait SlashCommand: Send + Sync {
    /// runs the slash command
    ///
    /// * `interaction`: the interaction object
    /// * `ctx`: the context
    /// * `options`: command options
    /// * returns a string that will be sent to the user
    async fn run(
        &self,
        interaction: &ApplicationCommandInteraction,
        ctx: &Context,
        options: &[CommandDataOption],
    ) -> String;

    /// register a slash command
    ///
    /// * `command`: CreateApplicationCommand object
    /// * returns a mutable reference to the CreateApplicationCommand object
    fn register(self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;

    /// create components for the slash command
    ///
    /// * `c`: create components object
    /// returns a mutable reference to the create components object
    fn create_components(self, c: &mut CreateComponents) -> &mut CreateComponents;

    /// handle the component interaction
    ///
    /// * `interaction`: component interaction object
    /// * `ctx`: command context
    async fn handle_component_interaction(
        self,
        interaction: &MessageComponentInteraction,
        ctx: &Context,
    );
    /// return a map of component id to component handler function
    ///
    /// * `interaction`: the message component interaction
    /// * `ctx`: command context
    /// returns a map of component id to component handler function
    fn component_handlers<'a>(
        &self,
        interaction: &MessageComponentInteraction,
        ctx: &Context,
    ) -> HashMap<String, Box<dyn std::future::Future<Output = ()>>>;
}
