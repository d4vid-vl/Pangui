use super::{spotify_token, Artist, Items};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APIResponse {
    pub artists: Items<Artist>,
}

#[allow(dead_code)]
pub async fn artist_data(artist: String) -> Result<Artist, Error> {
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
            Ok(json) => return Ok(json.artists.items.first().unwrap().clone()),
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