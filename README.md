# kukufm-dl ⚡

A lightning-fast, multi-threaded CLI downloader for **KukuFM** audio content, written in Rust with rich terminal UI and metadata tagging.

---

## ⚡ Features

- **Blazing Fast**: Multi-threaded parallel stream & segment downloads.
- **Rich Terminal UI**: Animated progress bars and status indicators powered by `indicatif`.
- **Full Metadata**: Embedded ID3/MP4 metadata (Title, Artist, Album, Cover Art).
- **Cross-Platform**: Pre-built binaries for Linux, macOS, and Windows.

---

## 📦 Getting Started

### Step 1: Download & Extract Binary

1. Go to **[Releases](https://github.com/Praveensenpai/kukufm-dl/releases)** and download the archive for your OS:
   - **Linux**: `kukufm-dl-linux-amd64.tar.gz`
   - **macOS (Apple Silicon)**: `kukufm-dl-macos-arm64.tar.gz`
   - **macOS (Intel)**: `kukufm-dl-macos-x86_64.tar.gz`
   - **Windows**: `kukufm-dl-windows-amd64.zip`
2. Extract the downloaded archive (`.tar.gz` or `.zip`).
3. Open Terminal / Command Prompt inside the extracted folder.

*(Or build from source with `cargo build --release`)*

---

### Step 2: Cookie Setup

1. Copy `cookies_example.txt` to `cookies.txt` (or create a file named `cookies.txt` in the same folder as the binary):
   ```bash
   cp cookies_example.txt cookies.txt
   ```
2. Log in to [KukuFM](https://kukufm.com) in your browser.
3. Open Developer Tools (`F12`), switch to the **Console** tab, and run:
   ```js
   copy(document.cookie)
   ```
4. Paste the copied text into `cookies.txt` and save.

---

## 🚀 Usage

Run the command in your terminal/Command Prompt inside the binary folder:

### Linux / macOS:
```bash
./kukufm-dl --url https://kukufm.com/show/revenge-of-my-fake-boyfriend-8 --from-ep 1 --to-ep 10 --parallel-downloads 3
```

### Windows (Command Prompt / PowerShell):
```powershell
.\kukufm-dl.exe --url https://kukufm.com/show/revenge-of-my-fake-boyfriend-8 --from-ep 1 --to-ep 10 --parallel-downloads 3
```

---

## 📖 Command Arguments

| Parameter | Example Value | Description |
| :--- | :--- | :--- |
| `--url` | `https://kukufm.com/show/slug` | **Required.** Full web URL of the show. |
| `--from-ep` | `1` | Start episode number (default: `1`). |
| `--to-ep` | `10` | End episode number (set `0` for all remaining). |
| `--parallel-downloads` | `3` | Concurrent episode downloads (recommended: `3` to `5`). |

---

## ⚠️ Requirements

- `ffmpeg` installed and available in system `PATH`.
- Active KukuFM subscription cookies in `cookies.txt`.

---

## ⚖️ License

For personal offline listening only. Respect content creators and KukuFM terms of service.