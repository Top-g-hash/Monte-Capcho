use std::process::Command;
use image::{DynamicImage, ImageBuffer, Luma};
use tempfile::NamedTempFile;
use regex::Regex;
use tesseract::Tesseract;
use anyhow::{Result, Context};
use std::path::PathBuf;

struct ImageProcessor {
    image_path: PathBuf,
}

impl ImageProcessor {
    fn new(path: PathBuf) -> Self {
        Self { image_path: path }
    }

    fn preprocess_image(&self) -> Result<DynamicImage> {
        // Load the image
        let img = image::open(&self.image_path)
            .with_context(|| format!("Failed to open image at {:?}", self.image_path))?;

        // Convert to grayscale
        let gray = img.grayscale();

        // Increase contrast
        let contrasted = gray.adjust_contrast(2.0);

        // Apply threshold (simple binary threshold)
        let threshold = 128u8;
        let binary = contrasted.to_luma8();
        let binary = ImageBuffer::from_fn(binary.width(), binary.height(), |x, y| {
            if binary.get_pixel(x, y)[0] > threshold {
                Luma([255u8])
            } else {
                Luma([0u8])
            }
        });

        Ok(DynamicImage::ImageLuma8(binary))
    }
}

struct OcrProcessor;

impl OcrProcessor {
    fn new() -> Self {
        Self
    }

    fn process_image(&self, image_path: &std::path::Path) -> Result<String> {
        let image_path_str = image_path.to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path string"))?;

        let mut tess = Tesseract::new(None, Some("eng"))
            .context("Failed to initialize Tesseract")?;

        tess = tess.set_image(image_path_str)
            .context("Failed to set image")?;

        let text = tess.get_text()
            .context("Failed to perform OCR")?;

        Ok(self.clean_text(text))
    }

    fn clean_text(&self, text: String) -> String {
        let re_whitespace = Regex::new(r"\s+").unwrap();
        let text = re_whitespace.replace_all(&text, " ");

        let re_artifacts = Regex::new(r"[|]").unwrap();
        let text = re_artifacts.replace_all(&text, "");

        let re_hyphen = Regex::new(r"(\w+)-\s*\n\s*(\w+)").unwrap();
        let text = re_hyphen.replace_all(&text, "$1$2");

        text.trim().to_string()
    }
}

fn main() -> Result<()> {
    // Create temporary file with .png extension
    let tmp_file = NamedTempFile::new()?;
    let screenshot_path = tmp_file.path().with_extension("png");

    // Capture region with slurp
    let slurp_output = {
        let output = Command::new("slurp")
            .output()
            .context("Failed to run slurp")?;

        if !output.status.success() {
            anyhow::bail!("Slurp failed to capture region");
        }

        String::from_utf8_lossy(&output.stdout).trim().to_string()
    };

    println!("Selected region: {}", slurp_output);

    // Capture screenshot with grim
    let status = Command::new("grim")
        .args(["-g", &slurp_output])
        .arg(&screenshot_path)
        .status()
        .context("Failed to run grim")?;

    if !status.success() {
        anyhow::bail!("Grim failed to capture screenshot");
    }

    // Verify file exists and has size
    if !screenshot_path.exists() {
        anyhow::bail!("Screenshot file was not created");
    }

    let file_size = std::fs::metadata(&screenshot_path)
        .context("Failed to get screenshot file metadata")?
        .len();

    if file_size == 0 {
        anyhow::bail!("Screenshot file is empty");
    }

    println!("Screenshot captured successfully: {:?} ({} bytes)", screenshot_path, file_size);

    // Process the image
    let processor = ImageProcessor::new(screenshot_path.clone());
    let processed_image = processor.preprocess_image()?;

    // Save processed image to temporary file
    let processed_tmp = NamedTempFile::new()?;
    let processed_path = processed_tmp.path().with_extension("png");
    processed_image.save(&processed_path)
        .context("Failed to save processed image")?;

    // Perform OCR
    let ocr = OcrProcessor::new();
    let text = ocr.process_image(&processed_path)?;

    // Print results
    println!("\nExtracted Text:");
    println!("---------------");
    println!("{}", text);

    // Save text to file in home directory
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let output_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(format!("extracted_text_{}.txt", timestamp));

    std::fs::write(&output_path, &text)
        .context("Failed to save extracted text")?;

    println!("\nText saved to: {}", output_path.display());

    Ok(())
}
