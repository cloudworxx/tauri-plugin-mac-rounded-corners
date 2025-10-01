import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

export interface RoundedCornersConfig {
  /**
   * Corner radius in pixels (default: 12.0)
   */
  cornerRadius?: number;
  /**
   * Horizontal offset for Traffic Lights in pixels
   * Positive = right, Negative = left (default: 0.0)
   */
  offsetX?: number;
  /**
   * Vertical offset for Traffic Lights in pixels
   * Positive = down, Negative = up (default: 0.0)
   */
  offsetY?: number;
}

/**
 * Enables rounded corners for the current window (macOS only)
 * Uses only public APIs - App Store compatible
 */
export async function enableRoundedCorners(config?: RoundedCornersConfig): Promise<void> {
  try {
    const window = getCurrentWebviewWindow();
    await invoke('enable_rounded_corners', {
      window,
      offsetX: config?.offsetX ?? 0.0,
      offsetY: config?.offsetY ?? 0.0,
    });
  } catch (error) {
    console.error('Failed to enable rounded corners:', error);
    throw error;
  }
}

/**
 * Enables modern window style with rounded corners and shadow (macOS only)
 * Recommended method for best visual appearance
 */
export async function enableModernWindowStyle(config?: RoundedCornersConfig): Promise<void> {
  try {
    const window = getCurrentWebviewWindow();
    await invoke('enable_modern_window_style', {
      window,
      cornerRadius: config?.cornerRadius ?? 12.0,
      offsetX: config?.offsetX ?? 0.0,
      offsetY: config?.offsetY ?? 0.0,
    });
  } catch (error) {
    console.error('Failed to enable modern window style:', error);
    throw error;
  }
}
