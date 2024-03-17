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
    let tracks = album.tracks.items.iter().map(|x| x.name.clone()).collect::<Vec<String>>().join(", ");

    let embed = { CreateEmbed::new() 
    .author(CreateEmbedAuthor::new("Información del álbum").icon_url("https://www.freepnglogos.com/uploads/spotify-logo-png/spotify-icon-marilyn-scott-0.png"))
    .title(album.name)
    .url(album.external_urls.spotify)
    .thumbnail(album.images.first().unwrap().url.clone()) // La primera imágen del álbum
    .color(serenity::Color::from_rgb(30, 215, 96))
    .timestamp(Timestamp::now())
    .footer(CreateEmbedFooter::new(format!("Solicitado por {}", &username)).icon_url(userurl))
    .fields([
        ("Artista", album.artists.first().unwrap().name.clone(), true),
        ("Fecha de lanzamiento", album.release_date, true),
        ("Número de canciones", album.total_tracks.to_string(), true),
        ("Tipo de álbum", format!("{:?}", album.album_type), true),
        ("Sello discográfico", album.label, true),
        ("Tracks", tracks, false),
        ("Copyright", album.copyrights.first().unwrap().text.clone(), false),
    ])
    };

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}