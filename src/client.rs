use color_eyre::Result;
use reqwest::blocking::Client;

use crate::riot::Region;
use crate::api::account::NewAccount;

pub fn add_account(name: String, region: Region) -> Result<()> {
    let mut s = name.split('#');

    let name = s.next().expect("Failed to parse username");
    let tagline = s.next().expect("Failed to parse tagline");

    let client = Client::new();
    let url = "http://localhost:8080/api/account";
    let body = NewAccount {
        username: name.to_string(),
        tagline: tagline.to_string(),
        region: region
    };

    client.post(url)
        .json(&body)
        .send()?
        .error_for_status()?;

    Ok(())
}
