use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use serenity::all::GatewayIntents;
use tokio::time::Instant;
use tracing::debug;

use crate::{framework::options::commands, Data, Error};

pub mod options;

pub async fn framework_options() -> FrameworkOptions<Data, Error>{
    let start_time = Instant::now();

    let framework_options = FrameworkOptions {
        commands: commands::guild_commands().await,
        event_handler: |ctx, event, framework, _data| {
            Box::pin(options::event_handler::handle(ctx, event, framework))
        },
        prefix_options: PrefixFrameworkOptions {
            prefix: Some(format!("?")),
            ..Default::default()
        },
        ..Default::default()
    };

    let elapsed_time = start_time.elapsed();
    debug!("Initialised framework options in {elapsed_time:.2?}");

    framework_options
}

pub async fn framework() -> Framework<Data, Error> {
    let start_time = Instant::now();

    let framework = Framework::builder()
    .setup(move |ctx, _ready, framework| {
        Box::pin(async move {
            println!("Logged in as {}", _ready.user.name);
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(Data {})
        })
    })
    .options(framework_options().await)
    .build();

    let elapsed_time = start_time.elapsed();
    debug!("Initialised framework in {elapsed_time:.2?}");

    framework
}

pub fn intents() -> GatewayIntents {
    let start_time = Instant::now();

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::AUTO_MODERATION_CONFIGURATION
        | GatewayIntents::AUTO_MODERATION_EXECUTION;

    let elapsed_time = start_time.elapsed();
    debug!("Initialised intents in {elapsed_time:.2?}");

    intents
}