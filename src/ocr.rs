use std::process::Command;
use tempfile::NamedTempFile;
use tesseract::Tesseract;
use anyhow::{Result, Context};
use image::DynamicImage ;
use imageproc::contrast::adaptive_threshold;

fn detect_display_server() -> &'static str {
    // Check XDG_SESSION_TYPE first (most reliable indicator)
    if let Ok(session) = std::env::var("XDG_SESSION_TYPE") {
        if session == "wayland" {
            return "wayland";
        } else if session == "x11" {
            return "x11";
        }
    }

    // Fallback to checking display environment variables
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        return "wayland";
    } else if std::env::var("DISPLAY").is_ok() {
        return "x11";
    }

    // Default to x11 if unable to determine
    "x11"
}

pub fn capture_and_process() -> Result<String> {
    // Create temporary file with .png extension
    let tmp_file = NamedTempFile::new()?;
    let screenshot_path = tmp_file.path().with_extension("png");

    let display_server = detect_display_server();

    // Capture region and screenshot based on the display server
    if display_server == "wayland" {
        // Wayland: Use slurp and grim

        // Capture region with slurp
        let slurp_output = {
            let output = Command::new("slurp").output()?;
            if !output.status.success() {
                anyhow::bail!("Region selection cancelled");
            }
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        };

        // Capture screenshot with grim
        let status = Command::new("grim")
            .args(["-g", &slurp_output])
            .arg(&screenshot_path)
            .status()?;
        if !status.success() {
            anyhow::bail!("Failed to capture screenshot");
        }
    } else {
        // X11: Use slop and maim

        // Capture region with slop
        let slop_output = {
            let output = Command::new("slop")
                .args(["--format=%x,%y,%w,%h"])
                .output()
                .context("Failed to run slop. Make sure it's installed for X11 region selection")?;
            if !output.status.success() {
                anyhow::bail!("Region selection cancelled");
            }
            let coords = String::from_utf8_lossy(&output.stdout).trim().to_string();
            // Convert slop format to maim format
            let parts: Vec<&str> = coords.split(',').collect();
            if parts.len() >= 4 {
                format!("{}x{}+{}+{}", parts[2], parts[3], parts[0], parts[1])
            } else {
                anyhow::bail!("Invalid region format from slop");
            }
        };

        // Capture screenshot with maim
        let status = Command::new("maim")
            .args(["-g", &slop_output])
            .arg(&screenshot_path)
            .status()
            .context("Failed to run maim. Make sure it's installed for X11 screenshots")?;
        if !status.success() {
            anyhow::bail!("Failed to capture screenshot");
        }
    }

    // Process image (same for both X11 and Wayland)
    let img = image::open(&screenshot_path)?;
    // Convert image to grayscale
    let gray = img.grayscale();
    // Increase contrast
    let contrasted = gray.adjust_contrast(2.0);

    // Convert to an 8-bit grayscale image
    let luma = contrasted.to_luma8();
    // Apply adaptive thresholding.
    // The parameters (block size and constant) may need tuning depending on your images.
let binary = adaptive_threshold(&luma, 15);

    // Save processed image
    let processed_img = DynamicImage::ImageLuma8(binary);
    let processed_tmp = NamedTempFile::new()?;
    let processed_path = processed_tmp.path().with_extension("png");
    processed_img.save(&processed_path)?;

    // Perform OCR
    let image_path_str = processed_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
    let mut tess = Tesseract::new(None, Some("eng"))?;
    tess = tess.set_image(image_path_str)?;
    let text = tess.get_text()?;

    Ok(text)
}
