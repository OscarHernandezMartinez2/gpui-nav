use gpui::*;
use gpui_nav::{Screen, ScreenContext};

use crate::app_state::AppState;
use crate::screens::home::HomeScreen;

/// Profile screen - displays user information and profile options
pub struct ProfileScreen {
    ctx: ScreenContext<AppState>,
}

impl ProfileScreen {
    pub fn new(app_state: WeakEntity<AppState>) -> Self {
        Self {
            ctx: ScreenContext::new(app_state),
        }
    }
}

impl Screen for ProfileScreen {
    fn id(&self) -> &'static str {
        "profile"
    }
}

impl Render for ProfileScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let username = self.ctx.app_state().upgrade()
            .map(|state| state.read(cx).username.clone())
            .unwrap_or_default();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(self.render_header(cx))
            .child(self.render_content(cx, &username))
    }
}

impl ProfileScreen {
    fn render_header(&self, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .items_center()
            .px_4()
            .py_3()
            .bg(rgb(0x2d2d30))
            .border_b_1()
            .border_color(rgb(0x3e3e42))
            .child(self.back_button(cx))
            .child(
                div()
                    .flex_1()
                    .text_center()
                    .text_xl()
                    .text_color(rgb(0xcccccc))
                    .child("Profile")
            )
    }

    fn back_button(&self, cx: &mut Context<Self>) -> Div {
        div()
            .px_3()
            .py_1()
            .bg(rgb(0x3e3e42))
            .hover(|style| style.bg(rgb(0x4e4e52)))
            .rounded_md()
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                this.ctx.update(cx, |app, inner_cx| {
                    app.navigator.pop(inner_cx);
                });
            }))
            .child(
                div()
                    .text_color(rgb(0xcccccc))
                    .child("← Back")
            )
    }

    fn render_content(&self, cx: &mut Context<Self>, username: &str) -> Div {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .flex_1()
            .gap_4()
            .p_8()
            .child(self.render_user_info(username))
            .child(self.render_user_details())
            .child(self.home_button(cx))
    }

    fn render_user_info(&self, username: &str) -> Div {
        div()
            .text_2xl()
            .text_color(rgb(0xcccccc))
            .child(if username.is_empty() {
                "Guest User".to_string()
            } else {
                username.to_string()
            })
    }

    fn render_user_details(&self) -> Div {
        div()
            .text_color(rgb(0x9cdcfe))
            .child("user@example.com")
    }

    fn home_button(&self, cx: &mut Context<Self>) -> Div {
        div()
            .px_4()
            .py_2()
            .bg(rgb(0x608b4e))
            .hover(|style| style.bg(rgb(0x4e7a3e)))
            .rounded_md()
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                this.ctx.update(cx, |app, inner_cx| {
                    let home = HomeScreen::new(inner_cx.weak_entity());
                    app.navigator.clear_and_push(home, inner_cx);
                });
            }))
            .child(
                div()
                    .text_color(rgb(0xffffff))
                    .child("Go Home")
            )
    }
}