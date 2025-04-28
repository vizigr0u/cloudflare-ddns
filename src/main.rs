use reqwest;
use serde_json::{json, Value};
use std::env;

const DEFAULT_IP_PROVIDER: &str = "https://api.ipify.org";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let email = env::var("CLOUDFLARE_EMAIL").expect("CLOUDFLARE_EMAIL not set");
    let api_key = env::var("CLOUDFLARE_API_KEY").expect("CLOUDFLARE_API_KEY not set");
    let zone_id = env::var("ZONE_ID").expect("ZONE_ID not set");
    let record_id = env::var("DNS_RECORD_ID").expect("DNS_RECORD_ID not set");

    let ip_provider = env::var("IP_PROVIDER_URL").unwrap_or_else(|_| DEFAULT_IP_PROVIDER.to_string());

    let current_ip = reqwest::get(&ip_provider).await?.text().await?;

    if current_ip.is_empty() {
        return Err(Box::from(format!("Failed to fetch IP address from {}", &ip_provider)));
    }

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        zone_id, record_id
    );

    let get_response = client
        .get(&url)
        .header("X-Auth-Email", &email)
        .bearer_auth(&api_key)
        .send()
        .await?;

    if !get_response.status().is_success() {
        return Err(Box::from(format!(
            "Failed to fetch DNS record: {:?}",
            get_response.text().await?
        )));
    }

    let record_data: Value = get_response.json().await?;

    let stored_ip = record_data["result"]["content"]
        .as_str()
        .ok_or("Failed to parse DNS record content")?;

    if stored_ip == current_ip {
        println!("DNS record is up-to-date ({})", current_ip);
        return Ok(());
    }

    println!(
        "Patching DNS record to use {} instead of {}",
        current_ip, stored_ip
    );

    let patch_response = client
        .patch(&url)
        .header("X-Auth-Email", email)
        .bearer_auth(&api_key)
        .json(&json!({"content": current_ip}))
        .send()
        .await?;

    if patch_response.status().is_success() {
        println!("DNS record updated successfully: {:?}", patch_response.text().await?);
    } else {
        eprintln!("Error updating DNS record: {:?}", patch_response.text().await?);
    }

    Ok(())
}
