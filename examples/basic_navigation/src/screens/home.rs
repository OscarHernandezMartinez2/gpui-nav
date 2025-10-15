use gpui::*;
use gpui_nav::{Screen, ScreenContext};

use crate::app_state::AppState;
use crate::screens::{ProfileScreen, SettingsScreen};

/// Home screen - the main entry point of the application
pub struct HomeScreen {
    ctx: ScreenContext<AppState>,
}

impl HomeScreen {
    pub fn new(app_state: WeakEntity<AppState>) -> Self {
        Self {
            ctx: ScreenContext::new(app_state),
        }
    }
}

impl Screen for HomeScreen {
    fn id(&self) -> &'static str {
        "home"
    }
}

impl Render for HomeScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let username = self.ctx.app_state().upgrade()
            .map(|state| state.read(cx).username.clone())
            .unwrap_or_default();

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_8()
                    .bg(rgb(0x2d2d30))
                    .rounded_lg()
                    .shadow_lg()
                    .child(
                        div()
                            .text_2xl()
                            .text_color(rgb(0xcccccc))
                            .child("Home Screen")
                    )
                    .child(self.render_user_status(&username))
                    .child(self.render_navigation_buttons(cx, &username))
            )
    }
}

impl HomeScreen {
    fn render_user_status(&self, username: &str) -> Div {
        if !username.is_empty() {
            div()
                .text_lg()
                .text_color(rgb(0x569cd6))
                .child(format!("Welcome, {}!", username))
        } else {
            div()
                .text_color(rgb(0x969696))
                .child("Not logged in")
        }
    }

    fn render_navigation_buttons(&self, cx: &mut Context<Self>, username: &str) -> Div {
        div()
            .flex()
            .gap_3()
            .mt_4()
            .child(self.profile_button(cx))
            .child(self.settings_button(cx))
            .child(self.auth_button(cx, username))
    }

    fn profile_button(&self, cx: &mut Context<Self>) -> Div {
        create_button("Profile", rgb(0x007acc), rgb(0x005a9e))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                this.ctx.update(cx, |app, inner_cx| {
                    let profile = ProfileScreen::new(inner_cx.weak_entity());
                    app.navigator.push(profile, inner_cx);
                });
            }))
    }

    fn settings_button(&self, cx: &mut Context<Self>) -> Div {
        create_button("Settings", rgb(0x608b4e), rgb(0x4e7a3e))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                this.ctx.update(cx, |app, inner_cx| {
                    let settings = SettingsScreen::new(inner_cx.weak_entity());
                    app.navigator.push(settings, inner_cx);
                });
            }))
    }

    fn auth_button(&self, cx: &mut Context<Self>, username: &str) -> Div {
        if username.is_empty() {
            create_button("Login", rgb(0x569cd6), rgb(0x4178a9))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                    this.ctx.update(cx, |app, inner_cx| {
                        app.login("JohnDoe".to_string());
                        inner_cx.notify();
                    });
                }))
        } else {
            create_button("Logout", rgb(0xd16969), rgb(0xb85450))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                    this.ctx.update(cx, |app, inner_cx| {
                        app.logout();
                        inner_cx.notify();
                    });
                }))
        }
    }
}

fn create_button(label: &str, bg_color: Rgba, hover_color: Rgba) -> Div {
    div()
        .px_4()
        .py_2()
        .bg(bg_color)
        .hover(move |style| style.bg(hover_color))
        .rounded_md()
        .cursor_pointer()
        .child(
            div()
                .text_color(rgb(0xffffff))
                .child(label.to_string())
        )
}