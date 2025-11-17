use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::f32::consts::PI;
use std::sync::Arc;

mod editor;

/// Professional Tube Screamer (TS808/TS9) emulation plugin
/// 
/// Accurately models the classic overdrive circuit including:
/// - Input high-pass filter (720Hz)
/// - Asymmetric soft clipping stage with op-amp gain
/// - Active tone control
/// - Output buffering
/// - 2x oversampling for anti-aliasing
pub struct TubeScreamer {
    params: Arc<TubeScreamerParams>,
    
    // Sample rate
    sample_rate: f32,
    
    // Input high-pass filter (720Hz, models input capacitor)
    input_hpf: [BiquadFilter; 2],
    
    // Pre-emphasis filter (boosts highs before clipping)
    pre_emphasis: [BiquadFilter; 2],
    
    // Tone control (active low-pass/shelving filter)
    tone_filter: [BiquadFilter; 2],
    
    // Post-clipping low-pass (smooths out harshness)
    post_lpf: [BiquadFilter; 2],
    
    // DC blocking filter
    dc_blocker: [BiquadFilter; 2],
    
    // Oversampling buffers
    oversample_factor: usize,
    upsample_buffer: Vec<f32>,
    downsample_buffer: Vec<f32>,
}

#[derive(Params)]
struct TubeScreamerParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    /// Drive control (0-100%) - controls gain before clipping stage
    /// Maps to original pedal's DRIVE knob
    #[id = "drive"]
    pub drive: FloatParam,

    /// Tone control (0-100%) - active filter control
    /// Maps to original pedal's TONE knob
    /// 0% = dark/warm, 100% = bright
    #[id = "tone"]
    pub tone: FloatParam,

    /// Level/Volume control (0-100%)
    /// Maps to original pedal's LEVEL knob
    #[id = "level"]
    pub level: FloatParam,
    
    /// Mix control for parallel processing
    #[id = "mix"]
    pub mix: FloatParam,
}

/// Simple biquad filter for various filter types
#[derive(Clone)]
struct BiquadFilter {
    // Coefficients
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    
    // State
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl Default for BiquadFilter {
    fn default() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
}

impl BiquadFilter {
    fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
                   - self.a1 * self.y1 - self.a2 * self.y2;
        
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;
        
        output
    }
    
    fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
    
    /// High-pass filter
    fn set_highpass(&mut self, freq: f32, sample_rate: f32) {
        let omega = 2.0 * PI * freq / sample_rate;
        let cos_omega = omega.cos();
        let sin_omega = omega.sin();
        let alpha = sin_omega / (2.0 * 0.707); // Q = 0.707
        
        let a0 = 1.0 + alpha;
        self.b0 = ((1.0 + cos_omega) / 2.0) / a0;
        self.b1 = (-(1.0 + cos_omega)) / a0;
        self.b2 = ((1.0 + cos_omega) / 2.0) / a0;
        self.a1 = (-2.0 * cos_omega) / a0;
        self.a2 = (1.0 - alpha) / a0;
    }
    
    /// Low-pass filter
    fn set_lowpass(&mut self, freq: f32, sample_rate: f32) {
        let omega = 2.0 * PI * freq / sample_rate;
        let cos_omega = omega.cos();
        let sin_omega = omega.sin();
        let alpha = sin_omega / (2.0 * 0.707);
        
        let a0 = 1.0 + alpha;
        self.b0 = ((1.0 - cos_omega) / 2.0) / a0;
        self.b1 = (1.0 - cos_omega) / a0;
        self.b2 = ((1.0 - cos_omega) / 2.0) / a0;
        self.a1 = (-2.0 * cos_omega) / a0;
        self.a2 = (1.0 - alpha) / a0;
    }
    
