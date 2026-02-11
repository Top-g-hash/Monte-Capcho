use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView};
use imageproc::contrast::adaptive_threshold;
use imageproc::contrast::threshold;
use imageproc::filter::gaussian_blur_f32;
use std::process::Command;
use tempfile::NamedTempFile;
use tesseract::Tesseract;

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

    // Load image
    let img = image::open(&screenshot_path)?;

    // Convert to grayscale
    let mut gray = img.grayscale().to_luma8();

    // ---- Detect dark theme ----
    let avg_brightness: u64 = gray.pixels().map(|p| p[0] as u64).sum::<u64>()
        / (gray.width() as u64 * gray.height() as u64);

    if avg_brightness < 110 {
        image::imageops::invert(&mut gray);
    }

    // ---- Resize (very important for small fonts) ----
    let resized = image::imageops::resize(
        &gray,
        gray.width() * 2,
        gray.height() * 2,
        image::imageops::FilterType::Lanczos3,
    );

    // ---- Apply slight blur (helps threshold stability) ----
    let blurred = gaussian_blur_f32(&resized, 1.0);

    // ---- Increase contrast ----
    let contrasted = DynamicImage::ImageLuma8(blurred)
        .adjust_contrast(45.0)
        .to_luma8();

    // ---- Hard threshold (2 args only!) ----
    let binary = threshold(&contrasted, 160);

    // Save processed image
    let processed_img = DynamicImage::ImageLuma8(binary);
    let processed_tmp = NamedTempFile::new()?;
    let processed_path = processed_tmp.path().with_extension("png");
    processed_img.save(&processed_path)?;

    // Perform OCR
    let image_path_str = processed_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
    let mut tess = Tesseract::new(None, Some("eng"))?
    .set_variable("preserve_interword_spaces", "1")?
    .set_variable("tessedit_pageseg_mode", "6")?
    .set_variable(
        "tessedit_char_whitelist",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789{}[]()<>;:.,_+-=*/!&|\"'\\# ",
    )?
        .set_variable("textord_heavy_nr", "1")?
      .set_variable("textord_min_linesize", "2.5")?;

    tess = tess.set_image(image_path_str)?;
    let text = tess.get_text()?;

    Ok(text)
}
