// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/ocr-icons.toml
// 060e23ecdca33b83634d8d734e7bf4675752e5b5d245cd383246aa82f0203e5e
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../fonts/ocr-icons.ttf");

pub fn capture<'a>() -> Text<'a> {
    icon("\u{1F4F7}")
}

pub fn clear<'a>() -> Text<'a> {
    icon("\u{E760}")
}

pub fn copy<'a>() -> Text<'a> {
    icon("\u{F0C5}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("ocr-icons"))
}
