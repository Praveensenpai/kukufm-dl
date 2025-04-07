# 🎧 Kukufm Premium Downloader

Download your favorite **premium Kukufm shows** fast, with full metadata. Perfect for offline binge-listening.

## 🌟 Features

- ▶️ Download full seasons in one go
- 🚀 Fast parallel downloads (3-5x speed boost)
- 🏷️ Auto metadata: title, author, cover, description
- 🔍 Clean, interactive CLI using `rich`
- 🧹 Auto-cleans temp files
- 📈 Shows live progress and file sizes

## ⚠️ Requirements

- A **Premium Kukufm account** (trial or paid)
- `ffmpeg` installed and added to PATH
- Logged-in browser cookies

---

## 🚀 Getting Started

### 🐙 Clone the Repo

```bash
git clone https://github.com/praveensenpai/kukufm-dl.git
cd kukufm-dl
```

---

## 🛠 Setup

### 1. Install `uv`

#### macOS / Linux
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

#### Windows
```powershell
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

---

### 2. Install Dependencies

```bash
uv sync
```

---

## 🍪 Add Your Cookies

### 1. Rename Example File

```bash
mv cookies_example.txt cookies.txt
```

### 2. Get Your Kukufm Cookies

- Login to Kukufm in your browser (Chrome/Firefox)
- Open Dev Tools (F12 or Ctrl+Shift+I)
- Go to the **Console** tab
- Paste and run this:
  ```js
  copy(document.cookie)
  ```
- Paste that copied text into `cookies.txt`

---

## 💻 How to Use

Run this with required flags:

```bash
uv run main.py --url https://kukufm.com/show/xyz --from-ep 1 --to-ep 0 --parallel-downloads 3
```
- Show URL: Full web URL (e.g., https://kukufm.com/show/show-name)

- Start Episode: First episode number to download

- End Episode: 0 for all available episodes

- Parallel Downloads: Recommended 3-5 for best performance


### 📁 Output Structure

```
downloads/
└── Show Name/
    ├── Show Name - Episode 1.m4a
    ├── Show Name - Episode 2.m4a
    └── ...
```

---

## 🧯 Troubleshooting

### ❌ 403 Forbidden?

- Double-check your cookies
- Make sure you're logged in with an active subscription

### ❌ ffmpeg not found?

Check it's installed and in your PATH:

```bash
ffmpeg -version
```

---

## ⚖️ Legal Stuff

This tool is for **personal use only** with a valid Kukufm Premium account. Sharing or redistributing content is against Kukufm's terms. Support creators by keeping your subscription active.