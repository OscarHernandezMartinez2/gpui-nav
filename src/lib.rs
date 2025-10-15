//! # gpui-nav
//!
//! A lightweight screen navigation library for GPUI applications.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gpui::*;
//! use gpui_navigator::{Navigator, Screen, ScreenContext};
//!
//! // Define your app state
//! pub struct AppState {
//!     navigator: Navigator,
//! }
//!
//! // Define a screen
//! pub struct HomeScreen {
//!     ctx: ScreenContext,
//! }
//!
//! impl Screen for HomeScreen {
//!     fn id(&self) -> &'static str {
//!         "home"
//!     }
//! }
//!
//! impl Render for HomeScreen {
//!     fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
//!         div().child("Home Screen")
//!     }
//! }
//! ```
//!
//! ## Navigation Operations
//!
//! ### Push a new screen
//! ```rust,ignore
//! self.ctx.push(SettingsScreen::new(self.ctx.app_state()), cx);
//! ```
//!
//! ### Pop the current screen
//! ```rust,ignore
//! self.ctx.pop(cx);
//! ```
//!
//! ### Replace the current screen
//! ```rust,ignore
//! self.ctx.replace(LoginScreen::new(self.ctx.app_state()), cx);
//! ```
//!
//! ### Clear stack and push new screen
//! ```rust,ignore
//! self.ctx.clear_and_push(HomeScreen::new(self.ctx.app_state()), cx);

mod navigator;
pub mod context;
mod screen;

pub use context::ScreenContext;
pub use navigator::Navigator;
pub use screen::Screen;

pub mod prelude {
    pub use crate::ScreenContext;
}