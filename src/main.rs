use reqwest;
use std::env;
use std::error::Error;
use serde_json::Value;
mod episode_title;

async fn get_chatgpt(api_key: String, title: &str) -> Result<String, Box<dyn Error>> {
    // Set up the client and the request
    let client = reqwest::Client::new();
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "temperature": 0.5,
            "messages": [{"role": "user", "content": format!("Create a Doctor Who episode synopsis for the ficticious episode '{}'", title)}]
        }))
        .send()
        .await?;
    let data: Value = response.json().await?;
    // Return the response text
    Ok(data["choices"][0]["message"]["content"].to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the API key from an environment variable
    let api_key = env::var("CHAT_GPT_API_KEY").expect("Expected an API key in the environment");
    let episode = episode_title::get_episode_title();
    let response = get_chatgpt(api_key, &episode).await?;
    println!("Response: {}, {} ", episode, response );// response);

    Ok(())
}
