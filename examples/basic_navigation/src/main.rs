use gpui::*;

mod app_state;
mod screens;

use app_state::AppState;
use screens::HomeScreen;

/// Main view that holds the app state and renders the current screen
struct NavigationExample {
    app_state: Entity<AppState>,
}

impl NavigationExample {
    fn new(cx: &mut Context<Self>) -> Self {
        let app_state = cx.new(|cx| {
            let mut state = AppState::new();
            // Push initial home screen
            let home = HomeScreen::new(cx.weak_entity());
            state.navigator.push(home, cx);
            state
        });

        Self { app_state }
    }
}

impl Render for NavigationExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(current_view) = self.app_state.read(cx).navigator.current() {
            current_view.clone()
        } else {
            AnyView::from(cx.new(|_| EmptyView))
        }
    }
}

/// Empty view for fallback when no screen is active
struct EmptyView;

impl Render for EmptyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(
                div()
                    .text_color(rgb(0xcccccc))
                    .child("No active screen")
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Navigation Example".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(800.0), px(600.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| {
                cx.new(NavigationExample::new)
            },
        )
        .unwrap();
    });
}