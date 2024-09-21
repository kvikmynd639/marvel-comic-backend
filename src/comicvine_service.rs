use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]  // Add Serialize here
pub struct Comic {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<Image>,
}

#[derive(Deserialize, Serialize)]  // Add Serialize here as well
pub struct Image {
    pub original_url: String,
}


pub async fn fetch_comics() -> Result<Vec<Comic>, reqwest::Error> {
    let api_key = "5c460abcd5cd96ac8e9a6a6577a27f3baca810be";  // Replace with your ComicVine API Key
    let api_url = format!(
        "https://comicvine.gamespot.com/api/issues/?api_key={}&format=json",
        api_key
    );

    let response = reqwest::get(&api_url)
        .await?
        .json::<ApiResponse>()
        .await?;

    Ok(response.results)
}

#[derive(Deserialize)]
struct ApiResponse {
    pub results: Vec<Comic>,
}
