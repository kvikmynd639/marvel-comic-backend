use reqwest;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use dotenv::dotenv;
use std::env;
use md5::{Md5, Digest};  // Add these imports for MD5 hashing


#[derive(Deserialize, Serialize)]
pub struct Comic {
    pub title: String,
    pub description: Option<String>,
    pub thumbnail: Thumbnail,
}

#[derive(Deserialize, Serialize)]
pub struct Thumbnail {
    pub path: String,
    pub extension: String,
}

#[derive(Deserialize)]
pub struct MarvelResponse {
    pub data: MarvelData,
}

#[derive(Deserialize)]
pub struct MarvelData {
    pub results: Vec<Comic>,
}

pub async fn fetch_marvel_comics() -> Result<Vec<Comic>, reqwest::Error> {
    dotenv().ok();  // Load environment variables from .env

    let public_key = env::var("PUBLIC_KEY").expect("PUBLIC_KEY not found in .env");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env");
    
    let ts = timestamp();  // Generate a timestamp
    let hash = generate_hash(&ts, &private_key, &public_key);  // Generate the hash

    let api_url = format!(
        "https://gateway.marvel.com/v1/public/comics?ts={}&apikey={}&hash={}",
        ts, public_key, hash
    );

    let response = reqwest::get(&api_url)
        .await?
        .json::<MarvelResponse>()
        .await?;

    Ok(response.data.results)
}

// Generate a timestamp
fn timestamp() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs().to_string()
}

// Generate MD5 hash

fn generate_hash(ts: &str, private_key: &str, public_key: &str) -> String {
    let input = format!("{}{}{}", ts, private_key, public_key);

    // Create an MD5 hasher instance
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());

    // Compute the hash and format it as hexadecimal
    let result = hasher.finalize();
    format!("{:x}", result)
}
