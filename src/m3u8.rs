use anyhow::{bail, Context, Result};
use m3u8_rs::Playlist;
use reqwest::Client;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct M3U8Parser {
    client: Client,
    downloader: Client,
}

impl M3U8Parser {
    pub fn new(client: Client, downloader: Client) -> Self {
        Self { client, downloader }
    }

    pub async fn get_stream_urls(&self, hls_url: &str) -> Result<Vec<String>> {
        let resp = self.client.get(hls_url).send().await?.bytes().await?;

        let playlist_url = match m3u8_rs::parse_playlist(&resp) {
            Ok((_, Playlist::MasterPlaylist(pl))) => {
                let variant = pl.variants.first().context("Empty master playlist")?;
                let mut base = hls_url.to_string();
                if let Some(pos) = base.rfind('/') {
                    base.truncate(pos + 1);
                }
                format!("{}{}", base, variant.uri)
            }
            Ok((_, Playlist::MediaPlaylist(_))) => hls_url.to_string(),
            Err(e) => bail!("Failed to parse master M3U8 playlist: {:?}", e),
        };

        let media_bytes = self.client.get(&playlist_url).send().await?.bytes().await?;
        let mut base_media = playlist_url;
        if let Some(pos) = base_media.rfind('/') {
            base_media.truncate(pos + 1);
        }

        match m3u8_rs::parse_playlist(&media_bytes) {
            Ok((_, Playlist::MediaPlaylist(pl))) => {
                let urls = pl
                    .segments
                    .into_iter()
                    .map(|seg| {
                        if seg.uri.starts_with("http://") || seg.uri.starts_with("https://") {
                            seg.uri
                        } else {
                            format!("{}{}", base_media, seg.uri)
                        }
                    })
                    .collect();
                Ok(urls)
            }
            _ => bail!("Failed to parse media M3U8 playlist"),
        }
    }

    pub async fn download_segment(&self, url: &str, target_path: PathBuf) -> Result<PathBuf> {
        let bytes = self.downloader.get(url).send().await?.bytes().await?;
        let mut file = File::create(&target_path).await?;
        file.write_all(&bytes).await?;
        Ok(target_path)
    }
}
