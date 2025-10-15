# gpui-nav

A lightweight screen navigation library for GPUI applications.

[![Crates.io](https://img.shields.io/crates/v/gpui-nav.svg)](https://crates.io/crates/gpui-nav)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
gpui = "0.2.1"
gpui-nav = "0.1.1"
```

## Basic Usage

```rust
use gpui::*;
use gpui_nav::{Navigator, Screen, ScreenContext};

// Define your app state
pub struct AppState {
    navigator: Navigator,
}

// Define a screen
pub struct HomeScreen {
    ctx: ScreenContext<AppState>,
}

impl Screen for HomeScreen {
    fn id(&self) -> &'static str {
        "home"
    }
}

impl Render for HomeScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child("Home Screen")
            .child(
                div()
                    .child("Go to Settings")
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.ctx.update(cx, |app, inner_cx| {
                            let settings = SettingsScreen::new(inner_cx.weak_entity());
                            app.navigator.push(settings, inner_cx);
                        });
                    }))
            )
    }
}
```

## Navigation Operations

### Push a new screen
```rust
let settings_screen = SettingsScreen::new(ctx.weak_entity());
app.navigator.push(settings_screen, cx);
```

### Pop the current screen
```rust
app.navigator.pop(cx);
```

### Replace the current screen
```rust
let login_screen = LoginScreen::new(ctx.weak_entity());
app.navigator.replace(login_screen, cx);
```

### Clear stack and push new screen
```rust
let home_screen = HomeScreen::new(ctx.weak_entity());
app.navigator.clear_and_push(home_screen, cx);
```

## Examples

### Basic Navigation Example

A complete example demonstrating navigation between multiple screens with state management:

```bash
cd examples/basic_navigation
cargo run
```

**Features demonstrated:**
- Multiple screens (Home, Profile, Settings)
- All navigation operations (push, pop, replace, clear_and_push)
- Shared state management
- Login/logout flow
- Clean modular architecture

👉 **[View the complete example](examples/basic_navigation/)**

## Core Concepts

### Screen Trait

Every screen must implement the `Screen` trait:

```rust
pub trait Screen {
    fn id(&self) -> &'static str;
}
```

### ScreenContext

`ScreenContext` provides convenient navigation methods:

```rust
pub struct ScreenContext<T> {
    // Provides access to app state and navigation
}

impl<T> ScreenContext<T> {
    pub fn new(app_state: WeakEntity<T>) -> Self;
    pub fn app_state(&self) -> WeakEntity<T>;
    pub fn update<R>(&self, cx: &mut Context<impl Render>, f: impl FnOnce(&mut T, &mut Context<T>) -> R) -> Option<R>;
}
```

### Navigator

The `Navigator` manages your navigation stack:

```rust
impl Navigator {
    pub fn new() -> Self;
    pub fn push<S: Screen, T: 'static>(&mut self, screen: S, cx: &mut Context<T>);
    pub fn pop<T: 'static>(&mut self, cx: &mut Context<T>) -> bool;
    pub fn replace<S: Screen, T: 'static>(&mut self, screen: S, cx: &mut Context<T>) -> bool;
    pub fn clear_and_push<S: Screen, T: 'static>(&mut self, screen: S, cx: &mut Context<T>);
    pub fn current(&self) -> Option<&AnyView>;
    pub fn can_go_back(&self) -> bool;
    pub fn len(&self) -> usize;
}
```

## Architecture

```
Your App State
    ├── Navigator (manages screen stack)
    ├── Shared Data (accessible to all screens)
    └── Business Logic

Screen A ←→ ScreenContext ←→ Navigator ←→ Screen B
    ↓              ↓                         ↓
  UI Logic    Navigation API            UI Logic
```

## Best Practices

1. **Single Navigator**: Keep one navigator instance in your app state
2. **Screen IDs**: Use descriptive, unique screen identifiers
3. **State Management**: Store shared data in your app state, not in screens
4. **Memory**: Screens are automatically cleaned up when popped

## Compatibility

- **GPUI**: 0.2+
- **Rust**: 1.70+

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
