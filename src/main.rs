use iced::widget::{button, column, container, scrollable, text, Text};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use iced::window::Settings as WindowSettings;
use image::{DynamicImage, ImageBuffer, Luma};
use std::process::Command;
use tempfile::NamedTempFile;
use tesseract::Tesseract;
use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() -> iced::Result {
    ScreenshotOcr::run(Settings {
        window: WindowSettings {
            size: (500, 400),
            ..Default::default()
        },
        ..Default::default()
    })
}

struct ScreenshotOcr {
    ocr_text: String,
    status_message: String,
    error_message: String,
}

#[derive(Debug, Clone)]
enum Message {
    CaptureAndProcess,
    CopyToClipboard,
}

impl Sandbox for ScreenshotOcr {
    type Message = Message;

    fn new() -> Self {
        Self {
            ocr_text: String::new(),
            status_message: "Click 'Capture' to start".to_string(),
            error_message: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Screenshot OCR")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CaptureAndProcess => {
                self.status_message = "Selecting region...".to_string();
                self.error_message.clear();

                match capture_and_process() {
                    Ok(text) => {
                        self.ocr_text = text;
                        self.status_message = "Text extracted successfully".to_string();
                    }
                    Err(e) => {
                        self.error_message = format!("Error: {}", e);
                        self.status_message = "Failed to process".to_string();
                    }
                }
            }
            Message::CopyToClipboard => {
                if let Ok(mut ctx) = ClipboardContext::new() {
                    if ctx.set_contents(self.ocr_text.clone()).is_ok() {
                        self.status_message = "Copied to clipboard".to_string();
                    } else {
                        self.error_message = "Failed to copy to clipboard".to_string();
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let capture_button = button("Capture")
            .on_press(Message::CaptureAndProcess);

        let copy_button = button("Copy to Clipboard")
            .on_press(Message::CopyToClipboard);

        let status = if !self.error_message.is_empty() {
            text(&self.error_message).style(iced::Color::from_rgb(0.8, 0.0, 0.0))
        } else {
            text(&self.status_message)
        };

        let content = column![
            status,
            capture_button,
            scrollable(
                text(&self.ocr_text)
                    .width(Length::Fill)
            ).height(Length::Fill),
            copy_button,
        ]
        .padding(20)
        .spacing(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn capture_and_process() -> Result<String> {
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
