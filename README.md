# 🧩 MonteCapcho — Text Extractor for Linux

MonteCapcho is a lightweight Linux tool that lets you capture any region of your screen and extract text using offline OCR.

Built with:
* 🦀 Rust — native performance
* ❄️ Iced GUI — clean and minimal
* 🔍 Tesseract OCR — offline text recognition
* 🖼️ grim + slurp (Wayland)
* 📸 maim + slop (X11)
* 📋 CopyQ for clipboard persistence

## ✨ Features

* Capture a region of your screen
* Extract text instantly using OCR
* One-click copy to clipboard
* Works on Wayland and X11
* Fully offline (no internet needed)
* Simple, centered GUI for viewing/editing text
* CLI mode for quick terminal usage

**Upcoming enhancements:**
* High Accuracy Mode (PaddleOCR)
* Code-aware OCR
* Image preprocessing pipeline
* Better dark-theme OCR support

## 🛠 Dependencies

**Wayland:**
```
grim
slurp
tesseract
leptonica
copyq
```

**X11:**
```
maim
slop
tesseract
leptonica
copyq
```

## 📦 Installation

### Arch Linux (PKGBUILD)

Clone your PKGBUILD directory and run:
```bash
makepkg -si
```

Then launch:
```bash
text-extractor
```

(Once you upload to AUR, installation becomes as easy as: `yay -S text-extractor`)

### Build From Source (Any Linux)

```bash
git clone https://github.com/Top-g-hash/Monte-Capcho
cd Monte-Capcho
cargo build --release
./target/release/text-extractor
```

## 🚀 Usage

### CLI Mode

Capture and extract:
```bash
text-extractor --capture
```

Capture + copy to clipboard:
```bash
text-extractor --capture --copy
```

**Flags:**
* `-c` / `--capture` — perform screenshot + OCR
* `-p` / `--copy` — copy output text to clipboard

### GUI Mode

Simply run:
```bash
text-extractor
```

You'll see:
* Editable text area
* Capture button
* Copy button
* Iced-based UI

## 📁 Project Structure

```
src/                → App source code
fonts/              → Embedded font assets
assets/             → Icons & desktop file
Cargo.toml          → Rust project config
build.rs            → Font embedding / build scripts
```

## 📜 License

This project is dual-licensed under:
* MIT License
* Apache 2.0 License

See the `LICENSE` files for details.

## 👏 Acknowledgments

MonteCapcho is supported by:
* Iced — GUI framework
* Tesseract OCR
* CopyQ — clipboard persistence
* clap — CLI parsing
* Rust community crates

## 💬 Contributing

Issues, suggestions, and pull requests are welcome! Visit: https://github.com/Top-g-hash/Monte-Capchopowering the application.
    Clap – For command-line argument parsing.
    arboard – For clipboard integration.
    iced_fontello – For icon font support.


