mod config;
mod slash_commands;
mod general_commands;

use config::setup::setup;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    setup().await;
}


