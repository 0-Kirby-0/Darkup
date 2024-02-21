use clipboard::ClipboardProvider;

pub enum ClipboardHandler {
    Desktop(clipboard::ClipboardContext),
    Web(),
}

impl ClipboardHandler {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        Self::Desktop(clipboard::ClipboardProvider::new().unwrap())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn get_clipboard(&mut self) -> String {
        match self {
            Self::Desktop(context) => context.get_contents().unwrap(),
            Self::Web() => unimplemented!(),
        }
    }

    pub fn set_clipboard(&mut self, content: &str) {
        match self {
            Self::Desktop(context) => context.set_contents(content.to_owned()).unwrap(),
            Self::Web() => unimplemented!(),
        }
    }
}
