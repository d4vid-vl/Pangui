use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(
    prefix_command,
    slash_command,
    category = "Spotify",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5
)]
pub async fn artista(
    ctx: Context<'_>,
    #[description = "Nombre del artista"] artista: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    Ok(())
}