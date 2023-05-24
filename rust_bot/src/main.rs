mod commands;
mod utils;

use crate::commands::command::SlashCommand;
use lazy_static::lazy_static;
use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::application::interaction::Interaction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;
use std::collections::HashMap;
use std::{collections::HashSet, sync::Arc};
use std::{dbg, env};
use tracing::{error, info};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

lazy_static! {
    /// hashmap of all commands name to their command struct
    static ref COMMANDS: Mutex<HashMap<&'static str, Box<dyn SlashCommand>>> = {
        let mut map = HashMap::<&'static str, Box<dyn SlashCommand>>::new();
        let meetup = commands::meetup::Meetup::default();
        let tech_events = commands::tech_events::TechEvents::default();
        map.insert("meetup", Box::new(meetup));
        map.insert("tech-events", Box::new(tech_events));
        return Mutex::new(map);
    };

}

/// the event handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);
            command
                .create_interaction_response(&ctx.http, |r| {
                    r.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                })
                .await
                .unwrap();

            let mut commands = COMMANDS.lock().await;
            // let mut commands = commands::command::all_commands();
            let cmd = commands.get_mut(command.data.name.as_str()).unwrap();
            let content_vec = cmd.run(&command, &ctx).await;

            // if there are more content, send as follow up message
            for (index, content) in content_vec.iter().enumerate() {
                if index == 0 {
                    if let Err(why) = command
                        .edit_original_interaction_response(&ctx.http, |response| {
                            response.content(content_vec.get(0).unwrap())
                            // .components(|c| cmd.create_components(c))
                        })
                        .await
                    {
                        println!("Failed to respond to slash command: {}", why);
                    }
                } else if let Err(e) = command
                    .channel_id
                    .send_message(&ctx.http, |c| c.content(content))
                    .await
                {
                    println!("Failed to send follow up message: {}", e)
                }
            }
            command
                .channel_id
                .send_message(&ctx.http, |m| m.components(|c| cmd.create_components(c)))
                .await
                .unwrap();
        } else if let Interaction::MessageComponent(component) = &interaction {
            let mut commands = COMMANDS.lock().await;
            // let mut commands = commands::command::all_commands();
            for (_, cmd_def) in commands.iter_mut() {
                let component_ids = cmd_def.all_component_ids();
                // check if the current slash command handles this component interaction
                if component_ids
                    .values()
                    .any(|value| value == &component.data.custom_id)
                {
                    cmd_def.handle_component_interaction(&component, &ctx).await;
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        // TODO: how to get logging to work
        info!("Connected as {}", ready.user.name);

        for guild_id in ctx.cache.guilds() {
            register_commands(&guild_id, &ctx).await;
        }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

/// register slash commands for a guild
///
/// * `guild_id`: the guild to register
/// * `ctx`: the context
async fn register_commands(guild_id: &GuildId, ctx: &Context) {
    // check if the guild is cached
    let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        commands
            .create_application_command(|command| {
                commands::meetup::Meetup::default().register(command)
            })
            .create_application_command(|command| {
                commands::tech_events::TechEvents::default().register(command)
            })
    })
    .await;
    dbg!(&commands);

    // notify that the commands have been registered for this guild
    // sender
    // .send(format!("Registered commands for guild {}", guild_id))
    // .unwrap();
}

async fn delete_commands(guild_id: &GuildId, http: Http) {
    // TODO: how to delete commands
    println!("Deleting commands for guild {}", guild_id)
    // guild_id.delete_application_command(http);
    // http.delete_guild_application_command(guild_id)
}

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    dotenv::dotenv().expect("Failed to load .env file");
    // commands::meetup::test();

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to `debug`.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new().configure(|c| c.owners(owners).prefix("~"));

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
        dbg!("Shutting down");
        // let ctx = &client.cache_and_http.cache;
        // for guild_id in ctx.guilds() {
        // register_commands(&guild_id, &ctx).await;
        // }
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
