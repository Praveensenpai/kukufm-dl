use anyhow::{bail, Context, Result};
use lofty::prelude::*;
use lofty::probe::Probe;
use lofty::tag::Tag;
use reqwest::Client;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::models::Episode;

pub struct AudioProcessor {
    downloader: Client,
}

impl AudioProcessor {
    pub fn new(downloader: Client) -> Self {
        Self { downloader }
    }

    pub fn concat_segments(
        &self,
        segments: &[PathBuf],
        output_path: &Path,
        temp_dir: &Path,
    ) -> Result<()> {
        let concat_list_path = temp_dir.join("file_list.txt");
        let mut list_file = fs::File::create(&concat_list_path)?;

        for seg in segments {
            let abs_path = fs::canonicalize(seg)?;
            writeln!(list_file, "file '{}'", abs_path.display())?;
        }
        drop(list_file);

        let output = Command::new("ffmpeg")
            .arg("-y")
            .arg("-f")
            .arg("concat")
            .arg("-safe")
            .arg("0")
            .arg("-i")
            .arg(&concat_list_path)
            .arg("-c")
            .arg("copy")
            .arg(output_path)
            .output()
            .context("Failed to execute FFmpeg command")?;

        let _ = fs::remove_file(&concat_list_path);

        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            bail!("FFmpeg failed: {}", err_msg);
        }

        Ok(())
    }

    pub async fn tag_audio(&self, episode: &Episode, audio_file: &Path) -> Result<()> {
        let mut tagged_file = match Probe::open(audio_file)?.read() {
            Ok(tf) => tf,
            Err(e) => bail!("Failed to read audio tags: {:?}", e),
        };

        let tag = match tagged_file.primary_tag_mut() {
            Some(t) => t,
            None => {
                let tag_type = tagged_file.primary_tag_type();
                tagged_file.insert_tag(Tag::new(tag_type));
                tagged_file.primary_tag_mut().unwrap()
            }
        };

        tag.set_title(episode.title.clone());
        tag.set_artist(episode.author.clone());
        tag.set_album(episode.show_title.clone());

        if let Ok(resp) = self.downloader.get(&episode.cover_url).send().await {
            if let Ok(bytes) = resp.bytes().await {
                let picture = lofty::picture::Picture::new_unchecked(
                    lofty::picture::PictureType::CoverFront,
                    Some(lofty::picture::MimeType::Jpeg),
                    None,
                    bytes.to_vec(),
                );
                tag.push_picture(picture);
            }
        }

        tagged_file.save_to_path(audio_file, lofty::config::WriteOptions::default())?;
        Ok(())
    }
}
