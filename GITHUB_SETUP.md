# GitHub Repository Setup Guide

## Repository Settings

### Description
Use one of these from REPO_DESCRIPTION.md:
```
Tauri v2 plugin for native macOS rounded corners with customizable Traffic Lights positioning. App Store compatible. MIT licensed.
```

### Topics (GitHub Tags)
Add these topics to your repository for better discoverability:

```
tauri
tauri-plugin
tauri-v2
macos
rounded-corners
traffic-lights
frameless-window
window-decorations
desktop-app
rust
typescript
app-store
cocoa
native-ui
```

### Website
```
https://cloudworxx.us
```

### Social Preview Image
Consider creating a 1280x640px image showing:
- Before/After comparison
- The rounded corners in action
- Traffic Lights positioning examples

## Initial Git Setup

```bash
cd /Volumes/Development/_ELECTRON-TAURI/tauri-plugin-mac-rounded-corners/tauri-plugin-mac-rounded-corners/

# Initialize git if not done
git init

# Add files
git add .

# First commit
git commit -m "Initial release v1.0.0

- Native macOS rounded corners support
- Customizable Traffic Lights positioning
- App Store compatible (public APIs only)
- TypeScript support
- Full documentation"

# Add remote (replace with your GitHub URL)
git remote add origin https://github.com/cloudworxx/tauri-plugin-mac-rounded-corners.git

# Push
git branch -M main
git push -u origin main

# Create release tag
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

## GitHub Repository Settings

### Enable Features
- ✅ Issues
- ✅ Discussions (optional, for community Q&A)
- ✅ Projects (optional)
- ✅ Wiki (optional)

### Branch Protection (optional)
- Require pull request reviews before merging
- Require status checks to pass

### About Section
- Add description
- Add website: https://cloudworxx.us
- Add topics (see above)

## Create First Release

1. Go to Releases → "Create a new release"
2. Tag: `v1.0.0`
3. Title: `v1.0.0 - Initial Release`
4. Description: Copy from CHANGELOG.md
5. Attach any screenshots or demo videos
6. Publish release

## Optional: npm Package

If you want to publish on npm:

```bash
# Login to npm
npm login

# Publish (make sure package.json name is correct)
npm publish --access public
```

## Promotion

Consider sharing on:
- Reddit: r/tauri, r/rust
- Twitter/X with #Tauri #Rust #macOS
- Tauri Discord community
- Dev.to blog post

## README Badges to Update

After publishing, update these in README.md:
- npm version badge
- npm downloads badge
- GitHub stars badge
- GitHub issues badge
