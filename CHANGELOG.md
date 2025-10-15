# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2025-10-15

### Added
- Initial release of gpui-nav
- Core navigation system with `Navigator` struct
- Screen management with `Screen` trait
- Context helper with `ScreenContext` for convenient navigation
- Navigation operations:
  - `push()` - Navigate to a new screen
  - `pop()` - Go back to the previous screen
  - `replace()` - Replace the current screen
  - `clear_and_push()` - Reset navigation stack and push new screen
- Navigation history tracking with screen IDs
- Stack management with utilities:
  - `current()` - Get the current active screen
  - `can_go_back()` - Check if back navigation is possible
  - `len()` - Get the number of screens in stack
  - `is_empty()` - Check if navigation stack is empty
- Type-safe screen transitions using GPUI's `AnyView`
- Comprehensive documentation with usage examples
- Complete working example demonstrating:
  - Multi-screen navigation (Home, Profile, Settings)
  - State management across screens
  - Login/logout flow
  - All navigation operations
  - Clean modular architecture
- Prelude module for convenient imports
- Full GPUI 0.2+ compatibility
- MIT license

### Documentation
- Comprehensive README with quick start guide
- API documentation with code examples
- Architecture overview and best practices
- Complete working example with modular structure
- Inline code documentation for all public APIs

### Example
- `basic_navigation` - Complete demonstration app showing:
  - Navigation between multiple screens
  - Shared state management with login/logout
  - All four navigation operations in action
  - Clean separation of concerns with modules
  - Professional UI with dark theme

[Unreleased]: https://github.com/benodiwal/gpui-nav/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/benodiwal/gpui-nav/releases/tag/v0.1.0