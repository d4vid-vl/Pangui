use std::sync::Arc;

use tokio::time::Duration;
use tracing::info;

use crate::serenity::{ActivityData, Ready};

pub async fn handle(ctx: &crate::serenity::Context, ready: &Ready) {
    let user_name = &ready.user.name;

    info!("Connected as @{user_name}");

    let ctx = Arc::new(ctx.clone());
    tokio::spawn(async move {
        loop {
            set_activity(&ctx).await;

            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
}

async fn set_activity(ctx: &crate::serenity::Context) {
    ctx.set_activity(Some(ActivityData::watching("pautas de las pruebas de cálculo")));
}