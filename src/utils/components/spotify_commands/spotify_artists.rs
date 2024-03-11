use super::spotify_token;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;

pub async fn artist_data(artist: String) {
    let token = spotify_token().await.unwrap();

    let url = format!("https://api.spotify.com/v1/search?q={}&type=artist", artist);
    let client: Client = Client::new();
    let res = client
    .get(url)
    .header(AUTHORIZATION, format!("Bearer {}", token))
    .header(CONTENT_TYPE, "application/json")
    .header(ACCEPT, "application/json")
    .send()
    .await
    .unwrap();

    match res.status() {
        reqwest::StatusCode::OK => {

            // TODO: Parsear el JSON y devolver los datos del artista necesarios en un Struct
            
            match res.json().await {
                Ok(json) => {
                    println!("{:?}", json);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Error 401: Unauthorized, need to refresh token");
        }
        other => {
            panic!("Weird error: {:?}", other);
        }
    };
}