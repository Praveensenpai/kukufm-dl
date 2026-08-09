use anyhow::{Context, Result};
use reqwest::cookie::Jar;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, USER_AGENT};
use reqwest::{Client, Url};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

pub struct HttpClientPair {
    pub authenticated: Client,
    pub downloader: Client,
}

impl HttpClientPair {
    pub async fn new(cookie_input: Option<&str>, cookie_file: Option<&Path>) -> Result<Self> {
        let jar = Arc::new(Jar::default());
        let site_url = "https://kukufm.com".parse::<Url>()?;

        let downloader = Client::builder()
            .timeout(Duration::from_secs(180))
            .build()
            .context("Failed to build downloader HTTP client")?;

        let parse_and_add_cookies = |raw_text: &str, jar: &Arc<Jar>, site_url: &Url| -> usize {
            let mut added = 0;
            for pair in raw_text.split(';') {
                let pair = pair.trim();
                if pair.is_empty() || pair.starts_with('#') {
                    continue;
                }
                if let Some((name, val)) = pair.split_once('=') {
                    let cookie_line = format!("{}={}; Domain=kukufm.com; Path=/", name.trim(), val.trim());
                    jar.add_cookie_str(&cookie_line, site_url);
                    added += 1;
                }
            }
            added
        };

        if let Some(cookie_str) = cookie_input {
            if cookie_str.starts_with("http://") || cookie_str.starts_with("https://") {
                println!("🌐 Fetching cookies from URL: {}", cookie_str);
                let resp_text = downloader
                    .get(cookie_str)
                    .send()
                    .await
                    .with_context(|| format!("Failed to fetch cookies from URL: {}", cookie_str))?
                    .text()
                    .await
                    .context("Failed to read response body for cookie URL")?;
                let count = parse_and_add_cookies(&resp_text, &jar, &site_url);
                println!("🔑 Loaded {} cookies from URL", count);
            } else {
                let count = parse_and_add_cookies(cookie_str, &jar, &site_url);
                if count > 0 {
                    println!("🔑 Loaded {} cookies from --cookie argument", count);
                } else {
                    eprintln!("⚠️ Warning: Provided --cookie string yielded no key=value pairs");
                }
            }
        } else {
            let target_file = cookie_file.unwrap_or_else(|| Path::new("cookies.txt"));
            let file_str = target_file.to_string_lossy();

            if file_str.starts_with("http://") || file_str.starts_with("https://") {
                println!("🌐 Fetching cookies from URL: {}", file_str);
                let resp_text = downloader
                    .get(file_str.as_ref())
                    .send()
                    .await
                    .with_context(|| format!("Failed to fetch cookies from URL: {}", file_str))?
                    .text()
                    .await
                    .context("Failed to read response body for cookie URL")?;
                let count = parse_and_add_cookies(&resp_text, &jar, &site_url);
                println!("🔑 Loaded {} cookies from URL", count);
            } else if target_file.exists() {
                let content = fs::read_to_string(target_file)
                    .with_context(|| format!("Failed to read cookie file: {}", target_file.display()))?;
                let count = parse_and_add_cookies(&content, &jar, &site_url);
                if count > 0 {
                    println!("🔑 Loaded {} cookies from {}", count, target_file.display());
                } else {
                    eprintln!("⚠️ Warning: Cookie file '{}' is empty or contains no valid cookies", target_file.display());
                }
            } else if cookie_file.is_some() {
                anyhow::bail!("Specified cookie file not found: {}", target_file.display());
            } else {
                eprintln!("⚠️ Warning: 'cookies.txt' not found in current directory.");
                eprintln!("   Downloading premium content requires cookies.");
                eprintln!("   Provide 'cookies.txt' or use '--cookie-file <PATH_OR_URL>' / '--cookie \"<STRING_OR_URL>\"'.");
            }
        }

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en"));
        headers.insert("dnt", HeaderValue::from_static("1"));
        headers.insert("priority", HeaderValue::from_static("u=1, i"));
        headers.insert(
            "sec-ch-ua",
            HeaderValue::from_static(
                "\"Not(A:Brand\";v=\"99\", \"Google Chrome\";v=\"133\", \"Chromium\";v=\"133\"",
            ),
        );
        headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
        headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Linux\""));
        headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
        headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
        headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36",
            ),
        );

        let authenticated = Client::builder()
            .cookie_provider(jar)
            .default_headers(headers)
            .timeout(Duration::from_secs(180))
            .danger_accept_invalid_certs(true)
            .build()
            .context("Failed to build authenticated HTTP client")?;

        let downloader = Client::builder()
            .timeout(Duration::from_secs(180))
            .build()
            .context("Failed to build downloader HTTP client")?;

        Ok(Self {
            authenticated,
            downloader,
        })
    }
}
