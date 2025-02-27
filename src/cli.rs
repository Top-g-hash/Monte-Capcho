use clap::Parser;


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
