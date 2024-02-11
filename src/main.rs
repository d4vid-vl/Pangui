mod framework;

use framework::{framework, intents};
use poise::serenity_prelude as serenity;
use dotenv;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Expected env");
    let token = std::env::var("PANGUI_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = intents();
    let framework = framework().await;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}