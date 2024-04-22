use stabilityai::{
    error::StabilityAIError,
    types::{ClipGuidancePreset, Sampler, StylePreset, TextToImageRequestBodyArgs},
    Client,
};
use std::error::Error;
use std::path::PathBuf;

pub async fn generate_image(title: String, synopsis: String) -> Result<Vec<PathBuf>, StabilityAIError> {
     // create client, reads STABILITY_API_KEY environment variable for API key.
    let client = Client::new();

    let request = TextToImageRequestBodyArgs::default()
        .text_prompts(
            format!("Generate a poster for the ficticious Doctor Who episode '{}'. If an enemy is named in the episode, you must show it in the picture. Synopsis for the episode is: {}", title, synopsis),
        )
        .samples(1)
        .steps(30_u32)
        .clip_guidance_preset(ClipGuidancePreset::FastBlue)
        .sampler(Sampler::KDpmpp2sAncestral)
        .width(1216_u16)
        .height(832_u16)
        .style_preset(StylePreset::Photographic)
        .build()?;

    let artifacts = client
        .generate("stable-diffusion-xl-1024-v1-0")
        .text_to_image(request)
        .await?;

    // Create directory if doesn't exist and save images
    let paths = artifacts.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image saved at {}", path.display()));

    Ok(paths)
}