use gpui::{Context, Render};

/// A screen that can be displayed in the navigation stack.
///
/// Screens must implement both `Render` and this trait to be used with the navigator.
///
/// # Example
///
/// ```rust,ignore
/// use gpui::*;
/// use gpui_nav::Screen;
///
/// pub struct MyScreen {
///     // your fields
/// }
///
/// impl Screen for MyScreen {
///     fn id(&self) -> &'static str {
///         "my_screen"
///     }
/// }
///
/// impl Render for MyScreen {
///     fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
///         div().child("My Screen")
///     }
/// }
/// ```
pub trait Screen: Render + 'static {
    /// Returns a unique identifier for this screen.
    ///
    /// This ID is used for history tracking and debugging.
    fn id(&self) -> &'static str;

    /// Called when the screen is pushed onto the navigation stack.
    ///
    /// Use this to initialize state or start timers/animations.
    fn on_enter(&mut self, _cx: &mut Context<Self>)
    where
        Self: Sized,
    {
    }

    /// Called when the screen is popped from the navigation stack.
    ///
    /// Use this to clean up resources or cancel ongoing operations.
    fn on_exit(&mut self, _cx: &mut Context<Self>)
    where
        Self: Sized,
    {
    }
}
