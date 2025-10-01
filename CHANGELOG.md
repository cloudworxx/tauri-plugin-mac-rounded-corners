# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Example Tauri app demonstrating the plugin
- Video tutorial
- Support for multiple window instances

## [1.0.0] - 2025-01-XX

### Added
- Initial release
- `enable_rounded_corners()` function for basic rounded corners
- `enable_modern_window_style()` function with shadow and layer-based clipping
- Customizable Traffic Lights positioning via `offsetX` and `offsetY` parameters
- Customizable corner radius via `cornerRadius` parameter
- TypeScript type definitions
- Full documentation and examples
- App Store compatible implementation using only public APIs
- Support for Tauri v2
- MIT License

### Features
- Native macOS rounded corners (12px default)
- Traffic Lights (Close, Minimize, Maximize) positioning
- Optional shadow effects
- Transparent titlebar
- Full-size content view
- Works with `decorations: false`

[Unreleased]: https://github.com/cloudworxx/tauri-plugin-mac-rounded-corners/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/cloudworxx/tauri-plugin-mac-rounded-corners/releases/tag/v1.0.0
