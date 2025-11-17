# Audio Forge RS

Open source DAW plugins for Bitwig, crafted in Rust.

## About

Audio Forge RS is an organization dedicated to building high-quality, open source audio plugins specifically designed for Bitwig Studio. All plugins are written in Rust using modern plugin development frameworks, primarily [nih-plug](https://github.com/robbert-vdh/nih-plug), which provides excellent support for VST3 and CLAP formats that Bitwig uses.

## Philosophy

- **Open Source First**: All our plugins are released under the GNU Affero General Public License v3.0, ensuring they remain free and open forever.
- **Bitwig-Focused**: While our plugins work in any compatible DAW, we optimize the experience specifically for Bitwig Studio's workflow and modulators.
- **Rust-Powered**: Built with Rust for memory safety, performance, and modern development practices.
- **Community-Driven**: We welcome contributions, feedback, and collaboration from the audio development community.

## Technology Stack

Our plugins are built using:
- **[nih-plug](https://github.com/robbert-vdh/nih-plug)**: Modern Rust framework for VST3 and CLAP plugins
- **Rust**: Systems programming language for performance and safety
- **CLAP & VST3**: Industry-standard plugin formats with excellent Bitwig integration

## Plugins

This is a monorepo containing multiple plugins:

- **[Tube Screamer](plugins/tubescreamer/)** - Professional emulation of the classic TS808/TS9 overdrive pedal with authentic circuit modeling
- **Gain** - Simple gain/volume utility plugin (example/template)

More plugins coming soon!

## Getting Started

### For Users
1. Download the latest release from the [Releases](https://github.com/audio-forge-rs/plugins/releases) page
2. Copy the `.clap` or `.vst3` file to your plugin directory:
   - **macOS**: `~/Library/Audio/Plug-Ins/CLAP/` or `~/Library/Audio/Plug-Ins/VST3/`
   - **Linux**: `~/.clap/` or `~/.vst3/`
   - **Windows**: `C:\Program Files\Common Files\CLAP\` or `C:\Program Files\Common Files\VST3\`
3. Rescan plugins in Bitwig Studio

### For Developers
1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Clone this repository: `git clone https://github.com/audio-forge-rs/plugins.git`
3. Build all plugins: `cargo xtask bundle` or `just bundle`
4. Find the built plugins in `target/bundled/`

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed development documentation.

## Contributing

We welcome contributions! Whether you're:
- Reporting bugs
- Suggesting features
- Improving documentation
- Submitting pull requests

Please check individual plugin repositories for specific contribution guidelines.

## License

All Audio Forge RS projects are licensed under the [GNU Affero General Public License v3.0](LICENSE).

This ensures that:
- You can freely use, modify, and distribute our plugins
- If you modify and distribute plugins (including over a network), you must share your changes
- Commercial use is permitted as long as you comply with the AGPL terms

## Contact

- GitHub: [@audio-forge-rs](https://github.com/audio-forge-rs)
- Issues: Use individual plugin repositories

## Resources

- [nih-plug documentation](https://github.com/robbert-vdh/nih-plug)
- [Bitwig Studio](https://www.bitwig.com/)
- [CLAP specification](https://github.com/free-audio/clap)
- [Rust Audio Community](https://rust.audio/)

---

*Forging audio tools in Rust, one plugin at a time.*