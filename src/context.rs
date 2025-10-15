use gpui::{Context, Render, WeakEntity};

/// A helper context that provides convenient navigation methods to screens.
///
/// This struct should be stored in each screen and provides methods to navigate
/// without directly accessing the app state.
///
/// # Type Parameter
///
/// `T` - Your app state type (e.g., `AppState`)
///
/// # Example
///
/// ```rust,ignore
/// use gpui_nav::ScreenContext;
///
/// pub struct MyScreen {
///     ctx: ScreenContext<AppState>,
///     // other fields...
/// }
///
/// impl MyScreen {
///     pub fn new(app_state: WeakEntity<AppState>) -> Self {
///         Self {
///             ctx: ScreenContext::new(app_state),
///         }
///     }
/// }
/// ```
pub struct ScreenContext<T> {
    app_state: WeakEntity<T>,
}

impl<T> ScreenContext<T>
where
    T: 'static,
{
    /// Creates a new screen context with a reference to the app state.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let ctx = ScreenContext::new(cx.weak_entity());
    /// ```
    #[must_use]
    pub fn new(app_state: WeakEntity<T>) -> Self {
        Self { app_state }
    }

    /// Returns a clone of the app state weak entity.
    ///
    /// Use this when creating new screens that need navigation capabilities.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let next_screen = NextScreen::new(self.ctx.app_state());
    /// ```
    #[must_use]
    pub fn app_state(&self) -> WeakEntity<T> {
        self.app_state.clone()
    }

    /// Helper method to update the app state.
    ///
    /// This is a convenience method that handles the `Result` from `update()`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// self.ctx.update(cx, |app, inner_cx| {
    ///     app.navigator().push(NextScreen::new(self.ctx.app_state()), inner_cx);
    /// });
    /// ```
    pub fn update<R>(
        &self,
        cx: &mut Context<impl Render>,
        f: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> Option<R> {
        self.app_state.update(cx, f).ok()
    }
}

impl<T> Clone for ScreenContext<T> {
    fn clone(&self) -> Self {
        Self {
            app_state: self.app_state.clone(),
        }
    }
}
