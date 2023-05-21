use dotenv::dotenv;
use std::env;
use serenity::framework::{StandardFramework, standard::macros::hook};
use tracing::{info};
use serenity::prelude::*;
use serenity::model::channel::Message;

use super::general_command_loader::GENERAL_GROUP;
use super::slash_command_loader::Handler;

pub async fn setup() {
    dotenv().ok();
    let prefix = env::var("DISCORD_PREFIX").unwrap_or("!".to_string());

    let framework = StandardFramework::new()
    .configure(|c| c.prefix(prefix)) // set the bot's prefix to "!"
    .before(before)
    .unrecognised_command(unknown_command)
    .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[hook]
#[instrument]
pub async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!("Got command '{}' by user '{}'", command_name, msg.author.name);

    true
}

#[hook]
pub async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    info!("Could not find command named '{}'", unknown_command_name);
}
