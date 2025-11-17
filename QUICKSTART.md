# Quick Start Guide

Get up and running with Audio Forge RS plugin development in 5 minutes.

## Setup (One Time)

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Install Just (optional but recommended)
cargo install just

# 3. Verify setup
cargo --version
just --version  # if installed
```

## Build Your First Plugin

```bash
# Build the example gain plugin
just bundle-plugin gain

# Or without just:
cargo xtask bundle-plugin gain

# Output will be in: target/bundled/
```

## Install to Bitwig

### macOS
```bash
# Quick install
just install-mac

# Or manually:
cp -r target/bundled/*.clap ~/Library/Audio/Plug-Ins/CLAP/
cp -r target/bundled/*.vst3 ~/Library/Audio/Plug-Ins/VST3/
```

### Linux
```bash
# Quick install
just install-linux

# Or manually:
cp -r target/bundled/*.clap ~/.clap/
cp -r target/bundled/*.vst3 ~/.vst3/
```

### Windows
```powershell
# Copy manually to:
# C:\Program Files\Common Files\CLAP\
# C:\Program Files\Common Files\VST3\
```

Then rescan plugins in Bitwig Studio.

## Create Your Own Plugin

```bash
# 1. Create directory structure
mkdir -p plugins/my-plugin/src

# 2. Copy the template
cp plugins/gain/Cargo.toml plugins/my-plugin/Cargo.toml
cp -r plugins/gain/src/* plugins/my-plugin/src/

# 3. Edit Cargo.toml - change the name:
#    name = "audio-forge-my-plugin"

# 4. Edit src/lib.rs - update:
#    - Plugin name: const NAME: &'static str = "Audio Forge My Plugin";
#    - CLAP_ID: const CLAP_ID: &'static str = "com.audio-forge-rs.my-plugin";
#    - VST3_CLASS_ID: const VST3_CLASS_ID: [u8; 16] = *b"AFMyPluginXXXXXX";
#    - Your DSP code in the process() function

# 5. Build it
just bundle-plugin my-plugin

# 6. Install and test
just install-mac  # or install-linux
```

## Common Commands

| What | Command |
|------|---------|
| Build all plugins | `just bundle` |
| Build one plugin | `just bundle-plugin gain` |
| Install to system | `just install-mac` or `just install-linux` |
| Run tests | `just test` |
| Check code style | `just lint` |
| Format code | `just fmt` |
| Clean build files | `just clean` |

## Project Structure

```
plugins/
├── Cargo.toml              # Workspace config - shared settings
├── plugins/
│   ├── gain/               # Example plugin (copy this!)
│   │   ├── Cargo.toml      # Plugin package config
│   │   └── src/
│   │       ├── lib.rs      # Main plugin code + DSP
│   │       └── editor.rs   # GUI code
│   └── my-plugin/          # Your plugins go here
└── xtask/                  # Build automation (don't touch)
```

## Editing Plugin Code

### Where to add parameters (`src/lib.rs`)

```rust
#[derive(Params)]
struct MyPluginParams {
    // Add parameters here:
    #[id = "gain"]
    pub gain: FloatParam,
    
    #[id = "mix"]
    pub mix: FloatParam,
}
```

### Where to add DSP (`src/lib.rs`)

```rust
fn process(&mut self, buffer: &mut Buffer, ...) -> ProcessStatus {
    for channel_samples in buffer.iter_samples() {
        // Get parameter values
        let gain = self.params.gain.smoothed.next();
        
        for sample in channel_samples {
            // Process audio here
            *sample *= gain;
        }
    }
    ProcessStatus::Normal
}
```

### Where to add GUI widgets (`src/editor.rs`)

```rust
VStack::new(cx, |cx| {
    Label::new(cx, "My Plugin");
    
    // Add sliders here:
    ParamSlider::new(cx, Data::params, |params| &params.gain);
    ParamSlider::new(cx, Data::params, |params| &params.mix);
});
```

## Testing in Bitwig

1. Build: `just bundle-plugin my-plugin`
2. Install: `just install-mac`
3. Open Bitwig
4. Rescan plugins (Preferences > Plug-ins > Rescan)
5. Add plugin to a track
6. Test!

## When Things Go Wrong

### Plugin doesn't show up in Bitwig
```bash
# Check it was built
ls target/bundled/

# Check it was installed (macOS)
ls ~/Library/Audio/Plug-Ins/CLAP/

# Rescan in Bitwig
# Check Bitwig's plugin blacklist
```

### Build errors
```bash
# Clean and rebuild
cargo clean
just bundle-plugin my-plugin

# Check for typos in Cargo.toml and lib.rs
```

### Need help?
- Read [DEVELOPMENT.md](DEVELOPMENT.md) for detailed docs
- Check [.github/PLUGIN_TEMPLATE.md](.github/PLUGIN_TEMPLATE.md) for code templates
- Look at the gain plugin example in `plugins/gain/`
- Check [nih-plug examples](https://github.com/robbert-vdh/nih-plug/tree/master/plugins)

## Next Steps

1. **Explore the gain plugin** - Read through `plugins/gain/src/lib.rs`
2. **Read DEVELOPMENT.md** - Deep dive into plugin development
3. **Check the template** - See `.github/PLUGIN_TEMPLATE.md` for reference
4. **Start coding!** - Copy gain, rename it, and make it your own

Happy plugin building!
