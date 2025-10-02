# Integration Guide

This guide shows you how to integrate the macOS Rounded Corners plugin into your Tauri v2 project.

## Quick Start (5 Minutes)

### 1. Copy Plugin Files

```bash
# From the plugin directory
cp mod.rs your-project/src-tauri/src/plugins/mac_rounded_corners.rs
cp index.ts your-project/src/lib/mac-rounded-corners.ts
```

### 2. Update Cargo.toml

Add to `your-project/src-tauri/Cargo.toml`:

```toml
[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.26"
objc = "0.2.7"
```

### 3. Register Commands

In `your-project/src-tauri/src/lib.rs`:

```rust
mod plugins;
use plugins::mac_rounded_corners;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            mac_rounded_corners::enable_rounded_corners,
            mac_rounded_corners::enable_modern_window_style,
            mac_rounded_corners::reposition_traffic_lights  // v1.1.0
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Create `your-project/src-tauri/src/plugins/mod.rs`:

```rust
pub mod mac_rounded_corners;
```

### 4. Add Tauri Permissions

**CRITICAL STEP**: Add to `your-project/src-tauri/tauri.conf.json`:

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

**Note**: If you already have a `capabilities` array, just add the new permissions to your existing capability object.

### 5. Use in Your App

```typescript
// In your main component (e.g., App.tsx)
import { useEffect } from 'react';
import { enableModernWindowStyle } from './lib/mac-rounded-corners';

function App() {
  useEffect(() => {
    // Enable rounded corners with auto-reposition
    enableModernWindowStyle({
      cornerRadius: 12,
      offsetX: 5,
      offsetY: 10
    });
  }, []);

  return <div>Your App</div>;
}
```

Done! Your Tauri app now has beautiful rounded corners on macOS.

## Advanced Configuration

### Custom Traffic Lights Position

```typescript
enableModernWindowStyle({
  offsetX: 10,   // Move 10px to the right
  offsetY: 15,   // Move 15px down
});
```

### Manual Repositioning

If you need to manually trigger repositioning (rare):

```typescript
import { repositionTrafficLights } from './lib/mac-rounded-corners';

// After some window state change
await repositionTrafficLights();
```

### Cleanup on Unmount

```typescript
import { cleanupRoundedCorners } from './lib/mac-rounded-corners';

useEffect(() => {
  enableModernWindowStyle();
  
  return () => {
    cleanupRoundedCorners(); // Remove event listeners
  };
}, []);
```

## Tauri Configuration

Ensure `decorations: false` in `tauri.conf.json`:

```json
{
  "app": {
    "windows": [
      {
        "decorations": false
      }
    ]
  }
}
```

**Do NOT add `transparent: true`** - it's not needed and can cause issues.

## CSS Setup

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
  background-color: #f6f6f6;
}
```

## Troubleshooting

### Permission Errors

If you see errors like `window.is_fullscreen not allowed`, you forgot step 4. Add the permissions to `tauri.conf.json`.

### Traffic Lights Don't Reposition After Fullscreen

Make sure you:
1. Added the permissions (step 4)
2. Registered the `reposition_traffic_lights` command (step 3)
3. Are using v1.1.0 or later

### Compile Errors

If you get Rust compile errors:
```bash
cd src-tauri
cargo clean
cd ..
npm run tauri dev
```

## Testing

After integration:
1. Run `npm run tauri dev`
2. Check for rounded corners
3. Test Traffic Lights (all three buttons should work)
4. Go fullscreen and back - buttons should stay in position

## What's Next?

- Customize the corner radius
- Adjust Traffic Lights position
- Build your app: `npm run tauri build`

For more details, see the [README.md](README.md).