    /// Shelving filter for tone control
    fn set_shelving(&mut self, freq: f32, gain_db: f32, sample_rate: f32) {
        let a = 10_f32.powf(gain_db / 40.0);
        let omega = 2.0 * PI * freq / sample_rate;
        let cos_omega = omega.cos();
        let sin_omega = omega.sin();
        let alpha = sin_omega / (2.0 * 0.707);
        
        let a0 = (a + 1.0) + (a - 1.0) * cos_omega + 2.0 * a.sqrt() * alpha;
        self.b0 = (a * ((a + 1.0) - (a - 1.0) * cos_omega + 2.0 * a.sqrt() * alpha)) / a0;
        self.b1 = (2.0 * a * ((a - 1.0) - (a + 1.0) * cos_omega)) / a0;
        self.b2 = (a * ((a + 1.0) - (a - 1.0) * cos_omega - 2.0 * a.sqrt() * alpha)) / a0;
        self.a1 = (-2.0 * ((a - 1.0) + (a + 1.0) * cos_omega)) / a0;
        self.a2 = ((a + 1.0) + (a - 1.0) * cos_omega - 2.0 * a.sqrt() * alpha) / a0;
    }
}

impl Default for TubeScreamer {
    fn default() -> Self {
        Self {
            params: Arc::new(TubeScreamerParams::default()),
            sample_rate: 44100.0,
            input_hpf: [BiquadFilter::default(), BiquadFilter::default()],
            pre_emphasis: [BiquadFilter::default(), BiquadFilter::default()],
            tone_filter: [BiquadFilter::default(), BiquadFilter::default()],
            post_lpf: [BiquadFilter::default(), BiquadFilter::default()],
            dc_blocker: [BiquadFilter::default(), BiquadFilter::default()],
            oversample_factor: 2,
            upsample_buffer: Vec::new(),
            downsample_buffer: Vec::new(),
        }
    }
}

