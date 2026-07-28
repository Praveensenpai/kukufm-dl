use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ShowInfo {
    pub title: String,
    pub description: String,
    pub author: String,
    pub language: String,
    pub total_pages: usize,
}

#[derive(Debug, Clone)]
pub struct Episode {
    pub show_title: String,
    pub title: String,
    pub index: usize,
    pub duration_seconds: u64,
    pub hls_url: String,
    pub cover_url: String,
    pub author: String,
    pub language: String,
    pub description: String,
}

impl Episode {
    pub fn output_filename(&self) -> String {
        let clean_show = self.show_title.replace('/', "-");
        let clean_title = self.title.replace('/', "-");
        format!("{} - {}.m4a", clean_show, clean_title)
    }

    pub fn output_path(&self, base_dir: &PathBuf) -> PathBuf {
        base_dir.join(&self.show_title).join(self.output_filename())
    }
}

// Raw API Serde types with permissive Option fields
#[derive(Debug, Deserialize)]
pub(crate) struct RawAuthor {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawShow {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<RawAuthor>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawContent {
    pub hls_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawEpisode {
    pub index: Option<usize>,
    pub title: Option<String>,
    pub duration_s: Option<u64>,
    pub content: Option<RawContent>,
    pub image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiResponse {
    pub show: Option<RawShow>,
    pub n_pages: Option<usize>,
    pub episodes: Option<Vec<RawEpisode>>,
}
