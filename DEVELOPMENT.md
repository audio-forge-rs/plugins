# Audio Forge RS - Development Guide

This repository contains multiple Bitwig CLAP/VST3 plugins built with Rust and [nih-plug](https://github.com/robbert-vdh/nih-plug).

## Project Structure

```
audio-forge-rs/plugins/
├── Cargo.toml              # Workspace configuration
├── .cargo/
│   └── config.toml         # Build optimizations
├── plugins/
│   ├── gain/               # Example: Simple gain plugin
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs      # Plugin implementation
│   │       └── editor.rs   # GUI (using VIZIA)
│   └── [your-plugin]/      # Add more plugins here
├── xtask/                  # Build automation
│   ├── Cargo.toml
│   └── src/main.rs
├── justfile                # Task runner commands
└── DEVELOPMENT.md          # This file
```

## Prerequisites

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Just** (optional but recommended):
   ```bash
   # macOS
   brew install just
   
   # or using cargo
   cargo install just
   ```

3. **Platform-specific dependencies**:
   
   **macOS**: Xcode Command Line Tools
   ```bash
   xcode-select --install
   ```
   
   **Linux**: Development libraries
   ```bash
   # Ubuntu/Debian
   sudo apt install libx11-dev libxcursor-dev libxcb1-dev libxcb-render0-dev \
                    libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev
   
   # Fedora
   sudo dnf install libX11-devel libXcursor-devel libxcb-devel alsa-lib-devel
   ```

## Quick Start

### Using Just (recommended)

```bash
# List all available commands
just

# Build all plugins in debug mode
just build

# Build and bundle all plugins for release
just bundle

# Build a specific plugin
just bundle-plugin gain

# Install plugins to your system (macOS)
just install-mac

# Run tests
just test

# Run linter
just lint

# Format code
just fmt
```

### Using Cargo directly

```bash
# Build all plugins
cargo build --release

# Build and bundle plugins
cargo xtask bundle

# Build specific plugin
cargo build --release -p audio-forge-gain

# Run tests
cargo test

# Run clippy
cargo clippy
```

## Creating a New Plugin

### Method 1: Manual creation

1. Create a new directory in `plugins/`:
   ```bash
   mkdir -p plugins/my-plugin/src
   ```

2. Create `plugins/my-plugin/Cargo.toml`:
   ```toml
   [package]
   name = "audio-forge-my-plugin"
   version.workspace = true
   edition.workspace = true
   authors.workspace = true
   license.workspace = true

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   nih_plug = { workspace = true }
   nih_plug_vizia = { workspace = true }
   ```

3. Copy and modify `plugins/gain/src/lib.rs` as your starting point

4. The plugin will automatically be included in the workspace

### Method 2: Copy the gain example

```bash
cp -r plugins/gain plugins/my-plugin
# Then update the Cargo.toml and lib.rs with your plugin details
```

## Building and Testing

### Debug builds (faster compilation)
```bash
cargo build
```

### Release builds (optimized for performance)
```bash
cargo build --release
```

### Bundle plugins (creates .clap and .vst3 files)
```bash
cargo xtask bundle
# Outputs to: target/bundled/
```

### Install to system
```bash
# macOS
just install-mac

# Linux  
just install-linux

# Windows
# Copy from target/bundled/ to:
# %COMMONPROGRAMFILES%\VST3\
# %COMMONPROGRAMFILES%\CLAP\
```

## Plugin Development Tips

### 1. Hot Reloading
While nih-plug doesn't support true hot reloading, you can:
- Keep Bitwig open
- Rebuild the plugin: `just bundle-plugin my-plugin`
- Remove and re-add the plugin in Bitwig

### 2. Debugging
```bash
# Build with debug symbols
cargo build --profile profiling

# Use print debugging (appears in terminal when running Bitwig from CLI)
eprintln!("Debug: {}", value);

# Or use the nih_debug_assert! macros in nih-plug
```

### 3. GUI Development
The example uses `nih_plug_vizia`. You can also use:
- `nih_plug_egui` - Immediate mode GUI
- No GUI - Just parameters

### 4. Testing Audio
```bash
# Run unit tests
cargo test

# For integration testing, use:
# - Bitwig Studio
# - REAPER (with great CLAP support)
# - Standalone validators like pluginval
```

## Workspace Configuration

### Shared Dependencies
Common dependencies are defined in the root `Cargo.toml`:
- `nih_plug` - Core plugin framework
- `nih_plug_vizia` - GUI framework
- `nih_plug_egui` - Alternative GUI framework

To use in a plugin:
```toml
[dependencies]
nih_plug = { workspace = true }
```

### Build Profiles
- `release` - Optimized builds with LTO
- `profiling` - Release build with debug symbols

## Common Commands Reference

| Task | Just | Cargo |
|------|------|-------|
| Build all | `just build` | `cargo build` |
| Bundle all | `just bundle` | `cargo xtask bundle` |
| Bundle one | `just bundle-plugin gain` | `cargo xtask bundle-plugin gain` |
| Test | `just test` | `cargo test` |
| Lint | `just lint` | `cargo clippy` |
| Format | `just fmt` | `cargo fmt` |
| Clean | `just clean` | `cargo clean` |
| Install | `just install-mac` | (manual copy) |

## Troubleshooting

### Plugin doesn't appear in Bitwig
1. Check the bundled output exists: `ls target/bundled/`
2. Verify plugin is copied to the correct directory
3. Rescan plugins in Bitwig
4. Check Bitwig's plugin blacklist

### Build errors
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Update dependencies
cargo update
```

### GUI issues
- VIZIA requires specific platform libraries (see Prerequisites)
- Check the nih-plug repository for known issues

## Resources

- [nih-plug documentation](https://github.com/robbert-vdh/nih-plug)
- [nih-plug examples](https://github.com/robbert-vdh/nih-plug/tree/master/plugins)
- [CLAP specification](https://github.com/free-audio/clap)
- [Rust Audio Discord](https://discord.gg/rust-audio)

## CI/CD

The workspace is set up for easy CI/CD integration:

```bash
# Run all checks (like CI would)
just ci

# This runs:
# - Format check
# - Clippy linting  
# - Tests
```

## Contributing

1. Create a new branch for your plugin/feature
2. Follow the existing code style (run `just fmt`)
3. Ensure `just ci` passes
4. Create a pull request

## License

All plugins are licensed under AGPL-3.0. See LICENSE file for details.
