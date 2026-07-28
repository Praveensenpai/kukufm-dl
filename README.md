# kukufm-dl ⚡

A lightning-fast, multi-threaded CLI downloader for **KukuFM** audio content, rewritten in Rust with rich terminal UI and automatic metadata tagging.

---

## ⚡ Features

- **Blazing Fast**: Multi-threaded parallel stream & segment downloads.
- **Rich Terminal UI**: Animated progress bars and status indicators powered by `indicatif`.
- **Full Metadata**: Embedded ID3/MP4 metadata (Title, Artist, Album, Cover Art).
- **Cross-Platform**: Pre-built binaries for Linux, macOS, and Windows.

---

## 📦 Installation

### Pre-built Binaries
Download the latest binary for your operating system from [Releases](https://github.com/Praveensenpai/kukufm-dl/releases).

### Build from Source
```bash
git clone https://github.com/Praveensenpai/kukufm-dl.git
cd kukufm-dl
cargo build --release
```

---

## 🍪 Cookie Setup

1. Copy `cookies_example.txt` to `cookies.txt`:
   ```bash
   cp cookies_example.txt cookies.txt
   ```
2. Log in to [KukuFM](https://kukufm.com) in your browser.
3. Open Developer Tools (`F12`), open the **Console** tab, and run:
   ```js
   copy(document.cookie)
   ```
4. Paste the copied string into `cookies.txt`.

---

## 🚀 Usage

```bash
kukufm-dl --url https://kukufm.com/show/slug --from-ep 1 --to-ep 10 --parallel-downloads 3
```

### Options

| Flag | Description | Default |
| :--- | :--- | :--- |
| `--url <URL>` | Full show URL (*required*) | - |
| `--from-ep <NUM>` | Start episode number | `1` |
| `--to-ep <NUM>` | End episode number (`0` for all) | `0` |
| `--parallel-downloads <NUM>` | Concurrent episode download workers | `1` |

---

## ⚠️ Requirements

- `ffmpeg` installed and added to system `PATH`.
- Active KukuFM subscription cookies in `cookies.txt`.

---

## ⚖️ License

For personal offline listening only. Respect content creators and KukuFM terms of service.