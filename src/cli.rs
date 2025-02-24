use clap::Parser;
use arboard::Clipboard;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use std::{
    env,
    error::Error,
    process::{self, Stdio},
};

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
const DAEMONIZE_ARG: &str = "__internal_daemonize";

pub fn copy_text_to_clipboard(text: &str) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    {
        // If we are not in daemon mode already, spawn a detached process.
        if env::args().nth(1).as_deref() != Some(DAEMONIZE_ARG) {
            process::Command::new(env::current_exe()?)
                .arg(DAEMONIZE_ARG)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .current_dir("/") // or another safe directory
                .spawn()?;
           println!("{}",text);

            // In daemon mode: set the clipboard text and wait for it to persist.
            Clipboard::new()?
                .set().wait().text(text)?;
            return Ok(());
        }
    }



    Ok(())
}
