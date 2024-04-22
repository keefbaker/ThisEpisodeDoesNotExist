use reqwest;
use std::env;
use std::fs;
use std::error::Error;
use serde_json::Value;
use rand::seq::SliceRandom;
mod episode_title;
mod facebook;
mod cfg;
mod image;

async fn get_chatgpt(api_key: String, title: &str) -> Result<String, Box<dyn Error>> {
    // Set up the client and the request
    let client = reqwest::Client::new();
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "temperature": 0.5,
            "messages": [{"role": "user", "content": format!("Create a Doctor Who episode synopsis for the ficticious episode '{}'. The doctor is played by {}. Don't include any newlines in your response. Don't include the episode title in your response. Also don't guess what words I meant, take the words verbatim.", title, thedoctor())}]
        }))
        .send()
        .await?;
    let data: Value = response.json().await?;
    // Return the response text
    Ok(data["choices"][0]["message"]["content"].to_string())
}

fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let (first, rest) = word.split_at(1);
            first.to_uppercase() + rest
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the API key from an environment variable
    let api_key = env::var("CHAT_GPT_API_KEY").expect("Expected an API key in the environment");
    let episode = episode_title::get_episode_title();
    let response = get_chatgpt(api_key, &episode).await?;
    let capitalized = capitalize_words(&episode);
    let new_response = &response[1..response.len()-1];
    println!("Response: {}, {} ", episode, response );
    let photopath = image::generate_image(capitalized.clone(), new_response.to_string()).await?;
    let photo = fs::read(photopath[0].clone())?;
    facebook::post(format!("Doctor Who - {} \n\n{}", capitalized, new_response), cfg::page_id(), cfg::access_token(), photo).await?;
    std::fs::remove_file(photopath[0].clone());
    Ok(())
}

fn thedoctor() -> String {
    let doctors = vec![
        "Willam Hartnell",
        "Peter Cushing",
        "Patrick Troughton",
        "Jon Pertwee",
        "Tom Baker",
        "Peter Davison",
        "Colin Baker",
        "Sylvester McCoy",
        "Paul McGann",
        "John Hurt",
        "Christopher Eccleston",
        "David Tennant",
        "Matt Smith",
        "Peter Capaldi",
        "Jodie Whittaker",
        "Jo Martin",
        "Ncuti Gatwa",
    ];
    let mut rng = rand::thread_rng();
    let doctor = doctors.choose(&mut rng).unwrap().to_string();
    doctor
}