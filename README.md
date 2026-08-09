# kukufm-dl ⚡

A lightning-fast, multi-threaded CLI downloader for **KukuFM** audio content, written in Rust with rich terminal UI and metadata tagging.

---

## ⚡ Features

- **Blazing Fast**: Multi-threaded parallel stream & segment downloads.
- **Rich Terminal UI**: Animated progress bars and status indicators powered by `indicatif`.
- **Full Metadata**: Embedded ID3/MP4 metadata (Title, Artist, Album, Cover Art).
- **Flexible Cookie Sources**: Load cookies from a local `cookies.txt` file, a remote HTTP/HTTPS URL, or an inline cookie string.
- **Cross-Platform**: Pre-built binaries for Linux, macOS, and Windows.

---

## 📦 What's Included in Release Archives

Each downloadable release archive contains:
- `kukufm-dl` / `kukufm-dl.exe` (Main executable binary)
- `cookies_example.txt` (Sample cookies template with setup instructions)
- `README.md` (Documentation)

---

## 📂 File & Directory Structure Guide

Where to place `cookies.txt` depending on how you run `kukufm-dl`:

### Case A: Using Pre-built Release Binary (Downloaded Archive)

Extract the `.zip` or `.tar.gz` archive. Place `cookies.txt` **in the exact same directory** alongside the binary executable:

#### Linux / macOS:
```text
my-folder/
├── kukufm-dl           <-- Main executable binary
├── cookies.txt         <-- Place your cookies file HERE!
└── README.md
```

#### Windows:
```text
my-folder/
├── kukufm-dl.exe       <-- Main executable binary
├── cookies.txt         <-- Place your cookies file HERE!
└── README.md
```

---

### Case B: Building from Source (`cargo build` / `cargo run`)

If you clone the repository and build from source:

#### 1. Running with `cargo run`:
Place `cookies.txt` in the **root repository directory** (where `Cargo.toml` is located):
```text
kukufm-dl/
├── Cargo.toml
├── cookies.txt         <-- Place your cookies file HERE when using `cargo run`
├── src/
└── target/
```
Command:
```bash
cargo run -- --url https://kukufm.com/show/slug
```

#### 2. Running compiled release binary directly:
After running `cargo build --release`, the binary is generated at `target/release/kukufm-dl` (or `target/release/kukufm-dl.exe` on Windows).
You can run it directly from the project root:
```bash
./target/release/kukufm-dl --url https://kukufm.com/show/slug
```

---

## 📦 Getting Started

### Step 1: Download & Extract Archive

1. Go to **[Releases](https://github.com/Praveensenpai/kukufm-dl/releases)** and download the archive for your OS:
   - **Linux**: `kukufm-dl-linux-amd64.tar.gz`
   - **macOS (Apple Silicon)**: `kukufm-dl-macos-arm64.tar.gz`
   - **macOS (Intel)**: `kukufm-dl-macos-x86_64.tar.gz`
   - **Windows**: `kukufm-dl-windows-amd64.zip`
2. Extract the downloaded `.tar.gz` or `.zip` file into a folder.
3. Open Terminal / Command Prompt / PowerShell inside that extracted folder.

---

### Step 2: Cookie Setup (Choose Option A, B, or C)

KukuFM requires active subscription cookies to stream and download premium content.

#### Option A: Local `cookies.txt` File (Default / Recommended)

1. Rename `cookies_example.txt` to `cookies.txt` (or create `cookies.txt` in the same folder as `kukufm-dl`).
2. Log in to [KukuFM](https://kukufm.com) in your web browser.
3. Open **Developer Tools** (`F12` or `Ctrl+Shift+I` / `Cmd+Option+I`).
4. Switch to the **Console** tab and run:
   ```js
   copy(document.cookie)
   ```
5. Open `cookies.txt`, paste your copied cookie string, and save the file. `kukufm-dl` will detect it automatically!

#### Option B: Remote Cookie URL (`--cookie-file <URL>` or `-c <URL>`)

If your cookies are hosted online (e.g. Pastebin, Github Gist, or private server), pass the HTTP/HTTPS URL directly:
```bash
./kukufm-dl --url https://kukufm.com/show/slug -c https://example.com/my_cookies.txt
```

#### Option C: Inline Cookie String (`--cookie "<STRING>"`)

Pass your raw browser cookie string directly in the command line:
```bash
./kukufm-dl --url https://kukufm.com/show/slug --cookie "session=xyz123; token=abc456"
```

---

## 🚀 Usage Examples

### Standard (Using local `cookies.txt` in the same directory):

#### Linux / macOS:
```bash
./kukufm-dl --url https://kukufm.com/show/revenge-of-my-fake-boyfriend-8 --from-ep 1 --to-ep 10 --parallel-downloads 3
```

#### Windows (Command Prompt / PowerShell):
```powershell
.\kukufm-dl.exe --url https://kukufm.com/show/revenge-of-my-fake-boyfriend-8 --from-ep 1 --to-ep 10 --parallel-downloads 3
```

---

### Advanced (Using Custom File, Remote URL, or Direct Cookie):

```bash
# Custom local file path
./kukufm-dl --url https://kukufm.com/show/slug -c /path/to/my_cookies.txt

# Remote Cookie URL
./kukufm-dl --url https://kukufm.com/show/slug -c https://raw.githubusercontent.com/user/repo/main/cookies.txt

# Inline Cookie string
./kukufm-dl --url https://kukufm.com/show/slug --cookie "session=xyz123; token=abc456"
```

---

## 📖 Command Arguments

| Parameter | Short | Default | Description |
| :--- | :---: | :---: | :--- |
| `--url` | | *Required* | Full web URL of the KukuFM show (`https://kukufm.com/show/...`). |
| `--from-ep` | | `1` | Start episode number (>= 1). |
| `--to-ep` | | `0` | End episode number (`0` downloads all remaining episodes). |
| `--parallel-downloads` | | `1` | Concurrent episode download threads (recommended: `3` to `5`). |
| `--cookie-file` | `-c` | `cookies.txt` | Local file path **OR** remote HTTP/HTTPS URL containing KukuFM cookies. |
| `--cookie` | | | Raw cookie string **OR** remote HTTP/HTTPS URL directly. |

---

## ⚠️ Requirements

- `ffmpeg` installed and available in your system `PATH`.
- Active KukuFM subscription cookies via local `cookies.txt`, remote URL, or `--cookie` flag.

---

## ⚖️ License

For personal offline listening only. Respect content creators and KukuFM terms of service.