use serde::{Serialize, Deserialize};

/// Basic feed options
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GenerationOptions {
    // Height of the image in pixels. Must be in increments of 64 and pass the following validation:
    // For 768 engines: 589,824 ≤ height * width ≤ 1,048,576
    // All other engines: 262,144 ≤ height * width ≤ 1,048,576
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    // Width of the image in pixels. Must be in increments of 64 and pass the following validation:
    // For 768 engines: 589,824 ≤ height * width ≤ 1,048,576
    // All other engines: 262,144 ≤ height * width ≤ 1,048,576
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    // How strictly the diffusion process adheres to the prompt text.
    // Higher values keep your image closer to your prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubcfg_scale: Option<f32>,

    // Which clip guidance preset to use for the diffusion process.
    // Valid options: FAST_BLUE, FAST_GREEN, NONE, SIMPLE, SLOW, SLOWER, SLOWEST
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_guidance_preset: Option<ClipGuidancePreset>,

    // Which sampler to use for the diffusion process.
    // Valid options: DDIM, DDPM, K_DPMPP_2M, K_DPMPP_2S_ANCESTRAL, K_DPM_2, K_DPM_2_ANCESTRAL, K_EULER, K_EULER_ANCESTRAL, K_HEUN, KLMS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<Sampler>,

    // Number of images to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samples: Option<u32>,

    // Random noise seed (use 0 for a random seed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    // Number of diffusion steps to run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<u32>,

    // Pass in a style preset to guide the image model towards a particular style.
    // Valid options: 3D_MODEL, ANALOG_FILM, ANIME, CINEMATIC, COMIC_BOOK, DIGITAL_ART,
    // ENHANCE, FANTASY_ART, ISOMETRIC, LINE_ART, LOW_POLY, MODELING_COMPOUND,
    // NEON_PUNK, ORIGAMI, PHOTOGRAPHIC, PIXEL_ART, TILE_TEXTURE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_preset: Option<StylePreset>,

    // Extra parameters passed to the engine.
    // These parameters are used for in-development or experimental features
    // and may change without warning, so use with caution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClipGuidancePreset {
    FastBlue,
    FastGreen,
    None,
    Simple,
    Slow,
    Slower,
    Slowest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Sampler {
    Ddim,
    Ddpm,
    KDpmpp2M,
    KDpmpp2SAncestral,
    KDpm2,
    KDpm2Ancestral,
    KEuler,
    KEulerAncestral,
    KHeun,
    KLms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StylePreset {
    Model3D,
    AnalogFilm,
    Anime,
    Cinematic,
    ComicBook,
    DigitalArt,
    Enhance,
    FantasyArt,
    Isometric,
    LineArt,
    LowPoly,
    ModelingCompound,
    NeonPunk,
    Origami,
    Photographic,
    PixelArt,
    TileTexture,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Extras {} // Define extra parameters as needed
    
