# MonteCapcho - Text Extractor

MonteCapcho - Text Extractor is a cross-platform text extraction tool that leverages Optical Character Recognition (OCR) to capture text from a selected region of your screen. The application features both a command-line interface (CLI) and a graphical user interface (GUI) built with the [Iced](https://github.com/iced-rs/iced) framework.

## Features

- **OCR Capture:** Quickly extract text from any part of your screen.
- **Clipboard Integration:** Optionally copy extracted text to the clipboard. On Linux, the clipboard content is made persistent by spawning a daemon.
- **Dual-mode Operation:**
  - **CLI Mode:** Use flags to capture text and copy it without launching a full GUI.
  - **GUI Mode:** Launch an interactive editor with syntax highlighting for viewing and editing extracted text.
- **Cross-Platform:** Works on Windows, macOS, and Linux.
- **Customizable:** Configure window behavior (centered, fixed size, floating on top) and more via Iced's builder API.
- **Flatpak Packaging:** Easily package and distribute your app as a Flatpak.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (recommended edition 2021 or later)
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust’s package manager)

## Installation

1. **Clone the Repository:**

   ```sh
   git clone https://github.com/yourusername/montecapcho-text-extractor.git
   cd montecapcho-text-extractor
2. **Build the Application:**
    For a release build, run:
  ```sh
     cargo build --release
     ```
##  Usage
### Command-Line Mode
Use CLI flags to perform OCR capture and (optionally) copy text to the clipboard:
 ```sh
cargo run -- -c -p
```
   -  -c / --capture: Triggers OCR capture.
   -  -p / --copy: Copies the extracted text to the clipboard. (On Linux, a background daemon is spawned to keep the clipboard content alive.)

## Graphical User Interface (GUI) Mode

Launch the GUI without any flags:
```sh
cargo run
```
The GUI opens as a centered, rectangular window (with floating behavior on Windows) featuring a text editor for viewing and editing captured text. On-screen controls allow you to trigger OCR capture and manage clipboard copying.
## Configuration
### Window Settings

The application uses Iced's window settings to control the appearance and behavior of the GUI:

 -  **Centered:** The window opens in the center of the screen.
  - **Size:** A fixed rectangular window (e.g., 800x600 pixels).
   -  **Floating:** On Windows, the window is set to always stay on top.

These settings are applied using Iced's builder API in the main.rs file.
### Clipboard Persistence on Linux

On Linux, clipboard persistence is achieved by spawning a daemon process that holds the clipboard contents even after the main process exits. See the implementation in cli.rs for details.
### Fonts and Icons

    Icons: The app uses iced_fontello for icon fonts. The configuration is defined in the fonts/ocr-icons.toml file.
    Default Font: A monospaced font is used for the text editor.

## Flatpak Packaging

For packaging your application as a Flatpak, refer to the Flatpak Documentation and customize your Flatpak manifest accordingly. This allows you to distribute your application on a wide range of Linux distributions.
## Contributing

Contributions are welcome! If you have bug reports, feature requests, or code improvements, please open an issue or submit a pull request on GitHub.
## License

This project is licensed under the MIT License. See the LICENSE file for details.
## Acknowledgments

    Iced – The GUI framework powering the application.
    Clap – For command-line argument parsing.
    arboard – For clipboard integration.
    iced_fontello – For icon font support.


