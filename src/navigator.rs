use crate::screen::Screen;
use gpui::{AnyView, AppContext, Context, Entity};

/// A navigation stack that manages screen transitions.
///
/// The navigator is generic over the app state type and can be integrated
/// into any GPUI application.
///
/// # Example
///
/// ```rust
/// use gpui_nav::Navigator;
///
/// pub struct AppState {
///     navigator: Navigator,
///     // ... other state
/// }
/// ```
pub struct Navigator {
    stack: Vec<AnyView>,
    history: Vec<&'static str>,
}

impl Navigator {
    /// Creates a new empty navigator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gpui_nav::Navigator;
    ///
    /// let navigator = Navigator::new();
    /// assert!(navigator.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            history: Vec::new(),
        }
    }

    /// Pushes a new screen onto the navigation stack.
    ///
    /// The screen's `on_enter` method will be called if implemented.
    /// This method is generic over the context type, allowing it to work
    /// with any app state.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // In your app state method:
    /// pub fn navigator_push<S: Screen>(&mut self, screen: S, cx: &mut Context<Self>) {
    ///     self.navigator.push(screen, cx);
    /// }
    /// ```
    pub fn push<S: Screen, T: 'static>(&mut self, screen: S, cx: &mut Context<T>) {
        let screen_id = screen.id();
        let entity: Entity<S> = cx.new(|_| screen);
        self.stack.push(entity.into());
        self.history.push(screen_id);
        cx.notify();
    }

    /// Pops the current screen from the stack.
    ///
    /// Returns `true` if a screen was popped, `false` if the stack has only one screen.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if navigator.pop(cx) {
    ///     println!("Navigated back");
    /// } else {
    ///     println!("Already at root screen");
    /// }
    /// ```
    pub fn pop<T: 'static>(&mut self, cx: &mut Context<T>) -> bool {
        if self.stack.len() > 1 {
            self.stack.pop();
            self.history.pop();
            cx.notify();
            true
        } else {
            false
        }
    }

    /// Replaces the current screen with a new one.
    ///
    /// Returns `true` if a screen was replaced, `false` if the stack is empty.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if navigator.replace(LoginScreen::new(ctx), cx) {
    ///     println!("Screen replaced");
    /// } else {
    ///     println!("No screen to replace, stack is empty");
    /// }
    /// ```
    pub fn replace<S: Screen, T: 'static>(&mut self, screen: S, cx: &mut Context<T>) -> bool {
        if self.stack.is_empty() {
            false
        } else {
            self.stack.pop();
            self.history.pop();
            self.push(screen, cx);
            true
        }
    }

    /// Returns a reference to the current screen, if any.
    #[must_use]
    pub fn current(&self) -> Option<&AnyView> {
        self.stack.last()
    }

    /// Returns the navigation history as screen IDs.
    #[must_use]
    pub fn history(&self) -> &[&'static str] {
        &self.history
    }

    /// Clears the entire stack and pushes a new root screen.
    ///
    /// Useful for logout flows or resetting the app state.
    pub fn clear_and_push<S: Screen, T: 'static>(&mut self, screen: S, cx: &mut Context<T>) {
        self.stack.clear();
        self.history.clear();
        self.push(screen, cx);
    }

    /// Returns whether the navigator can go back.
    #[must_use]
    pub fn can_go_back(&self) -> bool {
        self.stack.len() > 1
    }

    /// Returns the number of screens in the stack.
    #[must_use]
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Returns whether the navigation stack is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl Default for Navigator {
    fn default() -> Self {
        Self::new()
    }
}
