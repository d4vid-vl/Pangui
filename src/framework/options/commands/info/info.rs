use poise::CreateReply;
use serenity::{all::colours::branding, builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter}, model::Timestamp};
use tracing::error;

use crate::{Context, Error};

use super::{
    CARGO_AUTHORS, CARGO_DESCRIPTION, CARGO_NAME, CARGO_RUST_VERSION, CARGO_VERSION, GITHUB_URL, BOT_INVITE_URL,
};

#[poise::command(
    slash_command,
    prefix_command,
    category = "Info",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5
)]
/// Información básica sobre este bot.
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    let http = ctx.http();

    let bot = match http.get_current_user().await {
        Ok(value) => value,
        Err(why) => {
            error!("Couldn't get current user: {why:?}");
            return Err(why.into());
        }
    };
    let bot_avatar_url = bot.avatar_url().unwrap_or(bot.default_avatar_url());

    let constants = [
        CARGO_NAME,         // 0
        CARGO_VERSION,      // 1
        CARGO_AUTHORS,      // 2
        CARGO_DESCRIPTION,  // 3
        GITHUB_URL,         // 4
        CARGO_RUST_VERSION, // 5
        BOT_INVITE_URL,     // 6
    ];

    let author = match constants[2].split(',').next() {
        Some(value) => value,
        None => "No author found",
    };
    let embed_author = CreateEmbedAuthor::new("Información sobre mi").icon_url(&bot_avatar_url);

    let footer = format!("Powered by Rust {}", constants[5]);
    let embed_footer = CreateEmbedFooter::new(footer);

    let info_embed = CreateEmbed::default()        
    .author(embed_author)
    .title(format!("{} v{}", constants[0], constants[1]))
    .description(constants[3])
    .fields(vec![
        ("Autor", author, true),
        ("Serenity", "0.12.0", true),
        ("Poise", "0.6.1", true)
    ])
    .url(format!("{}", constants[6]))
    .footer(embed_footer)
    .colour(branding::BLURPLE)
    .timestamp(Timestamp::now());

    let reply = CreateReply::default().embed(info_embed).ephemeral(false);
    ctx.send(reply).await?;

    Ok(())
}