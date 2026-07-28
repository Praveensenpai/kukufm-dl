use anyhow::{Context, Result};
use reqwest::cookie::Jar;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, USER_AGENT};
use reqwest::{Client, Url};
use std::fs;
use std::sync::Arc;
use std::time::Duration;

pub struct HttpClientPair {
    pub authenticated: Client,
    pub downloader: Client,
}

impl HttpClientPair {
    pub fn new() -> Result<Self> {
        let jar = Arc::new(Jar::default());
        let site_url = "https://kukufm.com".parse::<Url>()?;

        if let Ok(content) = fs::read_to_string("cookies.txt") {
            for pair in content.split(';') {
                let pair = pair.trim();
                if pair.is_empty() {
                    continue;
                }
                if let Some((name, val)) = pair.split_once('=') {
                    let cookie_line = format!("{}={}; Domain=kukufm.com; Path=/", name.trim(), val.trim());
                    jar.add_cookie_str(&cookie_line, &site_url);
                }
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
