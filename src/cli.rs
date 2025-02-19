use clap::Parser;
use arboard::Clipboard;

#[derive(Parser)]
#[command(name = "MonteCapcho - Text Extractor")]
#[command(about = "Extracts text using OCR from a selected region")]
pub struct Cli {
    /// Trigger screen capture and OCR processing
    #[arg(short = 'c', long)]
    pub capture: bool,

    /// Copy the extracted text to clipboard
    #[arg(short = 'p', long)]
    pub copy: bool,
}
pub fn copy_text_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text.to_string())?;
    #[cfg(target_os = "linux")]
    std::thread::sleep(std::time::Duration::from_secs(2)); // Give time for clipboard handoff
    Ok(())
}
