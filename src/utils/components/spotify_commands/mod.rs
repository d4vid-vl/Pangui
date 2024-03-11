pub mod spotify_artists;

use dotenv::dotenv;
use std::env;
use spotify_rs::{ClientCredsClient, ClientCredsFlow};

async fn spotify_token() -> Result<String, Box<dyn std::error::Error>>{
    dotenv().expect("Expected env");
    let client_id = env::var("SPOTIFY_ID").expect("missing SPOTIFY_ID");
    let client_secret = env::var("SPOTIFY_SECRET").expect("missing SPOTIFY_SECRET");

    let auth_flow = ClientCredsFlow::new(client_id, client_secret);
    let spotify = ClientCredsClient::authenticate(auth_flow).await?;
    let spotify_token = spotify.access_token();

    Ok(spotify_token.to_string())
}