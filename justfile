# Audio Forge RS - Just commands for common tasks
# Install just: https://github.com/casey/just

# List all available commands
default:
    @just --list

# Build all plugins in debug mode
build:
    cargo build

# Build all plugins in release mode
build-release:
    cargo build --release

# Build and bundle all plugins (uses nih-plug's bundler)
bundle:
    #!/usr/bin/env bash
    set -euo pipefail
    for plugin in plugins/*/; do
        plugin_name=$(basename "$plugin")
        echo "Bundling $plugin_name..."
        cargo run --package xtask --release -- bundle audio-forge-$plugin_name --release
    done
    echo "✓ All plugins bundled to target/bundled/"

# Build and bundle a specific plugin
bundle-plugin plugin:
    cargo run --package xtask --release -- bundle audio-forge-{{plugin}} --release

# Run tests for all plugins
test:
    cargo test

# Run clippy linter
lint:
    cargo clippy --all-targets --all-features

# Fix common linting issues
fix:
    cargo clippy --all-targets --all-features --fix

# Format all code
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Clean build artifacts
clean:
    cargo clean

# Check if everything compiles without building
check:
    cargo check --all-targets

# Build documentation
docs:
    cargo doc --no-deps --open

# Run all checks (format, lint, test)
ci: fmt-check lint test

# Install plugins to system plugin directory (macOS)
install-mac: bundle
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p ~/Library/Audio/Plug-Ins/CLAP
    mkdir -p ~/Library/Audio/Plug-Ins/VST3
    cp -r target/bundled/*.clap ~/Library/Audio/Plug-Ins/CLAP/ 2>/dev/null || true
    cp -r target/bundled/*.vst3 ~/Library/Audio/Plug-Ins/VST3/ 2>/dev/null || true
    echo "Plugins installed to ~/Library/Audio/Plug-Ins/"

# Install plugins to system plugin directory (Linux)
install-linux: bundle
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p ~/.clap
    mkdir -p ~/.vst3
    cp -r target/bundled/*.clap ~/.clap/ 2>/dev/null || true
    cp -r target/bundled/*.vst3 ~/.vst3/ 2>/dev/null || true
    echo "Plugins installed to ~/.clap/ and ~/.vst3/"

# Watch for changes and rebuild
watch:
    cargo watch -x build

# Create a new plugin from template
new-plugin name:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p plugins/{{name}}/src
    echo "Creating new plugin: {{name}}"
    # You can expand this to copy template files
    echo "Plugin directory created at: plugins/{{name}}"
    echo "Don't forget to add it to the workspace!"
