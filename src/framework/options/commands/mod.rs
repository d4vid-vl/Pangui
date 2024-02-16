mod info;
mod horario;

use poise::Command;
use crate::{Data, Error};

pub async fn guild_commands() -> Vec<Command<Data, Error>>{
    vec![
        info::info::info(),
        horario::revisar_nombre::revisar_nombre(),
        horario::revisar_sigla::revisar_sigla(),
        horario::revisar_nrc::revisar_nrc(),
        horario::revisar_profesor::revisar_profesor()
    ]
}