pub mod spotify_artists;
pub mod spotify_lps;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use spotify_rs::{ClientCredsClient, ClientCredsFlow};
use std::env;

// Structs generales
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct General {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Items<T> {
    pub items: Vec<T>,
}

async fn spotify_token() -> Result<String, Box<dyn std::error::Error>> {
    dotenv().expect("Expected env");
    let client_id = env::var("SPOTIFY_ID").expect("missing SPOTIFY_ID");
    let client_secret = env::var("SPOTIFY_SECRET").expect("missing SPOTIFY_SECRET");

    let auth_flow = ClientCredsFlow::new(client_id, client_secret);
    let spotify = ClientCredsClient::authenticate(auth_flow).await?;
    let spotify_token = spotify.access_token();

    Ok(spotify_token.to_string())
}
