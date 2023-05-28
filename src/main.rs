use std::env;
use std::error::Error;

use base64::decode;
use image::io::Reader as ImageReader;
use std::fs::File;

use stable_rs::{Stability, TextPrompt, GenerationOptions, StylePreset, ClipGuidancePreset};

fn base64_to_png(base64_image: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Decode the base64-encoded image
    let decoded_image = decode(base64_image)?;

    // Create an image reader from the decoded image data
    let reader = ImageReader::new(std::io::Cursor::new(decoded_image))
        .with_guessed_format()?
        .decode()?;

    // Save the image as PNG
    let mut output_file = File::create(output_path)?;
    reader.write_to(&mut output_file, image::ImageOutputFormat::Png)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read environment variable "MY_VARIABLE"
    let api_key = env::var("STABILITY_API_KEY").expect("API key is not set");


    let mut stability = Stability::new("test-user-agent", &api_key);
    stability.set_client_id("test-client-id");
    stability.init_client()?;
    
    let user_api = stability.user().expect("error initializing user api");
    let res = user_api.account().await?;
    println!("{}", serde_json::to_string_pretty(&res).unwrap());

    let engine_api = stability.engines().expect("error initializing engine api");
    let res = engine_api.engines().await?;
    println!("{}", serde_json::to_string_pretty(&res).unwrap());

    let generation_api = stability.generations().expect("error initializing generation api");
    let prompts = vec![TextPrompt{
        text: "A cyberpunk anime character, with resemblance to characters from Ghost in the Shell".to_string(),
        weight: None
    }];
    let options = &GenerationOptions {
        style_preset: Some(StylePreset::Anime),
        clip_guidance_preset: Some(ClipGuidancePreset::FastBlue),
        ..Default::default()
    };
    let res = generation_api.text_to_image("stable-diffusion-512-v2-0", prompts, options).await?;
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
    base64_to_png(res.artifacts[0].base64.as_str(), "/tmp/output.png").expect("error writing image");
    Ok(())
}