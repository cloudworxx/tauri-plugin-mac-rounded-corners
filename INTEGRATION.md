# Example Integration

This folder contains example files showing how to integrate the plugin into your Tauri project.

## Quick Start

1. **Copy the plugin files:**
   ```bash
   cp mod.rs your-project/src-tauri/src/plugins/mac_rounded_corners.rs
   cp index.ts your-project/src/lib/mac-rounded-corners.ts
   ```

2. **Update Cargo.toml:**
   ```toml
   [target.'cfg(target_os = "macos")'.dependencies]
   cocoa = "0.26"
   objc = "0.2.7"
   ```

3. **Register in lib.rs:**
   ```rust
   mod plugins;
   use plugins::mac_rounded_corners;
   
   .invoke_handler(tauri::generate_handler![
       mac_rounded_corners::enable_rounded_corners,
       mac_rounded_corners::enable_modern_window_style
   ])
   ```

4. **Use in your app:**
   ```typescript
   import { enableModernWindowStyle } from './lib/mac-rounded-corners';
   
   useEffect(() => {
     enableModernWindowStyle({
       cornerRadius: 12,
       offsetX: 5,
       offsetY: 10
     });
   }, []);
   ```

Done! Your Tauri app now has beautiful rounded corners on macOS.
