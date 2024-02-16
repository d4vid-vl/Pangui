mod info;
mod horario;

use poise::Command;
use crate::{Data, Error};

pub async fn guild_commands() -> Vec<Command<Data, Error>>{
    vec![
        info::info::info(),
        horario::revisar_curso::revisar_curso(),
        horario::revisar_sigla::revisar_sigla()
    ]
}