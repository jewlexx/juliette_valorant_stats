use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.henrikdev.xyz/valorant/v1/mmr/ap/noimradiorebel/6969";

#[derive(Debug, Serialize, Deserialize)]
pub struct ValorantStats {
    status: i64,
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    currenttier: i64,
    currenttierpatched: String,
    images: Images,
    ranking_in_tier: i64,
    mmr_change_to_last_game: i64,
    elo: i64,
    name: String,
    tag: String,
    old: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    small: String,
    large: String,
    triangle_down: String,
    triangle_up: String,
}

#[tokio::main]
async fn main() {
    let handler = service_fn(handler);
    run(handler).await.unwrap();
}

async fn handler(_: LambdaEvent<serde_json::Value>) -> Result<serde_json::Value, Error> {
    let response = reqwest::get(URL).await?;
    let body: ValorantStats = response.json().await?;

    let return_value = format!(
        "{} - RR: {}",
        body.data.currenttierpatched, body.data.ranking_in_tier
    );

    Ok(serde_json::from_str(&return_value)?)
}
