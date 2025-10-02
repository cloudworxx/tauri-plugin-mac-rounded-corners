# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Example Tauri app demonstrating the plugin
- Video tutorial
- Support for multiple window instances

## [1.1.0] - 2025-01-02

### Added
- **Auto-reposition feature**: Traffic Lights now automatically reposition after exiting fullscreen
- New `reposition_traffic_lights()` command for manual repositioning
- Event-based resize listener using `onResized` for instant repositioning
- `cleanupRoundedCorners()` function for proper cleanup on component unmount

### Changed
- Improved Traffic Lights positioning to be instant (event-based instead of polling)
- Updated TypeScript wrapper with automatic fullscreen monitoring
- Enhanced documentation with Tauri permissions requirements

### Fixed
- **Critical**: Traffic Lights no longer stuck at wrong position after fullscreen exit
- **Critical**: Close button now works correctly (added missing `NSClosableWindowMask`)
- Permission errors when using `isFullscreen()` and window events

### Documentation
- Added required Tauri permissions to README (`core:window:allow-is-fullscreen`, etc.)
- Updated integration guide with permission setup
- Added troubleshooting section for fullscreen issues
- Clarified that `transparent: true` is NOT needed in tauri.conf.json

## [1.0.0] - 2025-01-02

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

[Unreleased]: https://github.com/cloudworxx/tauri-plugin-mac-rounded-corners/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/cloudworxx/tauri-plugin-mac-rounded-corners/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/cloudworxx/tauri-plugin-mac-rounded-corners/releases/tag/v1.0.0
