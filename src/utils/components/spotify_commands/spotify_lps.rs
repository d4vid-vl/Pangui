use super::{spotify_token, Items};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use crate::Error;

// Necesario para la petición a la API de Spotify
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use spotify_rs::model::album::Album;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APIResponse {
    pub albums: Items<super::General>,
}

#[allow(dead_code)]
pub async fn album_id(album: String) -> Result<String, Error> {
    let token = spotify_token().await.unwrap(); // Token necesario para la petición
    let album_string = album.replace(" ", "+");

    let url = format!("https://api.spotify.com/v1/search?q={}&type=album", album_string); // URL de la petición
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
            Ok(json) => return Ok(json.albums.items.first().unwrap().id.clone()),
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

pub async fn album_data(album: String) -> Result<Album, Error> {

    let album_id = album_id(album).await.unwrap(); // ID del álbum
    dotenv().expect("Expected env");
    let client_id = env::var("SPOTIFY_ID").expect("missing SPOTIFY_ID");
    let client_secret = env::var("SPOTIFY_SECRET").expect("missing SPOTIFY_SECRET");

    let auth_flow = spotify_rs::ClientCredsFlow::new(client_id, client_secret);
    let mut spotify = spotify_rs::ClientCredsClient::authenticate(auth_flow).await?;

    let album = spotify.album(&album_id).get().await?;
    Ok(album)
}
