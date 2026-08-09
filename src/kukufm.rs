use anyhow::{bail, Context, Result};
use reqwest::Client;
use tokio::time::{sleep, Duration};

use crate::models::{ApiResponse, Episode, ShowInfo};

pub struct KuKuClient {
    client: Client,
}

impl KuKuClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn parse_show_slug(url: &str) -> Result<String> {
        let parts: Vec<&str> = url.trim().split('/').collect();
        if let Some(idx) = parts.iter().position(|&p| p == "show") {
            if let Some(&slug) = parts.get(idx + 1) {
                if !slug.is_empty() {
                    return Ok(slug.to_string());
                }
            }
        }
        bail!("Could not extract show slug from URL: {}", url);
    }

    async fn fetch_json(&self, url: &str) -> Result<ApiResponse> {
        for attempt in 1..=3 {
            match self.client.get(url).send().await {
                Ok(resp) => {
                    if resp.status().as_u16() == 404 {
                        bail!("Show or episode page not found (404)");
                    }
                    if resp.status().is_success() {
                        let text = resp.text().await.context("Failed to read response body")?;
                        let api_res: ApiResponse = serde_json::from_str(&text)
                            .with_context(|| format!("Failed to parse JSON response from {}: {}", url, text))?;
                        return Ok(api_res);
                    }
                }
                Err(e) if attempt == 3 => return Err(e.into()),
                Err(_) => {}
            }
            sleep(Duration::from_secs(3)).await;
        }
        bail!("Failed to fetch URL after 3 retries: {}", url);
    }

    pub async fn get_show(&self, slug: &str) -> Result<ShowInfo> {
        let url = format!("https://kukufm.com/api/v2.1/channels/{}/episodes?lang=english&page=1", slug);
        let res = self.fetch_json(&url).await?;

        let show = res.show.context("Missing 'show' key in API response")?;
        let author_name = show.author.and_then(|a| a.name).unwrap_or_default();

        Ok(ShowInfo {
            title: show.title.unwrap_or_default(),
            description: show.description.unwrap_or_default(),
            author: author_name,
            language: show.language.unwrap_or_default(),
            total_pages: res.n_pages.unwrap_or(1),
        })
    }

    pub async fn fetch_episodes(
        &self,
        slug: &str,
        from_ep: usize,
        to_ep: usize,
    ) -> Result<(ShowInfo, Vec<Episode>)> {
        let show = self.get_show(slug).await?;
        let per_page = 10;
        let mut current_page = if from_ep > 0 { ((from_ep - 1) / per_page) + 1 } else { 1 };

        let mut episodes = Vec::new();

        loop {
            let url = format!(
                "https://kukufm.com/api/v2.1/channels/{}/episodes?lang=english&page={}",
                slug, current_page
            );

            let res = match self.fetch_json(&url).await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error fetching JSON for page {}: {:#}", current_page, e);
                    break;
                }
            };

            let raw_eps = match res.episodes {
                Some(eps) if !eps.is_empty() => eps,
                _ => break,
            };

            let mut reached_limit = false;
            for ep in raw_eps {
                let idx = ep.index.unwrap_or(0);
                if idx < from_ep {
                    continue;
                }
                if to_ep > 0 && idx > to_ep {
                    reached_limit = true;
                    break;
                }

                let hls = match ep.content.and_then(|c| c.hls_url) {
                    Some(u) if !u.is_empty() => u,
                    _ => continue,
                };

                episodes.push(Episode {
                    show_title: show.title.clone(),
                    title: ep.title.unwrap_or_else(|| format!("Episode - {}", idx)),
                    index: idx,
                    duration_seconds: ep.duration_s.unwrap_or(0),
                    hls_url: hls,
                    cover_url: ep.image.unwrap_or_default(),
                    author: show.author.clone(),
                    language: show.language.clone(),
                    description: show.description.clone(),
                });
            }

            if reached_limit {
                break;
            }
            current_page += 1;
        }

        Ok((show, episodes))
    }
}
