# Plugin Template

Use this as a guide when creating new plugins.

## Quick Start

```bash
# Create the plugin directory
mkdir -p plugins/my-plugin/src

# Copy this template structure
```

## File: `plugins/my-plugin/Cargo.toml`

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
nih_plug_vizia = { workspace = true }  # or nih_plug_egui
atomic_float = { workspace = true }

# Add your specific dependencies here
```

## File: `plugins/my-plugin/src/lib.rs`

```rust
use nih_plug::prelude::*;
use std::sync::Arc;

// If using a GUI, import the editor module
mod editor;

/// Brief description of your plugin
struct MyPlugin {
    params: Arc<MyPluginParams>,
}

#[derive(Params)]
struct MyPluginParams {
    // GUI state (if using a GUI)
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    // Your parameters here
    #[id = "param1"]
    pub param1: FloatParam,
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(MyPluginParams::default()),
        }
    }
}

impl Default for MyPluginParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            
            param1: FloatParam::new(
                "Parameter 1",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}

impl Plugin for MyPlugin {
    const NAME: &'static str = "Audio Forge My Plugin";
    const VENDOR: &'static str = "Audio Forge RS";
    const URL: &'static str = "https://github.com/audio-forge-rs/plugins";
    const EMAIL: &'static str = "info@audio-forge-rs.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn reset(&mut self) {
        // Reset internal state
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            // Get smoothed parameter values
            let param1 = self.params.param1.smoothed.next();

            // Process each sample
            for sample in channel_samples {
                // Your DSP code here
                *sample *= param1;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for MyPlugin {
    const CLAP_ID: &'static str = "com.audio-forge-rs.my-plugin";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Description of your plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        // Add more features as needed
    ];
}

impl Vst3Plugin for MyPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"AFMyPluginXXXXXX"; // Make this unique!
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        // Add more categories as needed
    ];
}

nih_export_clap!(MyPlugin);
nih_export_vst3!(MyPlugin);
```

## File: `plugins/my-plugin/src/editor.rs` (if using GUI)

```rust
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::MyPluginParams;

#[derive(Lens)]
struct Data {
    params: Arc<MyPluginParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (400, 300))
}

pub(crate) fn create(
    params: Arc<MyPluginParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "My Plugin")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Thin)
                .font_size(30.0)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            // Add your GUI widgets here
            ParamSlider::new(cx, Data::params, |params| &params.param1);
        })
        .row_between(Pixels(10.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}
```

## Important: Update VST3_CLASS_ID

Make sure to use a unique 16-byte ID for `VST3_CLASS_ID`. You can generate one using:

```bash
# Option 1: Use the first 16 chars of a UUID
uuidgen | head -c 16

# Option 2: Use your plugin name padded with X's
# Example: "AFMyPluginXXXXXX" for "MyPlugin"
```

## Parameter Types Reference

```rust
// Float parameter
FloatParam::new("Name", default, FloatRange::Linear { min: 0.0, max: 1.0 })

// Integer parameter  
IntParam::new("Name", default, IntRange::Linear { min: 0, max: 100 })

// Boolean parameter
BoolParam::new("Name", default)

// Enum parameter
EnumParam::new("Name", default)
```

## Common Float Ranges

```rust
// Linear
FloatRange::Linear { min: 0.0, max: 1.0 }

// Gain in dB
FloatRange::Skewed {
    min: util::db_to_gain(-30.0),
    max: util::db_to_gain(30.0),
    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
}

// Frequency
FloatRange::Skewed {
    min: 20.0,
    max: 20_000.0,
    factor: FloatRange::skew_factor(-2.0),
}
```

## Testing Your Plugin

```bash
# Build and bundle
just bundle-plugin my-plugin

# Install (macOS)
just install-mac

# Check the output
ls target/bundled/
```
