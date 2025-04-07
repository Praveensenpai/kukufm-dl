# 🎧 Kukufm Premium Downloader

A lightning-fast tool to download premium Kukufm content with full metadata preservation. Perfect for offline listening!

## 🌟 Features
- ▶️ Download full show seasons with single command
- 🚀 Parallel downloads (3-5x faster)
- 📁 Automatic metadata embedding (title/author/cover/description)
- 🔍 Interactive Rich-powered CLI interface
- ♻️ Auto-cleanup of temporary files
- 📈 Progress tracking and size estimates

## ⚠️ Prerequisites
- Active Kukufm **Premium Account** (paid or trial)
- FFmpeg installed system-wide
- Cookies from logged-in session

## 🚀 Quick Start

## 🐙 Clone Repo

```
git clone https://github.com/praveensenpai/kukufm-dl.git
cd kukufm-dl
```

## 🛠 Installation

### Install `uv`

#### On macOS and Linux:
```sh
curl -LsSf https://astral.sh/uv/install.sh | sh
```

#### On Windows:
```sh
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

### Install Dependencies
```sh
uv sync
```

## 🛠 Configure Cookies
### Rename
```
cookies_example.txt -> cookies.txt
```

## 🔐 Getting Cookies (Required)

- Login to Kukufm in Chrome/Firefox

- Open Developer Tools (F12 or Ctrl+Shift+I)

- Navigate to Console tab

- Run this command:
```
copy(document.cookie)
```
- Paste contents into cookies.txt (include ALL text)

## 💻 Basic Usage

Follow the interactive prompts:

- Show URL: Full web URL (e.g., https://kukufm.com/show/show-name)

- Start Episode: First episode number to download

- End Episode: 0 for all available episodes

- Parallel Downloads: Recommended 3-5 for best performance

- 📂 Output Structure:
    ```
    downloads/
    └── Show Name/
        ├── Show Name - Episode 1.m4a
        ├── Show Name - Episode 2.m4a
        └── ...
    ```

## 🚨 Troubleshooting
### Common Issues

#### Q: Getting 403 Forbidden errors?
    Check the cookies or check if the login is exist in browser
    Ensure account has active subscription

#### Q: FFmpeg not found?
#### Verify installation
```
ffmpeg -version
```

# ⚖️ Legal Notice

#### This tool is intended for personal use only with legally obtained Premium accounts. Distributing downloaded content violates Kukufm's terms of service. Support creators by maintaining an active subscription.
