use super::{spotify_token, Artist};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    items: Vec<Artist>,
}

pub async fn artist_data(artist: String) -> Result<Artist, Box<dyn std::error::Error>> {
    let token = spotify_token().await.unwrap(); // Token necesario para la petición

    let url = format!("https://api.spotify.com/v1/search?q={}&type=artist", artist); // URL de la petición
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
            Ok(json) => return Ok(json.items.first().unwrap().clone()),
            Err(e) => Err(Box::<dyn std::error::Error>::from(format!("Error: {:?}", e))),
        }
    // En caso de no tener autorización, osea tener un token inválido, se debe refrescar el token
    } else if res.status() == reqwest::StatusCode::UNAUTHORIZED {
        Err(Box::<dyn std::error::Error>::from(format!("Error 401: Unauthorized, need to refresh token")))
    } else {
    // En cualquier otro caso, se retorna un error
        Err(Box::<dyn std::error::Error>::from(format!("Weird error: {:?}", res.status())))
    }
}
