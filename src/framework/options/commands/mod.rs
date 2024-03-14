mod horario;
mod info;
mod spotify;

use crate::{Data, Error};
use poise::Command;

pub async fn guild_commands() -> Vec<Command<Data, Error>> {
    vec![
        info::info::info(),
        horario::revisar_nombre::revisar_nombre(),
        horario::revisar_sigla::revisar_sigla(),
        horario::revisar_nrc::revisar_nrc(),
        horario::revisar_profesor::revisar_profesor(),
        spotify::spotify_artista::spotify_artista(),
        spotify::spotify_lp::spotify_lp(),
    ]
}
