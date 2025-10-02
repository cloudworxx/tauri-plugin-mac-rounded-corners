#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const PLUGIN_NAME = 'tauri-plugin-mac-rounded-corners';

// Colors for terminal output
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  red: '\x1b[31m',
  cyan: '\x1b[36m',
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function error(message) {
  log(`❌ ${message}`, 'red');
}

function success(message) {
  log(`✅ ${message}`, 'green');
}

function info(message) {
  log(`ℹ️  ${message}`, 'cyan');
}

function warning(message) {
  log(`⚠️  ${message}`, 'yellow');
}

function findProjectRoot() {
  let currentDir = process.cwd();
  
  while (currentDir !== path.parse(currentDir).root) {
    const packageJsonPath = path.join(currentDir, 'package.json');
    const tauriPath = path.join(currentDir, 'src-tauri');
    
    if (fs.existsSync(packageJsonPath) && fs.existsSync(tauriPath)) {
      return currentDir;
    }
    
    currentDir = path.dirname(currentDir);
  }
  
  return null;
}

function copyRustFile(projectRoot) {
  const pluginDir = path.join(__dirname, '..');
  const sourceFile = path.join(pluginDir, 'mod.rs');
  const targetDir = path.join(projectRoot, 'src-tauri', 'src', 'plugins');
  const targetFile = path.join(targetDir, 'mac_rounded_corners.rs');
  
  if (!fs.existsSync(sourceFile)) {
    error(`Source file not found: ${sourceFile}`);
    return false;
  }
  
  // Create plugins directory if it doesn't exist
  if (!fs.existsSync(targetDir)) {
    fs.mkdirSync(targetDir, { recursive: true });
    success(`Created directory: ${targetDir}`);
  }
  
  // Copy the file
  fs.copyFileSync(sourceFile, targetFile);
  success(`Copied Rust plugin to: ${targetFile}`);
  
  return true;
}

function updatePluginsMod(projectRoot) {
  const modFilePath = path.join(projectRoot, 'src-tauri', 'src', 'plugins', 'mod.rs');
  const modContent = 'pub mod mac_rounded_corners;\n';
  
  if (fs.existsSync(modFilePath)) {
    const existingContent = fs.readFileSync(modFilePath, 'utf8');
    if (existingContent.includes('mac_rounded_corners')) {
      info('plugins/mod.rs already includes mac_rounded_corners');
      return;
    }
    fs.appendFileSync(modFilePath, modContent);
  } else {
    fs.writeFileSync(modFilePath, modContent);
  }
  
  success('Updated plugins/mod.rs');
}

function showNextSteps() {
  log('\n' + '='.repeat(60), 'bright');
  log('Installation Complete! 🎉', 'green');
  log('='.repeat(60), 'bright');
  
  log('\n📝 Next Steps:\n', 'bright');
  
  log('1. Add dependencies to src-tauri/Cargo.toml:', 'cyan');
  log(`
[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.26"
objc = "0.2.7"
`, 'reset');
  
  log('2. Register commands in src-tauri/src/lib.rs:', 'cyan');
  log(`
mod plugins;
use plugins::mac_rounded_corners;

.invoke_handler(tauri::generate_handler![
    mac_rounded_corners::enable_rounded_corners,
    mac_rounded_corners::enable_modern_window_style,
    mac_rounded_corners::reposition_traffic_lights
])
`, 'reset');
  
  log('3. Add permissions to tauri.conf.json:', 'cyan');
  log(`
"permissions": [
    "core:window:allow-start-dragging",
    "core:window:allow-is-fullscreen",
    "core:window:allow-is-maximized",
    "core:event:allow-listen"
]
`, 'reset');
  
  log('4. Use in your app:', 'cyan');
  log(`
import { enableModernWindowStyle } from '@cloudworxx/${PLUGIN_NAME}';

useEffect(() => {
  enableModernWindowStyle();
}, []);
`, 'reset');
  
  log('\n📚 Documentation:', 'bright');
  log('   https://github.com/cloudworxx/tauri-plugin-mac-rounded-corners#readme\n', 'reset');
}

function main() {
  log('\n🔧 Installing Tauri Plugin: macOS Rounded Corners\n', 'bright');
  
  const projectRoot = findProjectRoot();
  
  if (!projectRoot) {
    error('Could not find Tauri project root!');
    error('Make sure you run this from within a Tauri project directory.');
    process.exit(1);
  }
  
  info(`Project root: ${projectRoot}`);
  
  const success1 = copyRustFile(projectRoot);
  
  if (!success1) {
    error('Installation failed!');
    process.exit(1);
  }
  
  updatePluginsMod(projectRoot);
  
  showNextSteps();
}

main();
