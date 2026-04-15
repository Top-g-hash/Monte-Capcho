# 🧩 MonteCapcho

> Select any region on your screen and instantly extract text like code snippets or simple CAPTCHA — fully offline.

MonteCapcho was built to reduce the pain of copying code from tutorials and quickly capture text from anywhere on your screen.

Built with **Rust**, **Iced**, and **Tesseract OCR**. Works on both Wayland and X11.

---

## ✨ Features

- **Draw & extract** — select a region and get text in one step
- **Fully offline** — nothing leaves your machine
- **Wayland & X11 support** — uses `grim + slurp` or `maim + slop` depending on your session
- **Simple GUI** — editable text area, capture button, one-click clipboard copy
- **CLI mode** — pipe-friendly, scriptable terminal interface

### Coming soon

- High-accuracy mode
- Code-aware OCR
- Image preprocessing pipeline
- Improved dark-background OCR

---

## 📦 Installation

### Arch Linux — AUR (recommended)

```sh
yay -S montecapcho
```

Or build manually from the PKGBUILD:

```sh
git clone https://aur.archlinux.org/montecapcho.git
cd montecapcho
makepkg -si
```

### Build from source

**Requirements:** Rust 1.82+, Tesseract, Leptonica

```sh
git clone https://github.com/Top-g-hash/Monte-Capcho
cd Monte-Capcho
cargo build --release
./target/release/MonteCapcho
```

---

## 🛠 Dependencies

AUR installations handle these automatically. If building from source, install them manually.

### Common

| Package | Purpose |
|---|---|
| `tesseract` | OCR engine |
| `leptonica` | Image processing library |
| `copyq` | Clipboard persistence |

> **Note:** `copyq` is not installed by default on most distros. Without it, copy-to-clipboard will not work.

```sh
# Arch
sudo pacman -S copyq

# Debian / Ubuntu
sudo apt install copyq
```

### Wayland only

| Package | Purpose |
|---|---|
| `grim` | Screenshot tool |
| `slurp` | Region selection |

### X11 only

| Package | Purpose |
|---|---|
| `maim` | Screenshot tool |
| `slop` | Region selection |

---

## 🚀 Usage

### GUI mode

```sh
MonteCapcho
```

The GUI includes an editable text area, a capture button, and a copy button.

| Shortcut | Action |
|---|---|
| `Ctrl+S` | Capture region and extract text |
| `Ctrl+C` | Copy extracted text to clipboard |

### CLI mode

Capture and print text to stdout:

```sh
MonteCapcho --capture
```

Capture and copy directly to the clipboard:

```sh
MonteCapcho --capture --copy
```

### Flags

| Flag | Description |
|---|---|
| `-c`, `--capture` | Perform screenshot and OCR |
| `-p`, `--copy` | Copy output text to the clipboard |

---

## 📁 Project structure

```
src/         Application source code
fonts/       Embedded font assets
assets/      Icons and .desktop file
Cargo.toml   Rust project config
build.rs     Build script for font embedding
```

---

## 📜 License

Licensed under the [MIT License](LICENSE).

---

## 👏 Acknowledgments

- [Iced](https://github.com/iced-rs/iced) — GUI framework
- [Tesseract OCR](https://github.com/tesseract-ocr/tesseract) — text recognition engine
- [CopyQ](https://hluk.github.io/CopyQ/) — clipboard management
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing

---

## 💬 Contributing

Contributions are welcome — bugs, features, and improvements alike.

1. Fork the repo
2. Create a branch: `git checkout -b feature/my-feature`
3. Commit your changes
4. Open a pull request

Report issues at [github.com/Top-g-hash/Monte-Capcho/issues](https://github.com/Top-g-hash/Monte-Capcho/issues)
