use reqwest;
use std::error::Error;

// pub async fn post(message: String, page_id: String, access_token: String) -> Result<String, Box<dyn Error>> {

//     let api_url = format!("https://graph.facebook.com/v19.0/{}/feed", page_id);
//     let client = reqwest::Client::new();
//     let params = [("message", message), ("access_token", access_token)];
    

//     let response = client.post(&api_url)
//         .form(&params)
//         .send()
//         .await?;

//     let post_id: serde_json::Value = response.json().await?;
//     println!("Post ID: {}", post_id.to_string());

//     Ok((post_id["id"].to_string()))
// }
pub async fn post(message: String, page_id: String, access_token: String, photo: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let api_url = format!("https://graph.facebook.com/v19.0/{}/photos", page_id);
    let client = reqwest::Client::new();
    let params = [
        ("message", message.clone()),
        ("access_token", access_token.clone()),
    ];

    let response = client.post(&api_url)
        .multipart(reqwest::multipart::Form::new()
            .text("message", message.clone())
            .text("access_token", access_token.clone())
            .part("source", reqwest::multipart::Part::bytes(photo).file_name("photo.png").mime_str("image/png")?))
        .send()
        .await?;

    let post_id: serde_json::Value = response.json().await?;
    println!("Post ID: {}", post_id.to_string());

    Ok(post_id["id"].to_string())
}