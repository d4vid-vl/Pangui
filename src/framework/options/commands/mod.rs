mod info;

use poise::Command;
use crate::{Data, Error};

pub async fn guild_commands() -> Vec<Command<Data, Error>>{
    vec![
        info::info::info()
    ]
}