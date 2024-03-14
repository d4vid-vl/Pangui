use crate::{Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};
use ::serenity::all::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Timestamp};
use crate::utils::components::spotify_commands::spotify_lps::*;

#[poise::command(
    prefix_command,
    slash_command,
    category = "Spotify",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5
)]
/// Muestra información de un álbum de Spotify
pub async fn spotify_lp(
    ctx: Context<'_>,
    #[description = "Nombre del álbum"] album_name: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let album = album_data(album_name).await?; // Los datos del álbum en un struct

    let username = &ctx.author().name;
    let userurl = ctx.author().avatar_url().expect("none").to_string();
    let tracks = album.tracks.expect("None :3").iter().map(|s| s.name.clone()).collect::<Vec<_>>().join(", ");

    let embed = { CreateEmbed::new() 
    .author(CreateEmbedAuthor::new("Información del álbum").icon_url("https://www.freepnglogos.com/uploads/spotify-logo-png/spotify-icon-marilyn-scott-0.png"))
    .title(album.name)
    .url(album.external_urls.expect("None :3").spotify)
    .thumbnail(album.images.first().unwrap().url.clone()) // La primera imágen del álbum
    .color(serenity::Color::from_rgb(30, 215, 96))
    .timestamp(Timestamp::now())
    .footer(CreateEmbedFooter::new(format!("Solicitado por {}", &username)).icon_url(userurl))
    .fields([
        ("Artista", album.artists.first().unwrap().name.clone(), true),
        ("Fecha de lanzamiento", album.release_date, true),
        ("Número de canciones", album.total_tracks.to_string(), true),
        ("Tipo de álbum", album.album_type.clone(), true),
        ("Sello discográfico", album.label.expect("None :3"), true),
        ("Tracks", tracks, false),
        ("Copyright", album.copyrights.expect("None :3").first().unwrap().text.clone(), false),
    ])
    };

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}