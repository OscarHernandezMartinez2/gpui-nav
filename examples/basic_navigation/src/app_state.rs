use gpui::SharedString;
use gpui_nav::Navigator;

/// Main application state that holds the navigator and shared data
pub struct AppState {
    pub navigator: Navigator,
    pub username: SharedString,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            navigator: Navigator::new(),
            username: SharedString::from(""),
        }
    }

    pub fn login(&mut self, username: String) {
        self.username = SharedString::from(username);
    }

    pub fn logout(&mut self) {
        self.username = SharedString::from("");
    }
}