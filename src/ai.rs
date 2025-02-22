// src/ai.rs
use reqwest;
use serde_json::json;
use crate::config::Config;
use crate::error::Error;

pub async fn generate_response(config: &Config, prompt: String) -> Result<String, Error> {
    let api_key = config.api_key.as_str();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-pro-005:generateContent?key={}", api_key);

    let client = reqwest::Client::new();
    let body = json!({
        "contents": [{
            "parts": [{ "text": prompt }]
        }]
    });

        let resp = client.post(url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await;

        let resp = match resp {
            Ok(resp) => resp,
            Err(e) => { return Err(Error::Request(e.into())); }
        };

        let json_result: Result<serde_json::Value, reqwest::Error> = resp.json().await;
        let json: serde_json::Value = match json_result {
            Ok(json) => json,
            Err(e) => {
                let io_error = std::io::Error::new(std::io::ErrorKind::Other, e);
                return Err(Error::Json(serde_json::Error::io(io_error.into())));
            }
        };

    println!("JSON Response: {:?}", json);

    let response_text = json["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("No response").to_string();

    Ok(response_text)
}
