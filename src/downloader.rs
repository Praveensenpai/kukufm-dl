use anyhow::{Context, Result};
use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::audio::AudioProcessor;
use crate::client::HttpClientPair;
use crate::kukufm::KuKuClient;
use crate::m3u8::M3U8Parser;
use crate::models::Episode;
use crate::utils::human_readable_size;

pub struct DownloaderConfig {
    pub show_url: String,
    pub from_ep: usize,
    pub to_ep: usize,
    pub parallel_downloads: usize,
    pub download_dir: PathBuf,
}

pub struct Downloader {
    config: DownloaderConfig,
    kukufm: Arc<KuKuClient>,
    m3u8_parser: Arc<M3U8Parser>,
    audio_processor: Arc<AudioProcessor>,
}

impl Downloader {
    pub fn new(config: DownloaderConfig, clients: HttpClientPair) -> Self {
        Self {
            config,
            kukufm: Arc::new(KuKuClient::new(clients.authenticated.clone())),
            m3u8_parser: Arc::new(M3U8Parser::new(
                clients.authenticated,
                clients.downloader.clone(),
            )),
            audio_processor: Arc::new(AudioProcessor::new(clients.downloader)),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let start_time = Instant::now();
        let slug = KuKuClient::parse_show_slug(&self.config.show_url)?;

        let fetch_spinner = ProgressBar::new_spinner();
        fetch_spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        fetch_spinner.set_message("Fetching show metadata...");
        fetch_spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let (show, episodes) = self
            .kukufm
            .fetch_episodes(&slug, self.config.from_ep, self.config.to_ep)
            .await?;

        fetch_spinner.finish_and_clear();

        println!();
        println!(
            " {}",
            style("⚡ kukufm-dl").cyan().bold()
        );
        println!(
            " {}  {}",
            style("│").dim(),
            style(&show.title).cyan().bold()
        );
        println!(
            " {}  Author:   {}",
            style("│").dim(),
            style(&show.author).white()
        );
        println!(
            " {}  Language: {}",
            style("│").dim(),
            style(&show.language).dim()
        );
        println!(
            " {}  Episodes: {} found in range",
            style("│").dim(),
            style(episodes.len()).yellow().bold()
        );
        println!();

        if episodes.is_empty() {
            println!(" {}", style("ℹ No episodes match specified range.").yellow());
            return Ok(());
        }

        let multi = Arc::new(MultiProgress::new());
        let sem = Arc::new(Semaphore::new(self.config.parallel_downloads));
        let total_bytes = Arc::new(AtomicU64::new(0));
        let mut set = JoinSet::new();

        for ep in episodes {
            let m3u8_parser = Arc::clone(&self.m3u8_parser);
            let audio_processor = Arc::clone(&self.audio_processor);
            let base_dir = self.config.download_dir.clone();
            let permit = Arc::clone(&sem);
            let multi = Arc::clone(&multi);
            let total_bytes = Arc::clone(&total_bytes);

            set.spawn(async move {
                let _permit = permit.acquire().await.unwrap();
                let pb = multi.add(ProgressBar::new(100));
                pb.set_style(
                    ProgressStyle::default_bar()
                        .template(" {spinner:.cyan} [{elapsed_precise}] [{bar:25.cyan/blue}] {pos}% {msg}")
                        .unwrap()
                        .progress_chars("━➤ "),
                );
                pb.enable_steady_tick(std::time::Duration::from_millis(80));

                let res = Self::download_single_episode(
                    ep,
                    m3u8_parser,
                    audio_processor,
                    base_dir,
                    &pb,
                )
                .await;

                match res {
                    Ok(bytes) => {
                        total_bytes.fetch_add(bytes, Ordering::Relaxed);
                    }
                    Err(ref e) => {
                        pb.finish_with_message(format!(
                            "{} Error: {:#}",
                            style("✖").red().bold(),
                            e
                        ));
                    }
                }
                res
            });
        }

        let mut completed = 0;
        while let Some(res) = set.join_next().await {
            if let Ok(Ok(_)) = res {
                completed += 1;
            }
        }

        let elapsed = start_time.elapsed();
        let bytes_sum = total_bytes.load(Ordering::Relaxed);

        println!();
        println!(
            " {} Finished downloading {} episodes ({}) in {:.1?}",
            style("✔").green().bold(),
            style(completed).cyan().bold(),
            style(human_readable_size(bytes_sum)).magenta().bold(),
            elapsed
        );
        println!();

        Ok(())
    }

    async fn download_single_episode(
        ep: Episode,
        parser: Arc<M3U8Parser>,
        processor: Arc<AudioProcessor>,
        base_dir: PathBuf,
        pb: &ProgressBar,
    ) -> Result<u64> {
        pb.set_message(format!("Fetching playlist for {}", ep.title));

        let stream_urls = parser
            .get_stream_urls(&ep.hls_url)
            .await
            .context("Failed to parse playlist")?;

        let show_dir = base_dir.join(&ep.show_title);
        let temp_dir = show_dir.join("temp").join(format!("ep_{}", ep.index));
        fs::create_dir_all(&temp_dir)?;

        let total_segments = stream_urls.len();
        pb.set_length(total_segments as u64);
        pb.set_position(0);
        pb.set_message(format!("{}", style(&ep.title).dim()));

        let segment_sem = Arc::new(Semaphore::new(10));
        let mut seg_set = JoinSet::new();

        for (idx, url) in stream_urls.into_iter().enumerate() {
            let parser = Arc::clone(&parser);
            let seg_path = temp_dir.join(format!("{:05}.ts", idx));
            let permit = Arc::clone(&segment_sem);

            seg_set.spawn(async move {
                let _permit = permit.acquire().await.unwrap();
                parser.download_segment(&url, seg_path).await
            });
        }

        let mut segments = Vec::new();
        let mut downloaded_count = 0;

        while let Some(res) = seg_set.join_next().await {
            let seg_path = res??;
            segments.push(seg_path);
            downloaded_count += 1;
            pb.set_position(downloaded_count);
        }
        segments.sort();

        pb.set_message(format!("Merging {}", style(&ep.title).dim()));

        let output_path = ep.output_path(&base_dir);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        processor.concat_segments(&segments, &output_path, &temp_dir)?;
        let _ = fs::remove_dir_all(&temp_dir);

        let size = fs::metadata(&output_path).map(|m| m.len()).unwrap_or(0);

        pb.set_message(format!("Tagging {}", style(&ep.title).dim()));
        let _ = processor.tag_audio(&ep, &output_path).await;

        pb.finish_with_message(format!(
            "{} {} ({})",
            style("✔").green().bold(),
            style(&ep.title).bold(),
            style(human_readable_size(size)).dim()
        ));

        Ok(size)
    }
}
