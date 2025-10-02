# macOS Rounded Corners Plugin for Tauri v2

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue)](https://tauri.app/)
[![macOS](https://img.shields.io/badge/macOS-compatible-green)](https://www.apple.com/macos/)
[![Version](https://img.shields.io/badge/version-1.1.0-blue)](https://github.com/cloudworxx/tauri-plugin-mac-rounded-corners)

A reusable Tauri plugin that enables native macOS rounded corners with customizable Traffic Lights positioning for frameless windows.

## Author

**Sascha Klein**  
Website: [cloudworxx.us](https://cloudworxx.us)  
Email: [hello@cloudworxx.us](mailto:hello@cloudworxx.us)

## Features

- ✅ **Rounded corners** (12px default, customizable)
- ✅ **Traffic Lights positioning** (Close, Minimize, Maximize buttons)
- ✅ **Auto-reposition after fullscreen** (NEW in v1.1.0)
- ✅ **App Store compatible** - Uses only public macOS APIs
- ✅ **Custom titlebar support** - Full-size content view
- ✅ **Native shadows** - Optional shadow effects
- ✅ **TypeScript support** - Full type definitions
- ✅ **Works with `decorations: false`**
- ✅ **Zero runtime dependencies**
- ✅ **MIT Licensed** - Free for personal and commercial use

## What's New in v1.1.0

- **Auto-reposition**: Traffic Lights automatically reposition after fullscreen exit
- **New command**: `reposition_traffic_lights()` for manual control
- **Event-based**: Uses `onResized` listener for instant repositioning
- **Bug fixes**: Close button now works correctly, fullscreen positioning fixed

## Installation

There are two ways to install this plugin: **automatic** (recommended) or **manual**.

### Option A: Automatic Installation (Recommended)

The plugin includes an automatic installer that copies files and updates your project configuration.

#### 1. Install the package

```bash
npm install @cloudworxx/tauri-plugin-mac-rounded-corners
```

The `postinstall` script automatically:
- ✅ Copies `mod.rs` to `src-tauri/src/plugins/mac_rounded_corners.rs`
- ✅ Creates/updates `src-tauri/src/plugins/mod.rs`

#### 2. Complete the setup

You still need to manually add:

**a) Dependencies to `src-tauri/Cargo.toml`:**
```toml
[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.26"
objc = "0.2.7"
```

**b) Commands in `src-tauri/src/lib.rs`:**
```rust
mod plugins;
use plugins::mac_rounded_corners;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            mac_rounded_corners::enable_rounded_corners,
            mac_rounded_corners::enable_modern_window_style,
            mac_rounded_corners::reposition_traffic_lights
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**c) Permissions to `tauri.conf.json`:**
```json
{
  "app": {
    "security": {
      "capabilities": [
        {
          "identifier": "main-capability",
          "windows": ["*"],
          "permissions": [
            "core:window:allow-start-dragging",
            "core:window:allow-is-fullscreen",
            "core:window:allow-is-maximized",
            "core:event:allow-listen"
          ]
        }
      ]
    }
  }
}
```

### Option B: Manual Installation

#### 1. Copy Rust Code

```bash
mkdir -p src-tauri/src/plugins
cp mod.rs src-tauri/src/plugins/mac_rounded_corners.rs
```

#### 2. Add Dependencies

Add to `src-tauri/Cargo.toml`:

```toml
[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.26"
objc = "0.2.7"
```

#### 3. Register Commands

In `src-tauri/src/lib.rs`:

```rust
mod plugins;
use plugins::mac_rounded_corners;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            mac_rounded_corners::enable_rounded_corners,
            mac_rounded_corners::enable_modern_window_style,
            mac_rounded_corners::reposition_traffic_lights  // NEW in v1.1.0
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Create `src-tauri/src/plugins/mod.rs`:

```rust
pub mod mac_rounded_corners;
```

#### 4. Add Required Permissions

**IMPORTANT**: Add these permissions to `tauri.conf.json`:

```json
{
  "app": {
    "security": {
      "capabilities": [
        {
          "identifier": "main-capability",
          "windows": ["*"],
          "permissions": [
            "core:window:allow-start-dragging",
            "core:window:allow-is-fullscreen",
            "core:window:allow-is-maximized",
            "core:event:allow-listen"
          ]
        }
      ]
    }
  }
}
```

#### 5. Import TypeScript Functions

**Option 1: Use from npm package (recommended)**
```typescript
import { enableModernWindowStyle } from '@cloudworxx/tauri-plugin-mac-rounded-corners';
```

**Option 2: Copy TypeScript file locally**
```bash
mkdir -p src/lib
cp node_modules/@cloudworxx/tauri-plugin-mac-rounded-corners/index.ts src/lib/mac-rounded-corners.ts
```

Then import from local file:
```typescript
import { enableModernWindowStyle } from './lib/mac-rounded-corners';
```

## Usage

### Basic Usage

**After npm installation:**

```typescript
import { useEffect } from 'react';
import { enableModernWindowStyle } from '@cloudworxx/tauri-plugin-mac-rounded-corners';

function App() {
  useEffect(() => {
    // Enable rounded corners on mount
    // Auto-repositioning after fullscreen is enabled automatically
    enableModernWindowStyle();
  }, []);

  return <div>Your App</div>;
}
```

### With Custom Configuration

```typescript
import { enableModernWindowStyle } from '@cloudworxx/tauri-plugin-mac-rounded-corners';

// Custom corner radius and Traffic Lights position
await enableModernWindowStyle({
  cornerRadius: 16,      // Default: 12
  offsetX: 5,            // Move Traffic Lights 5px right
  offsetY: 10,           // Move Traffic Lights 10px down
});
```

### Manual Repositioning

```typescript
import { repositionTrafficLights } from '@cloudworxx/tauri-plugin-mac-rounded-corners';

// Manually reposition Traffic Lights (useful after window state changes)
await repositionTrafficLights();
```

### Simple Mode (No Shadow)

```typescript
import { enableRoundedCorners } from '@cloudworxx/tauri-plugin-mac-rounded-corners';

// Simpler version without shadow and layer-based clipping
await enableRoundedCorners({
  offsetX: 0,
  offsetY: 0,
});
```

## CSS Setup

Add to your CSS to ensure clean rounded corners:

```css
html, body, #root {
  margin: 0;
  padding: 0;
  background-color: transparent;
  overflow: hidden;
}

.App {
  width: 100%;
  height: 100vh;
  background-color: #f6f6f6; /* Your app background */
}
```

## Traffic Lights Positioning

The Traffic Lights (Close, Minimize, Maximize) can be positioned using offset values:

- **offsetX**: Horizontal offset in pixels
  - Positive = move right
  - Negative = move left
  - Default: `0.0`

- **offsetY**: Vertical offset in pixels
  - Positive = move down
  - Negative = move up
  - Default: `0.0`

### Examples

```typescript
// Standard macOS position (top-left)
enableModernWindowStyle({ offsetX: 0, offsetY: 0 });

// Move right and down
enableModernWindowStyle({ offsetX: 10, offsetY: 15 });

// Move left (careful with negative values!)
enableModernWindowStyle({ offsetX: -5, offsetY: 5 });
```

**⚠️ Warning:** Avoid extreme offset values (< -5px or very large positive values) as they may cause Traffic Lights to become unclickable.

## API Reference

### `enableRoundedCorners(config?)`

Enables basic rounded corners with transparent titlebar.

**Parameters:**
- `config.offsetX` (optional): Horizontal Traffic Lights offset (default: 0)
- `config.offsetY` (optional): Vertical Traffic Lights offset (default: 0)

**Returns:** `Promise<void>`

### `enableModernWindowStyle(config?)`

Enables modern window style with rounded corners, shadow, and layer-based clipping. **Recommended.**

**Parameters:**
- `config.cornerRadius` (optional): Corner radius in pixels (default: 12)
- `config.offsetX` (optional): Horizontal Traffic Lights offset (default: 0)
- `config.offsetY` (optional): Vertical Traffic Lights offset (default: 0)

**Returns:** `Promise<void>`

### `repositionTrafficLights()` ✨ NEW

Manually repositions Traffic Lights to their configured position. Useful if you need to manually trigger repositioning.

**Returns:** `Promise<void>`

### `cleanupRoundedCorners()` ✨ NEW

Cleanup function to remove event listeners. Call when component unmounts.

**Returns:** `void`

## Tauri Configuration

Ensure your `tauri.conf.json` has decorations disabled:

```json
{
  "tauri": {
    "windows": [
      {
        "decorations": false
      }
    ]
  }
}
```

**Important:** Do NOT set `transparent: true` - this plugin provides rounded corners without requiring the transparency API, making it more reliable and App Store compatible.

## Platform Support

| Feature | macOS | Windows | Linux |
|---------|-------|---------|-------|
| Rounded Corners | ✅ | ❌ | ❌ |
| Traffic Lights | ✅ (native) | N/A | N/A |
| Shadow | ✅ (native) | ✅ | ✅ |
| Auto-reposition | ✅ | N/A | N/A |

On non-macOS platforms, the plugin commands are no-ops and won't throw errors.

## Troubleshooting

### Traffic Lights not repositioning after fullscreen

**Solution:** Ensure you have the required permissions in `tauri.conf.json`:
```json
"core:window:allow-is-fullscreen",
"core:event:allow-listen"
```

### Traffic Lights not clickable

- Check your offset values - they may be too extreme
- Ensure all window style masks are set (the plugin handles this)
- Safe ranges: `offsetX: -5 to 20`, `offsetY: 0 to 20`

### White flicker in corners

- Ensure `html`, `body`, `#root` have `background-color: transparent`
- Remove `border-radius` from these elements
- Let the Rust layer handle the clipping

### Close button disabled

- This is fixed in v1.1.0 (adds `NSClosableWindowMask`)
- Make sure you're using the latest version

### Permission errors in console

Add all required permissions to `tauri.conf.json` (see Installation step 4)

## Technical Details

### Used macOS APIs (all public)

- `NSWindow.styleMask`
- `NSFullSizeContentViewWindowMask`
- `setTitlebarAppearsTransparent`
- `setTitleVisibility`
- `setHasShadow`
- `CALayer.setCornerRadius`
- `setMasksToBounds`
- `standardWindowButton` (for repositioning)

**No private APIs** are used - fully App Store compliant.

### How Auto-Reposition Works

The plugin uses Tauri's `onResized` event listener to detect when the window exits fullscreen mode. When this event fires, it immediately calls `repositionTrafficLights()` to restore the custom position. This is event-based and instant (~0-50ms), not polling-based.

### Why this works

1. `NSFullSizeContentViewWindowMask` allows content in titlebar region
2. Transparent titlebar removes default chrome
3. Layer-based corner radius clips the content
4. Style masks enable Traffic Light functionality
5. Frame manipulation positions the buttons
6. Event listener repositions after state changes

## Requirements

- Tauri v2.x
- macOS 10.15 or later
- Rust stable toolchain
- Node.js 18+

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for detailed version history.

### Recent Changes

**v1.1.0** (2025-01-02)
- Added auto-reposition feature for Traffic Lights after fullscreen
- New `reposition_traffic_lights()` command
- Event-based resize listener
- Fixed Close button not working
- Added required permissions documentation

**v1.0.0** (2025-01-02)
- Initial release

## FAQ

**Q: Does this work with Tauri v1?**  
A: No, this plugin is designed for Tauri v2. For Tauri v1, you would need to adapt the code.

**Q: Can I use this in production?**  
A: Yes! The plugin uses only public APIs and is App Store compatible.

**Q: Will this work on Windows/Linux?**  
A: The commands are no-ops on non-macOS platforms, so your app won't crash, but rounded corners are macOS-only.

**Q: Can I customize the corner radius?**  
A: Yes! Use the `cornerRadius` parameter in `enableModernWindowStyle()`.

**Q: Do I need `transparent: true` in tauri.conf.json?**  
A: No! This plugin works without the transparency API, which makes it more reliable and avoids potential App Store issues.

**Q: Why do Traffic Lights reset position after fullscreen?**  
A: macOS resets custom button positions during fullscreen transitions. v1.1.0 automatically fixes this with event-based repositioning.

**Q: Is this free?**  
A: Yes! MIT licensed - free for personal and commercial use.

## Support

- 📧 Email: [hello@cloudworxx.us](mailto:hello@cloudworxx.us)
- 🐛 Issues: [GitHub Issues](https://github.com/cloudworxx/tauri-plugin-mac-rounded-corners/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/cloudworxx/tauri-plugin-mac-rounded-corners/discussions)

## License

MIT License - see [LICENSE](LICENSE) file for details.

Copyright (c) 2025 Sascha Klein (cloudworxx.us)

## Credits

Created for [PromptDeck](https://getpromptdeck.com)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Star History

If you find this plugin useful, please consider giving it a ⭐ on GitHub!

---

Made with ❤️ by Sascha Klein (cloudworxx.us)
