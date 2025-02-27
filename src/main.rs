use iced::highlighter;
use iced::widget::{
    button, column, container, horizontal_space,  row, text,
    text_editor , tooltip,
};
use iced::{Center, Element, Fill, Font, Task, Theme};
use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use iced::Subscription;
use iced::keyboard;
use clap::Parser;
use iced::window;
use iced::window::Position;
use std::env;
use std::process:: Stdio;
use std::error::Error;
use std::process::Command;

mod cli;
mod ocr;
mod icon;

pub fn main() -> iced::Result {
    let cli = cli::Cli::parse();
    let mut initial_text: Option<String> = None;
        let mut trigger_copy = false;
    if cli.capture {
        println!("Performing OCR capture...");
        match ocr::capture_and_process() {
            Ok(text) => {
                println!("Extracted text:\n{}", text);
                initial_text = Some(text.clone());
               if cli.copy {
                    // Instead of calling copy_text_to_clipboard here,
                    // we let the GUI trigger Message::CopyToClipboard.
                    trigger_copy = true;
                }
            }
            Err(e) => {
                eprintln!("Error during OCR capture: {}", e);
            }
        }
    }
 if let Err(e) = start_copyq() {
        eprintln!("Warning: failed to start CopyQ: {}", e);
    } else {
        println!("CopyQ started successfully.");
    }
    iced_fontello::build("fonts/ocr-icons.toml").expect("Build ocr-icons font");
    iced::application("MonteCapcho - Text Extractor", Editor::update, Editor::view)
            .centered()
        .window(window::Settings{position:Position::Centered,..Default::default()})
        .subscription(Editor::subscription)
        .theme(Editor::theme)
        .font(icon::FONT)
        .default_font(Font::MONOSPACE)
        .run_with(move|| Editor::new_with_text(initial_text, trigger_copy))
}

struct Editor {
    file: Option<PathBuf>,
    content: text_editor::Content,
    theme: highlighter::Theme,
    is_loading: bool,
    is_dirty: bool,
    status_message: String,
    error_message: String,
}

#[derive(Debug, Clone)]
enum Message {
    ActionPerformed(text_editor::Action),
    ThemeSelected(highlighter::Theme),
    NewFile,
     CaptureAndProcess,
   CopyToClipboard,
}

impl Editor {
    // New initializer that accepts an optional captured text.
    fn new_with_text(initial_text: Option<String>, trigger_copy: bool) -> (Self, Task<Message>) {
        let content = if let Some(text) = initial_text {
            text_editor::Content::with_text(&text)
        } else {
            text_editor::Content::new()
        };
        let task = if trigger_copy {
            // The async block here doesn't need to do any work; it's just used to schedule the message.
            Task::perform(async { () }, |_| Message::CopyToClipboard)
        } else {
            Task::none()
        };

        (
            Self {
                status_message: "Click 'Capture' to start".to_string(),
                error_message: String::new(),
                file: None,
                content,
                theme: highlighter::Theme::SolarizedDark,
                is_loading: false,
                is_dirty: false,
            },
             task,
        )
    }
    fn new() -> (Self, Task<Message>) {
        (
            Self {
            status_message: "Click 'Capture' to start".to_string(),
            error_message: String::new(),
                file: None,
                content: text_editor::Content::new(),
                theme: highlighter::Theme::SolarizedDark,
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

                match ocr::capture_and_process() {
                    Ok(text) => {
                      self.content = text_editor::Content::with_text(&text);
                    self.status_message = "Text extracted successfully".to_string();
                    }
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

            Message::NewFile => {
                if !self.is_loading {
                    self.file = None;
                    self.content = text_editor::Content::new();
                }

                Task::none()
            }



        }
    }

    fn view(&self) -> Element<Message> {
            let status = if !self.error_message.is_empty() {
                text(&self.error_message)
            } else {
                text(&self.status_message)
            };
        let controls = row![
            action(icon::clear(), "Clear Text ", Some(Message::NewFile)),
            action(icon::capture(), "Capture Text (Ctrl + S)", Some(Message::CaptureAndProcess)),
            action(icon::copy(), "Copy Text (Ctrl+C)", Some(Message::CopyToClipboard)),
                        horizontal_space(),
            status,


        ]
        .spacing(10)
        .align_y(Center);



        column![
            controls,
            text_editor(&self.content)
                .height(Fill)
                .on_action(Message::ActionPerformed)
                .wrapping(
                    text::Wrapping::Word
                 )
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
                        keyboard::Key::Character("c")
                            if key_press.modifiers.control() =>
                        {
                            Some(text_editor::Binding::Custom(
                                Message::CopyToClipboard,
                            ))
                        }
                        _ => text_editor::Binding::from_key_press(key_press),
                    }
                }),

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
 fn subscription(&self) -> Subscription<Message> {
    keyboard::on_key_press(|key, modifiers| {
        println!("Key: {:?}, Modifiers: {:?}", key, modifiers); // Debug output

        if modifiers.control() {
            match key {
                keyboard::Key::Character(ch) => {
                    match ch.to_lowercase().as_str() {
                        "c" => Some(Message::CopyToClipboard),
                        "s" => Some(Message::CaptureAndProcess),
                        _ => None,
                    }
                }

                _ => None,
            }
        } else {
            None
        }
    })
}}

#[derive(Debug, Clone)]
pub enum OtherError {
    DialogClosed,
    IoError(io::ErrorKind),
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



fn copy_text_using_copyq(text: &str) -> Result<(), Box<dyn Error>> {
    // Call the copyq command to store the text persistently.
    Command::new("copyq")
        .args(&["copy", text])
        .spawn()?
        .wait()?;
    Ok(())
}

// Example usage:
fn copy_editor_content(content: &text_editor::Content) -> Result<(), Box<dyn Error>> {
    let text = content.text();

    // Option 1: Rely solely on CopyQ:
    copy_text_using_copyq(&text)?;

    // Option 2: Use both the native API and CopyQ:
    // Clipboard::new()?.set_text(text.to_string())?;
    // copy_text_using_copyq(text)?;

    Ok(())
}
fn start_copyq() -> Result<(), Box<dyn Error>> {
    Command::new("sh")
        .arg("-c")
        .arg("pgrep copyq || copyq --start-server && copyq hide")
        .envs(env::vars())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}
