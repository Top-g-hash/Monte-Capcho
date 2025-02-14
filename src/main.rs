use iced::highlighter;
use iced::keyboard;
use iced::widget::{
    self, button, column, container, horizontal_space, pick_list, row, text,
    text_editor, toggler, tooltip,
};
use iced::{Center, Element, Fill, Font, Task, Theme};
use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use image::{DynamicImage, ImageBuffer, Luma};
use std::process::Command;
use tempfile::NamedTempFile;
use tesseract::Tesseract;
use anyhow::Result;
use arboard::Clipboard;

pub fn main() -> iced::Result {
    iced::application("MonteCapcho - Text Extractor", Editor::update, Editor::view)
        .theme(Editor::theme)
        .font(include_bytes!("../fonts/ocr-fonts.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .run_with(Editor::new)
}

struct Editor {
    file: Option<PathBuf>,
    content: text_editor::Content,
    theme: highlighter::Theme,
    word_wrap: bool,
    is_loading: bool,
    is_dirty: bool,
    // ocr_text: String,
    status_message: String,
    error_message: String,
}

#[derive(Debug, Clone)]
enum Message {
    ActionPerformed(text_editor::Action),
    ThemeSelected(highlighter::Theme),
    WordWrapToggled(bool),
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
     CaptureAndProcess,
   CopyToClipboard
}

impl Editor {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
               //  ocr_text: String::new(),
            status_message: "Click 'Capture' to start".to_string(),
            error_message: String::new(),
                file: None,
                content: text_editor::Content::new(),
                theme: highlighter::Theme::SolarizedDark,
                word_wrap: true,
                is_loading: false,
                is_dirty: false,
            },

        Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CopyToClipboard => {
                match copy_editor_content(&self.content) {
                    Ok(_) => {
                        self.status_message = "Copied to clipboard".to_string();
                    }
                    Err(e) => {
                        self.error_message = format!("Failed to copy: {}", e);
                    }
                }
                Task::none()
            }

            Message::CaptureAndProcess => {
                self.status_message = "Selecting region...".to_string();
                self.error_message.clear();

                match capture_and_process() {
                    Ok(text) => {
                      self.content = text_editor::Content::with_text(&text);
                    self.status_message = "Text extracted successfully".to_string();                    }
                    Err(e) => {
                        self.error_message = format!("Error: {}", e);
                        self.status_message = "Failed to process".to_string();
                    }
                }
                Task::none() // Return an empty task
            }

            Message::ActionPerformed(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();

                self.content.perform(action);

                Task::none()
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;

                Task::none()
            }
            Message::WordWrapToggled(word_wrap) => {
                self.word_wrap = word_wrap;

                Task::none()
            }
            Message::NewFile => {
                if !self.is_loading {
                    self.file = None;
                    self.content = text_editor::Content::new();
                }

                Task::none()
            }
            Message::OpenFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;

                    Task::perform(open_file(), Message::FileOpened)
                }
            }
            Message::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    self.content = text_editor::Content::with_text(&contents);
                }

                Task::none()
            }
            Message::SaveFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;

                    Task::perform(
                        save_file(self.file.clone(), self.content.text()),
                        Message::FileSaved,
                    )
                }
            }
            Message::FileSaved(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.file = Some(path);
                    self.is_dirty = false;
                }

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // let capture_button = button("Capture")
          //  .on_press(Message::CaptureAndProcess);

            let warning = if !self.error_message.is_empty() {
                text(&self.error_message)
            } else {
                text(&self.status_message)
            };
        let controls = row![
            action(new_icon(), "New file", Some(Message::NewFile)),
            action(extract_icon(), "Capture Text", Some(Message::CaptureAndProcess)),
            action(copy_icon(), "Copy text", Some(Message::CopyToClipboard)),
            action(
                open_icon(),
                "Open file",
                (!self.is_loading).then_some(Message::OpenFile)
            ),
            action(
                save_icon(),
                "Save file",
                self.is_dirty.then_some(Message::SaveFile)
            ),
            horizontal_space(),
            toggler(self.word_wrap)
                .label("Word Wrap")
                .on_toggle(Message::WordWrapToggled),
            pick_list(
                highlighter::Theme::ALL,
                Some(self.theme),
                Message::ThemeSelected
            )
            .text_size(14)
            .padding([5, 10])
        ]
        .spacing(10)
        .align_y(Center);

        let status = row![
            text(if let Some(path) = &self.file {
                let path = path.display().to_string();

                if path.len() > 60 {
                    format!("...{}", &path[path.len() - 40..])
                } else {
                    path
                }
            } else {
                String::from("New file")
            }),
            horizontal_space(),
            text({
                let (line, column) = self.content.cursor_position();

                format!("{}:{}", line + 1, column + 1)
            })
        ]
        .spacing(10);

        column![
            controls,
            text_editor(&self.content)
                .height(Fill)
                .on_action(Message::ActionPerformed)
                .wrapping(if self.word_wrap {
                    text::Wrapping::Word
                } else {
                    text::Wrapping::None
                })
                .highlight(
                    self.file
                        .as_deref()
                        .and_then(Path::extension)
                        .and_then(ffi::OsStr::to_str)
                        .unwrap_or("rs"),
                    self.theme,
                )
                .key_binding(|key_press| {
                    match key_press.key.as_ref() {
                        keyboard::Key::Character("s")
                            if key_press.modifiers.command() =>
                        {
                            Some(text_editor::Binding::Custom(
                                Message::SaveFile,
                            ))
                        }
                        _ => text_editor::Binding::from_key_press(key_press),
                    }
                }),
            warning,
            status,
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    fn theme(&self) -> Theme {
        if self.theme.is_dark() {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(picked_file).await
}

async fn load_file(
    path: impl Into<PathBuf>,
) -> Result<(PathBuf, Arc<String>), Error> {
    let path = path.into();

    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

async fn save_file(
    path: Option<PathBuf>,
    contents: String,
) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}

fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).center_x(30));

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box)
        .into()
    } else {
        action.style(button::secondary).into()
    }
}

fn new_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{F0F6}')
}

fn save_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{E801}')
}

fn open_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{F115}')
}
fn extract_icon <'a, Message>() -> Element<'a, Message> {
    icon('\u{E800}')
}
fn copy_icon <'a, Message>() -> Element<'a, Message> {
    icon('\u{F0C5}')
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("ocr-fonts");

    text(codepoint).font(ICON_FONT).into()
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
fn copy_editor_content(content: &text_editor::Content) -> Result<(), Box<dyn std::error::Error>> {
    let text = content.text();
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text.to_string())?;
    Ok(())
}
