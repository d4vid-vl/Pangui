pub mod spotify_artists;
pub mod spotify_lps;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use spotify_rs::{ClientCredsClient, ClientCredsFlow};
use std::env;

// TODO: Fixear errores al pedir álbumes, no recibe ciertos parámetros

// Structs generales
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Artist {
    pub name: String,
    pub id: String,
    pub genres: Option<Vec<String>>,
    pub images: Option<Vec<Images>>,
    pub popularity: Option<u32>,
    pub followers: Option<Followers>,
    pub external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    pub name: String,
    pub id: String,
    pub artists: Vec<Artist>,
    pub release_date: String,
    pub album_type: String,
    pub total_tracks: u32,
    pub images: Vec<Images>,
    pub genres: Option<Vec<String>>,
    pub tracks: Option<Vec<Track>>,
    pub label: Option<String>,
    pub copyrights: Option<Vec<Copyrights>>,
    pub external_urls: Option<ExternalUrls>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    pub name: String,
    pub id: String,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub duration_ms: u32,
    pub genres: Option<Vec<String>>,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Items<T> {
    pub items: Vec<T>,
}

// Structs específicos
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Images {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Followers {
    pub href: Option<String>,
    pub total: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Copyrights {
    pub text: String,
    pub type_: Option<String>,
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
