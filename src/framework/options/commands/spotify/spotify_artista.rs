use crate::{Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};
use ::serenity::all::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Timestamp};
use crate::utils::components::spotify_commands::spotify_artists::*;

#[poise::command(
    prefix_command,
    slash_command,
    category = "Spotify",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5
)]
/// Muestra información de un artista de Spotify
pub async fn spotify_artista(
    ctx: Context<'_>,
    #[description = "Nombre del artista"] artista: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let artist = artist_data(artista.clone()).await?; // Los datos del artista en un struct

    let username = &ctx.author().name;
    let userurl = ctx.author().avatar_url().expect("none").to_string();

    let embed = { CreateEmbed::new() 
    .author(CreateEmbedAuthor::new("Información del artista").icon_url("https://www.freepnglogos.com/uploads/spotify-logo-png/spotify-icon-marilyn-scott-0.png"))
    .title(artist.name)
    .url(artist.external_urls.spotify)
    .thumbnail(artist.images.expect("None :3").first().unwrap().url.clone()) // La primera imágen del artista
    .color(serenity::Color::from_rgb(30, 215, 96))
    .timestamp(Timestamp::now())
    .footer(CreateEmbedFooter::new(format!("Solicitado por {}", &username)).icon_url(userurl))
    .fields([
        ("Seguidores", artist.followers.expect("None :3").total.to_string(), true),
        ("Popularidad", format!("{}%", artist.popularity.expect("None :3").to_string()), true),
        ("Géneros", artist.genres.expect("None :3").join(", "), false),
    ])
    };

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}