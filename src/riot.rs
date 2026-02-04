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

impl Region {
    pub fn api_endpoint(&self) -> &'static str {
        match self {
            Region::NA => "https://na1.api.riotgames.com",
            Region::EUW => "https://euw1.api.riotgames.com",
            Region::EUNE => "https://eun1.api.riotgames.com",
            Region::KR => "https://kr.api.riotgames.com",
            Region::JP => "https://jp1.api.riotgames.com",
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub puuid: String,
    #[serde(rename = "gameName")]
    pub username: String,
    #[serde(rename = "tagLine")]
    pub tagline: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    pub queue_type: String,
    pub puuid: String,
    pub tier: String,
    pub rank: String,
    pub league_points: u32,
    pub wins: u32,
    pub losses: u32,
    pub hot_streak: bool,
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

pub async fn get_player_stats(puuid: &str, region: &Region) -> Result<Vec<PlayerStats>> {
    let url = format!(
        "{}/lol/league/v4/entries/by-puuid/{}",
        region.api_endpoint(),
        puuid
    );

    let client = build_client();
    let resp = client.get(&url).send().await?;

    if !resp.status().is_success() {
        return Err(color_eyre::eyre::eyre!(
            "Failed to fetch player stats from riot api: {}",
            resp.status()
        ));
    }

    let player_stats: Vec<PlayerStats> = resp.json().await?;
    Ok(player_stats)
}
