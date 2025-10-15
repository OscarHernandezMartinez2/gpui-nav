# Basic Navigation Example

This example demonstrates how to use gpui-nav for screen navigation in a GPUI application.

## Features Demonstrated

- **Multiple Screens**: Home, Profile, and Settings screens
- **Navigation Operations**:
  - Push new screens onto the navigation stack
  - Pop to go back to the previous screen
  - Clear and push to reset the navigation stack
  - Replace the current screen
- **State Management**: Shared app state accessible across screens
- **User Authentication**: Simple login/logout flow with username persistence

## Running the Example

```bash
cargo run
```

## Navigation Flow

1. **Home Screen** - The main entry point with buttons to:
   - Navigate to Profile
   - Navigate to Settings
   - Login/Logout to change user state

2. **Profile Screen** - Shows user information with options to:
   - Go back to the previous screen
   - Go directly to Home (clearing the navigation stack)

3. **Settings Screen** - Displays app settings with:
   - Back navigation
   - Settings display (Dark Mode, Notifications)

## Code Structure

- `AppState` - Holds the navigator instance and shared state (username)
- `HomeScreen`, `ProfileScreen`, `SettingsScreen` - Individual screen implementations
- `ScreenContext` - Provides convenient navigation methods to screens
- `Navigator` - Manages the navigation stack and screen transitions

## Key Concepts

### Creating a Screen

```rust
pub struct MyScreen {
    ctx: ScreenContext<AppState>,
}

impl Screen for MyScreen {
    fn id(&self) -> &'static str {
        "my_screen"
    }
}
```

### Navigation Operations

```rust
// Push a new screen
app.navigator.push(new_screen, cx);

// Go back
app.navigator.pop(cx);

// Replace current screen
app.navigator.replace(new_screen, cx);

// Clear stack and push
app.navigator.clear_and_push(home_screen, cx);
```