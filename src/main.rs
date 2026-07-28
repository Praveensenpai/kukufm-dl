mod audio;
mod client;
mod downloader;
mod kukufm;
mod m3u8;
mod models;
mod utils;

use anyhow::{bail, Result};
use clap::Parser;
use std::path::PathBuf;

use client::HttpClientPair;
use downloader::{Downloader, DownloaderConfig};
use utils::{delete_all_temp_folders, make_dirs};

/// Lightning-fast Kukufm Episode Downloader
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Show URL (e.g. https://kukufm.com/show/slug)
    #[arg(long)]
    url: String,

    /// Start episode number (>= 1)
    #[arg(long, default_value_t = 1)]
    from_ep: usize,

    /// End episode number (0 for all remaining)
    #[arg(long, default_value_t = 0)]
    to_ep: usize,

    /// Parallel episode download workers
    #[arg(long, default_value_t = 1)]
    parallel_downloads: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.url.contains("/show/") {
        bail!("Invalid --url: Must contain '/show/'");
    }
    if cli.from_ep < 1 {
        bail!("Invalid --from-ep: Must be >= 1");
    }
    if cli.to_ep != 0 && cli.from_ep > cli.to_ep {
        bail!("Invalid range: --to-ep cannot be less than --from-ep");
    }
    if cli.parallel_downloads < 1 {
        bail!("Invalid --parallel-downloads: Must be >= 1");
    }

    let download_dir = PathBuf::from("downloads");
    make_dirs(&download_dir)?;
    let _ = delete_all_temp_folders(&download_dir);

    let clients = HttpClientPair::new()?;
    let config = DownloaderConfig {
        show_url: cli.url,
        from_ep: cli.from_ep,
        to_ep: cli.to_ep,
        parallel_downloads: cli.parallel_downloads,
        download_dir,
    };

    let downloader = Downloader::new(config, clients);
    downloader.run().await?;

    Ok(())
}
