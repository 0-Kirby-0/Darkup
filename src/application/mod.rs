use eframe::App;

use crate::textfixer;

mod clipboard;

pub struct Application {
    textfixer: textfixer::Textfixer,
    clipboard: clipboard::ClipboardHandler,
}

impl Application {
    pub fn new() -> Self {
        Self {
            textfixer: textfixer::Textfixer::default(),
            clipboard: clipboard::ClipboardHandler::new(),
        }
    }
}
