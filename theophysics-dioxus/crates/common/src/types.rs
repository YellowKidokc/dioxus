use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Series {
    GTQ,
    MDA,
    Duality,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub series: Series,
    pub reading_level: String,
    pub claims_count: u32,
    pub audio_url: Option<String>,
    pub video_url: Option<String>,
    pub previous_slug: Option<String>,
    pub next_slug: Option<String>,
    pub narrative: String,
    pub system: String,
    pub academic: String,
}
