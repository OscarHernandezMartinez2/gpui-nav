use gpui::*;
use gpui_nav::{Screen, ScreenContext};

use crate::app_state::AppState;

/// Settings screen - displays application settings and configuration options
pub struct SettingsScreen {
    ctx: ScreenContext<AppState>,
}

impl SettingsScreen {
    pub fn new(app_state: WeakEntity<AppState>) -> Self {
        Self {
            ctx: ScreenContext::new(app_state),
        }
    }
}

impl Screen for SettingsScreen {
    fn id(&self) -> &'static str {
        "settings"
    }
}

impl Render for SettingsScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(self.render_header(cx))
            .child(self.render_settings_list())
    }
}

impl SettingsScreen {
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
                    .child("Settings")
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

    fn render_settings_list(&self) -> Div {
        div()
            .flex()
            .flex_col()
            .p_4()
            .gap_2()
            .child(self.setting_item("Dark Mode", "ON", true))
            .child(self.setting_item("Notifications", "OFF", false))
    }

    fn setting_item(&self, label: &str, value: &str, is_enabled: bool) -> Div {
        let value_color = if is_enabled {
            rgb(0x569cd6) // Blue
        } else {
            rgb(0x969696) // Gray
        };

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .py_3()
            .bg(rgb(0x2d2d30))
            .rounded_md()
            .child(
                div()
                    .text_color(rgb(0xcccccc))
                    .child(label.to_string())
            )
            .child(
                div()
                    .text_color(value_color)
                    .child(value.to_string())
            )
    }
}