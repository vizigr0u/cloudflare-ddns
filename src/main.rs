use cloudflare::Cloudflare;

use reqwest;
use std::env;

mod cloudflare;

async fn fetch_ip(ip_provider: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(ip_provider).await?;
    let ip = response.text().await?;

    if ip.is_empty() {
        return Err(Box::from(format!("Failed to fetch IP address from {}", &ip_provider)));
    }

    Ok(ip)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let ip_provider = env::var("IP_PROVIDER_URL").expect("IP_PROVIDER_URL not set");
    let cf = Cloudflare::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        eprintln!("Usage: {} record_name...", args[0]);
        return Ok(());
    }

    let current_ip = fetch_ip(&ip_provider).await?;

    for i in 1..args.len() {
        let record_name = &args[i];
        let record_id = cf.get_record_id(record_name).await?;

        let stored_ip = cf.get_record_content(&record_id).await?;

        if stored_ip == current_ip {
            println!("DNS record for {record_name} is up-to-date ({current_ip})");
            return Ok(());
        }

        cf.set_record_content(&record_id, &current_ip).await?;
        println!("DNS record for {record_name} updated to {current_ip}");
    }

    Ok(())
}
