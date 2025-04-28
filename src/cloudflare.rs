use reqwest;
use serde_json::{self, json, Value};
use std::env;

pub struct Cloudflare {
    email: String,
    api_key: String,
    zone_id: String
}

impl Cloudflare {
    pub fn init() -> Self {
        Self {
            email: env::var("CLOUDFLARE_EMAIL").expect("CLOUDFLARE_EMAIL not set"),
            api_key: env::var("CLOUDFLARE_API_KEY").expect("CLOUDFLARE_API_KEY not set"),
            zone_id: env::var("ZONE_ID").expect("ZONE_ID not set"),
        }
    }

    fn get_records_url(&self) -> String {
        format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", self.zone_id)
    }

    fn get_record_url(&self, record_id: &str) -> String {
        format!("{}/{record_id}", self.get_records_url())
    }

    pub async fn get_record_id(&self, record_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = self.get_records_url();
        let records: Value = self.get(&url).await?;

        let record = records["result"]
            .as_array()
            .and_then(|arr| {
                arr.iter().find(|r| {
                    r["type"].as_str() == Some("A") &&
                    r["name"].as_str() == Some(record_name)
                })
            });

        if let Some(record) = record {
            let id = record["id"].as_str().ok_or(format!("Failed to parse record ID in {record}"))?;
            Ok(id.to_string())
        } else {
            Err(Box::from(format!("Record {} not found", record_name)))
        }
    }

    pub async fn get_record_content(&self, record_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = self.get_record_url(record_id);
        let record: Value = self.get(&url).await?;

        let content = record["result"]["content"].as_str()
            .ok_or("Failed to parse content in DNS record")?;
        Ok(content.to_string())
    }

    pub async fn set_record_content(&self, record_id: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = self.get_record_url(record_id);
        let response: Value = self.patch(&url, &json!({"content": content})).await?;

        if !response["success"].as_bool().unwrap_or(false) {
            return Err(Box::from(format!(
                "Failed to update DNS record: {:?}",
                response
            )));
        }
        Ok(())
    }

    async fn get(&self, url: &str) -> Result<Value, Box<dyn std::error::Error>> {
        Ok(reqwest::Client::new()
            .get(url)
            .header("X-Auth-Email", &self.email)
            .bearer_auth(&self.api_key)
            .send().await?
            .json().await?)
    }

    async fn patch<T: serde::Serialize>(&self, url: &str, content: &T) -> Result<Value, Box<dyn std::error::Error>> {
        Ok(reqwest::Client::new()
            .patch(url)
            .header("X-Auth-Email", &self.email)
            .bearer_auth(&self.api_key)
            .json(&content)
            .send().await?
            .json().await?)
    }
}