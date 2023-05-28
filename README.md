# stable-rs

stable-rs is a Rust API wrapper for the StabilityAI API. It provides convenient methods for interacting with the StabilityAI services and performing various operations for `/v1/user`, `/v1/engines`, and  `/v1/generation`.

## Installation

Add the following to your Cargo.toml file:


```toml
[dependencies]
stable-rs = "0.1"
```

## Example Usage

```rust
use std::env;
use stable_rs::{Stability, TextPrompt, GenerationOptions, StylePreset};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read environment variable "STABILITY_API_KEY"
    let api_key = env::var("STABILITY_API_KEY").expect("API key is not set");

    // Initialize Stability client
    let mut stability = Stability::new("test-user-agent", &api_key);
    stability.set_client_id("test-client-id");
    stability.init_client()?;

    // User API example
    let user_api = stability.user().expect("Error initializing user API");
    let account_info = user_api.account().await?;
    println!("{}", serde_json::to_string_pretty(&account_info).unwrap());

    // Engine API example
    let engine_api = stability.engines().expect("Error initializing engine API");
    let engine_list = engine_api.engines().await?;
    println!("{}", serde_json::to_string_pretty(&engine_list).unwrap());

    // Generation API example
    let generation_api = stability.generations().expect("Error initializing generation API");
    let prompts = vec![
        TextPrompt {
            text: "A cyberpunk anime character, with resemblance to characters from Ghost in the Shell".to_string(),
            weight: None,
        }
    ];
    let options = GenerationOptions {
        style_preset: Some(StylePreset::Anime),
        
        ..Default::default()
    };
    let generation_result = generation_api.text_to_image("stable-diffusion-512-v2-0", prompts, &options).await?;
    println!("{}", serde_json::to_string_pretty(&generation_result).unwrap());
    Ok(())
}
```

## License ##

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.