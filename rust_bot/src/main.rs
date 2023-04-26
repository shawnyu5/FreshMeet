use std::env;
use std::{collections::HashSet, sync::Arc};
mod commands;
use crate::commands::command::Command;
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
use tracing::{error, info};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
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

            let content = match command.data.name.as_str() {
                "meetup" => {
                    commands::meetup::Meetup::run(&command, &ctx, &command.data.options).await
                }
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.content(content).components(|c| {
                        commands::meetup::create_components(c)
                        // c.create_action_row(|a| a.create_button(|b| b.label("Click me!")))
                    })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        } else if let Interaction::MessageComponent(component) = &interaction {
            dbg!(&component.data.custom_id);
            commands::meetup::handle_button_click(&component, &ctx).await;
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
        commands.create_application_command(|command| commands::meetup::Meetup::register(command))
    })
    .await;
    dbg!(&commands);

    // notify that the commands have been registered for this guild
    // sender
    // .send(format!("Registered commands for guild {}", guild_id))
    // .unwrap();
}

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    dotenv::dotenv().expect("Failed to load .env file");

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
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
