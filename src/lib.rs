//! # gpui-nav
//!
//! A lightweight screen navigation library for GPUI applications.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gpui::*;
//! use gpui_nav::{Navigator, Screen, ScreenContext};
//!
//! // Define your app state
//! pub struct AppState {
//!     navigator: Navigator,
//! }
//!
//! // Define a screen
//! pub struct HomeScreen {
//!     ctx: ScreenContext<AppState>,
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
//! let settings_screen = SettingsScreen::new(ctx.weak_entity());
//! app.navigator.push(settings_screen, cx);
//! ```
//!
//! ### Pop the current screen
//! ```rust,ignore
//! app.navigator.pop(cx);
//! ```
//!
//! ### Replace the current screen
//! ```rust,ignore
//! let login_screen = LoginScreen::new(ctx.weak_entity());
//! app.navigator.replace(login_screen, cx);
//! ```
//!
//! ### Clear stack and push new screen
//! ```rust,ignore
//! let home_screen = HomeScreen::new(ctx.weak_entity());
//! app.navigator.clear_and_push(home_screen, cx);
//! ```
//!
//! ## Examples
//!
//! See the [basic navigation example](https://github.com/benodiwal/gpui-nav/tree/main/examples/basic_navigation)
//! for a complete working demonstration.

pub mod context;
mod navigator;
mod screen;

#[cfg(test)]
mod tests;

pub use context::ScreenContext;
pub use navigator::Navigator;
pub use screen::Screen;

/// Prelude module for convenient imports
///
/// Import everything you need to get started with gpui-nav:
///
/// ```rust
/// use gpui_nav::prelude::*;
///
/// // Now you have access to Navigator, Screen, and ScreenContext
/// ```
pub mod prelude {
    //! Convenient re-exports of commonly used gpui-nav types

    pub use crate::{Navigator, Screen, ScreenContext};
}