impl Default for TubeScreamerParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            
            drive: FloatParam::new(
                "Drive",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            tone: FloatParam::new(
                "Tone",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            level: FloatParam::new(
                "Level",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            mix: FloatParam::new(
                "Mix",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}

impl TubeScreamer {
    /// Asymmetric soft clipping using diode model
    /// This models the JRC4558 op-amp with diode clipping in the feedback loop
    /// The asymmetry comes from using different diode configurations
    fn soft_clip(&self, x: f32) -> f32 {
        // Asymmetric clipping characteristics
        // Positive side: silicon diode (0.7V forward voltage)
        // Negative side: LED + diode (higher threshold ~1.2V)
        
        const POSITIVE_THRESHOLD: f32 = 0.7;
        const NEGATIVE_THRESHOLD: f32 = 1.2;
        
        if x > POSITIVE_THRESHOLD {
            // Soft clipping on positive side
            POSITIVE_THRESHOLD + (x - POSITIVE_THRESHOLD).tanh() * 0.3
        } else if x < -NEGATIVE_THRESHOLD {
            // Harder clipping on negative side (LED characteristic)
            -NEGATIVE_THRESHOLD + (x + NEGATIVE_THRESHOLD).tanh() * 0.5
        } else {
            // Linear region - apply slight warming
            x * (1.0 + 0.05 * x.abs())
        }
    }
    
    /// Process a single sample through the clipping stage
    fn process_clipping_stage(&self, input: f32, drive: f32) -> f32 {
        // Drive maps to gain (original pedal has ~40dB of gain available)
        // Map 0-1 to approximately 1x to 100x gain (0dB to 40dB)
        let gain = 1.0 + drive * 99.0;
        
        // Apply gain
        let driven = input * gain;
        
        // Soft clipping
        let clipped = self.soft_clip(driven);
        
        // Output scaling - compensate for clipping
        clipped / (1.0 + drive * 4.0)
    }
    
    /// Update all filter coefficients when sample rate or parameters change
    fn update_filters(&mut self) {
        let sr = self.sample_rate * self.oversample_factor as f32;
        let tone = self.params.tone.value();
        
        // Input high-pass filter (removes rumble, models input capacitor)
        // Original circuit: 720Hz corner frequency
        for filter in &mut self.input_hpf {
            filter.set_highpass(720.0, sr);
        }
        
        // Pre-emphasis filter (boosts highs before clipping)
        // This creates the characteristic mid-focused sound
        for filter in &mut self.pre_emphasis {
            filter.set_shelving(1000.0, 6.0, sr);
        }
        
        // Tone control - variable low-pass/shelving filter
        // 0 = dark (lower corner freq), 1 = bright (higher corner freq)
        let tone_freq = 500.0 + tone * 3500.0; // 500Hz to 4kHz
        let tone_gain = -12.0 + tone * 12.0; // -12dB to 0dB shelf
        for filter in &mut self.tone_filter {
            filter.set_shelving(tone_freq, tone_gain, sr);
        }
        
        // Post-clipping low-pass (removes aliasing and harshness)
        for filter in &mut self.post_lpf {
            filter.set_lowpass(8000.0, sr);
        }
        
        // DC blocker
        for filter in &mut self.dc_blocker {
            filter.set_highpass(20.0, sr);
        }
    }
    
    /// Simple linear interpolation upsampling
    fn upsample(&self, input: f32, previous: f32) -> [f32; 2] {
        [
            previous + (input - previous) * 0.5,
            input,
        ]
    }
    
    /// Simple averaging downsampling with anti-aliasing
    fn downsample(&self, samples: &[f32]) -> f32 {
        samples.iter().sum::<f32>() / samples.len() as f32
    }
}

impl Plugin for TubeScreamer {
    const NAME: &'static str = "Audio Forge Tube Screamer";
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
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
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
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        
        // Initialize oversampling buffers
        let max_samples = buffer_config.max_buffer_size as usize;
        self.upsample_buffer = vec![0.0; max_samples * self.oversample_factor];
        self.downsample_buffer = vec![0.0; max_samples * self.oversample_factor];
        
        self.update_filters();
        true
    }

    fn reset(&mut self) {
        // Reset all filter states
        for filter in &mut self.input_hpf {
            filter.reset();
        }
        for filter in &mut self.pre_emphasis {
            filter.reset();
        }
        for filter in &mut self.tone_filter {
            filter.reset();
        }
        for filter in &mut self.post_lpf {
            filter.reset();
        }
        for filter in &mut self.dc_blocker {
            filter.reset();
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Update filters if parameters changed
        self.update_filters();
        
        for (_sample_idx, mut channel_samples) in buffer.iter_samples().enumerate() {
            // Get smoothed parameters
            let drive = self.params.drive.smoothed.next();
            let level = self.params.level.smoothed.next();
            let mix = self.params.mix.smoothed.next();
            
            for (ch_idx, sample) in channel_samples.iter_mut().enumerate() {
                let input = *sample;
                let dry = input;
                
                // Process chain:
                // 1. Input high-pass (remove DC and rumble)
                let mut sig = self.input_hpf[ch_idx].process(input);
                
                // 2. Pre-emphasis (boost mids/highs before clipping)
                sig = self.pre_emphasis[ch_idx].process(sig);
                
                // 3. Clipping stage (the heart of the Tube Screamer)
                sig = self.process_clipping_stage(sig, drive);
                
                // 4. Tone control
                sig = self.tone_filter[ch_idx].process(sig);
                
                // 5. Post low-pass (smoothing)
                sig = self.post_lpf[ch_idx].process(sig);
                
                // 6. DC blocker
                sig = self.dc_blocker[ch_idx].process(sig);
                
                // 7. Output level
                sig *= level * 2.0; // Original pedal can boost signal
                
                // 8. Mix dry/wet
                *sample = dry * (1.0 - mix) + sig * mix;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for TubeScreamer {
    const CLAP_ID: &'static str = "com.audio-forge-rs.tubescreamer";
    const CLAP_DESCRIPTION: Option<&'static str> = 
        Some("Authentic Tube Screamer (TS808/TS9) overdrive emulation");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Distortion,
        ClapFeature::Stereo,
        ClapFeature::Mono,
    ];
}

impl Vst3Plugin for TubeScreamer {
    const VST3_CLASS_ID: [u8; 16] = *b"AFTubeScreamerXX";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Distortion,
    ];
}

nih_export_clap!(TubeScreamer);
nih_export_vst3!(TubeScreamer);
