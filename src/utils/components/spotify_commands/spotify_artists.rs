use super::{spotify_token, Items};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use crate::Error;

// Necesario para la petición a la API de Spotify
use serde::{Deserialize, Serialize};
use spotify_rs::model::artist::Artist;
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct APIResponse {
    artists: Items<super::General>,
}

#[allow(dead_code)]
async fn artist_id(artist: String) -> Result<String, Error> {
    let token = spotify_token().await.unwrap(); // Token necesario para la petición
    let artista_string = artist.replace(" ", "+");

    let url = format!("https://api.spotify.com/v1/search?q={}&type=artist", artista_string); // URL de la petición
    let client: Client = Client::new();
    let res = client // Petición a la API de Spotify
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    // Verifica si la petición fue exitosa
    if res.status() == reqwest::StatusCode::OK {
        // Verifica si el JSON es válido
        match res.json::<APIResponse>().await {
            Ok(json) => return Ok(json.artists.items.first().unwrap().id.clone()),
            Err(e) => Err(Error::from(format!("Error in the API Response: {:?}", e))),
        }
    // En caso de no tener autorización, osea tener un token inválido, se debe refrescar el token
    } else if res.status() == reqwest::StatusCode::UNAUTHORIZED {
        Err(Error::from(format!("Error 401: Unauthorized, need to refresh token")))
    } else {
    // En cualquier otro caso, se retorna un error
        Err(Error::from(format!("Weird error: {:?}", res.status())))
    }
}

pub async fn artist_data(artist: String) -> Result<Artist, Error> {
    let artist_id = artist_id(artist).await.unwrap(); // ID del artista

    dotenv().expect("Expected env");
    let client_id = env::var("SPOTIFY_ID").expect("missing SPOTIFY_ID");
    let client_secret = env::var("SPOTIFY_SECRET").expect("missing SPOTIFY_SECRET");

    let auth_flow = spotify_rs::ClientCredsFlow::new(client_id, client_secret);
    let mut spotify = spotify_rs::ClientCredsClient::authenticate(auth_flow).await?;

    let artist = spotify.artist(&artist_id).get().await?;

    Ok(artist)
}