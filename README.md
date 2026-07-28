# kukufm-dl ⚡

A lightning-fast, multi-threaded CLI downloader for **KukuFM** audio content, written in Rust with rich terminal UI and metadata tagging.

---

## ⚡ Features

- **Blazing Fast**: Multi-threaded parallel stream & segment downloads.
- **Rich Terminal UI**: Animated progress bars and status indicators powered by `indicatif`.
- **Full Metadata**: Embedded ID3/MP4 metadata (Title, Artist, Album, Cover Art).
- **Cross-Platform**: Pre-built binaries for Linux, macOS, and Windows.

---

## 📦 Quick Start

### 1. Download Binary or Build from Source

#### Option A: Download Pre-built Binary
Download the binary for your OS from **[Releases](https://github.com/Praveensenpai/kukufm-dl/releases)** and extract it.

#### Option B: Build from Source
```bash
git clone https://github.com/Praveensenpai/kukufm-dl.git
cd kukufm-dl
cargo build --release
```

---

### 2. Cookie Setup

1. Copy `cookies_example.txt` to `cookies.txt`:
   ```bash
   cp cookies_example.txt cookies.txt
   ```
2. Log in to your account on [KukuFM](https://kukufm.com).
3. Open Developer Tools (`F12`), switch to the **Console** tab, and run:
   ```js
   copy(document.cookie)
   ```
4. Paste the copied string into `cookies.txt`.

---

## 🚀 How to Run

### Using Pre-built Binary

#### Linux / macOS:
```bash
./kukufm-dl --url https://kukufm.com/show/revenge-of-my-fake-boyfriend-8 --from-ep 1 --to-ep 10 --parallel-downloads 3
```

#### Windows (Command Prompt / PowerShell):
```powershell
.\kukufm-dl.exe --url https://kukufm.com/show/revenge-of-my-fake-boyfriend-8 --from-ep 1 --to-ep 10 --parallel-downloads 3
```

---

### Using Cargo (Source Code)

```bash
cargo run --release -- --url https://kukufm.com/show/revenge-of-my-fake-boyfriend-8 --from-ep 1 --to-ep 10 --parallel-downloads 3
```

---

## 📖 Command Arguments Explained

| Parameter | Example Value | Description |
| :--- | :--- | :--- |
| `--url` | `https://kukufm.com/show/slug` | **Required.** Full web URL of the show. |
| `--from-ep` | `1` | Start episode number (default: `1`). |
| `--to-ep` | `10` | End episode number. Set to `0` to download all remaining episodes. |
| `--parallel-downloads` | `3` | Number of episodes to download concurrently (recommended: `3` to `5`). |

---

## ⚠️ Requirements

- `ffmpeg` installed and available in your system `PATH`.
- Active KukuFM subscription cookies in `cookies.txt`.

---

## ⚖️ License

For personal offline listening only. Respect content creators and KukuFM terms of service.