use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use strum::{Display, EnumString};

const RIOT_EU_BASE_URL: &str = "https://europe.api.riotgames.com";

#[derive(clap::ValueEnum, Clone, Debug, Display, EnumString, Serialize, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Region {
    #[serde(alias = "na")]
    NA,
    #[serde(alias = "euw")]
    EUW,
    #[serde(alias = "eune")]
    EUNE,
    #[serde(alias = "kr")]
    KR,
    #[serde(alias = "jp")]
    JP,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub puuid: String,
    #[serde(rename = "gameName")]
    pub username: String,
    #[serde(rename = "tagLine")]
    pub tagline: String,
}

fn build_client() -> reqwest::Client {
    let config = Config::get();

    reqwest::ClientBuilder::new()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "X-Riot-Token",
                config.riot.token.parse().unwrap(),
            );
            headers
        })
        .build()
        .unwrap()
}

pub async fn get_player_info(name: &str, tagline: &str) -> Result<PlayerInfo> {
    let url = format!(
        "{}/riot/account/v1/accounts/by-riot-id/{}/{}",
        RIOT_EU_BASE_URL, name, tagline
    );

    let client = build_client();
    let resp = client.get(&url).send().await?;

    if !resp.status().is_success() {
        return Err(color_eyre::eyre::eyre!(
            "Failed to fetch player info from riot api: {}",
            resp.status()
        ));
    }

    let player_info: PlayerInfo = resp.json().await?;
    Ok(player_info)
}
