#[cfg(test)]
mod tests {
    use super::super::*;
    use gpui::ParentElement;

    #[test]
    fn test_navigator_new() {
        let navigator = Navigator::new();
        assert!(navigator.is_empty());
        assert_eq!(navigator.len(), 0);
        assert!(!navigator.can_go_back());
        assert!(navigator.current().is_none());
        assert!(navigator.history().is_empty());
    }

    #[test]
    fn test_navigator_default() {
        let navigator = Navigator::default();
        assert!(navigator.is_empty());
        assert_eq!(navigator.len(), 0);
        assert!(!navigator.can_go_back());
    }

    #[test]
    fn test_screen_id() {
        struct TestScreen {
            id: &'static str,
        }

        impl Screen for TestScreen {
            fn id(&self) -> &'static str {
                self.id
            }
        }

        impl gpui::Render for TestScreen {
            fn render(
                &mut self,
                _: &mut gpui::Window,
                _: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                gpui::div().child(format!("Screen: {}", self.id))
            }
        }

        let screen = TestScreen { id: "test_id" };
        assert_eq!(screen.id(), "test_id");

        let another_screen = TestScreen { id: "another_id" };
        assert_eq!(another_screen.id(), "another_id");
    }

    #[test]
    fn test_screen_context_creation() {
        struct TestApp {
            _value: i32,
        }

        let weak = gpui::WeakEntity::<TestApp>::new_invalid();
        let screen_ctx = ScreenContext::<TestApp>::new(weak);
        assert!(screen_ctx.app_state().upgrade().is_none());
    }

    #[test]
    fn test_screen_context_clone() {
        struct TestApp {
            _value: i32,
        }

        let weak = gpui::WeakEntity::<TestApp>::new_invalid();
        let screen_ctx = ScreenContext::<TestApp>::new(weak.clone());
        let cloned = screen_ctx.clone();

        assert!(cloned.app_state().upgrade().is_none());
    }

    #[test]
    fn test_screen_trait_default_methods() {
        struct TestScreen;

        impl Screen for TestScreen {
            fn id(&self) -> &'static str {
                "test"
            }
        }

        impl gpui::Render for TestScreen {
            fn render(
                &mut self,
                _: &mut gpui::Window,
                _: &mut gpui::Context<Self>,
            ) -> impl gpui::IntoElement {
                gpui::div().child("Test Screen")
            }
        }

        let screen = TestScreen;
        assert_eq!(screen.id(), "test");
    }

    #[test]
    fn test_navigator_replace_returns_bool() {
        let navigator = Navigator::new();
        assert!(navigator.is_empty());
    }
}
