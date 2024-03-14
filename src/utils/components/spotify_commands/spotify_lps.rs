use super::{spotify_token, Album, Items};
use serde::{Deserialize, Serialize};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use crate::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APIResponse {
    pub albums: Items<Album>,
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
    let token = spotify_token().await.unwrap(); // Token necesario para la petición

    let url = format!("https://api.spotify.com/v1/albums/{}", album_id); // URL de la petición
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
        match res.json::<Album>().await {
            Ok(json) => return Ok(json),
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
