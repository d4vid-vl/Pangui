mod ready;

use poise::FrameworkContext;

use crate::{serenity::FullEvent, Data, Error};

pub async fn handle(
    ctx: &crate::serenity::Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            ready::handle(ctx, data_about_bot).await;
        }
        _ => {}
    }

    Ok(())
}