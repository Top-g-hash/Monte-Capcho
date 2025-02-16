use std::process::Command;
use tempfile::NamedTempFile;
use tesseract::Tesseract;
use anyhow::Result;
use image::{DynamicImage, ImageBuffer, Luma};






pub fn capture_and_process() -> Result<String> {
    // Create temporary file with .png extension
    let tmp_file = NamedTempFile::new()?;
    let screenshot_path = tmp_file.path().with_extension("png");

    // Capture region with slurp
    let slurp_output = {
        let output = Command::new("slurp")
            .output()?;

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

    // Process image
    let img = image::open(&screenshot_path)?;
    let gray = img.grayscale();
    let contrasted = gray.adjust_contrast(2.0);

    // Apply threshold
    let binary = contrasted.to_luma8();
    let binary = ImageBuffer::from_fn(binary.width(), binary.height(), |x, y| {
        if binary.get_pixel(x, y)[0] > 128 {
            Luma([255u8])
        } else {
            Luma([0u8])
        }
    });

    // Save processed image
    let processed_img = DynamicImage::ImageLuma8(binary);
    let processed_tmp = NamedTempFile::new()?;
    let processed_path = processed_tmp.path().with_extension("png");
    processed_img.save(&processed_path)?;

    // Perform OCR
    let image_path_str = processed_path.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid path"))?;

    let mut tess = Tesseract::new(None, Some("eng"))?;
    tess = tess.set_image(image_path_str)?;
    let text = tess.get_text()?;

    Ok(text)
}

