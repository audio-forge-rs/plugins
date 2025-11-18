use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

mod editor;

/// Lonesome Picker - Intelligent banjo MIDI processor for Ample Ethno Banjo
/// 
/// Transforms any MIDI input into authentic alt-country/Americana banjo parts.
/// Designed for sparse, atmospheric, melodic banjo (not rapid bluegrass rolls).
/// 
/// Features:
/// - Smart picking pattern generation
/// - Drone string (5th string) integration
/// - Open tuning awareness (G-D-G-B-D)
/// - Automatic articulations (hammer-ons, pull-offs, slides)
/// - Clawhammer and fingerpicking intelligence
/// - Melodic phrase generation
/// - Sparse, atmospheric capabilities
/// - Optimized for Ample Ethno Banjo (picking mode)
pub struct LonesomePicker {
    params: Arc<LonesonmePickerParams>,
    
    // Input state
    active_input_notes: Vec<u8>,
    chord_notes: Vec<u8>,          // Current chord being held
    
    // Banjo state
    current_melody_note: Option<u8>,
    last_melody_note: Option<u8>,
    drone_active: bool,
    
    // Pattern state
    pattern_position: usize,
    pick_timer: f64,
    beat_position: f64,
    
    // Musical memory
    phrase_notes: Vec<u8>,         // Current phrase being played
    note_history: Vec<(u8, f64)>,  // Recent notes with timing
    
    // Timing
    sample_rate: f64,
    samples_per_beat: f64,
    current_sample: u64,
    next_pick_sample: u64,
}

#[derive(Params)]
struct LonesonmePickerParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    /// Picking style
    #[id = "style"]
    pub style: EnumParam<PickingStyle>,
    
    /// Density (how many notes)
    #[id = "density"]
    pub density: FloatParam,
    
    /// Drone string usage
    #[id = "drone"]
    pub drone: FloatParam,
    
    /// Melodic movement (passing tones, runs)
    #[id = "melodic"]
    pub melodic: FloatParam,
    
    /// Articulation variety
    #[id = "articulation"]
    pub articulation: FloatParam,
    
    /// Sparseness/space (opposite of busy)
    #[id = "sparseness"]
    pub sparseness: FloatParam,
    
    /// Auto-transpose to banjo range
    #[id = "auto_transpose"]
    pub auto_transpose: BoolParam,
}

#[derive(Enum, PartialEq, Clone, Copy)]
enum PickingStyle {
    /// Simple melody notes
    #[name = "Melody"]
    Melody,
    
    /// Clawhammer style (thumb + finger)
    #[name = "Clawhammer"]
    Clawhammer,
    
    /// Forward roll pattern
    #[name = "Forward Roll"]
    ForwardRoll,
    
    /// Alternating thumb pattern
    #[name = "Alternating"]
    Alternating,
    
    /// Sparse, atmospheric picking
    #[name = "Sparse"]
    Sparse,
    
    /// Melodic runs and phrases
    #[name = "Melodic Run"]
    MelodicRun,
}

// Ample Ethno Banjo range (5-string banjo in standard G tuning)
// String 5 (drone): G4 (67)
// String 1: D4 (62)
// String 2: B3 (59)
// String 3: G3 (55)
// String 4: D3 (50)
const BANJO_MIN_NOTE: u8 = 50;   // D3 (4th string open)
const BANJO_MAX_NOTE: u8 = 81;   // A5 (reasonable fret limit)
const BANJO_DRONE_NOTE: u8 = 67; // G4 (5th string)
const BANJO_SWEET_MAX: u8 = 74;  // D5

impl Default for LonesomePicker {
    fn default() -> Self {
        Self {
            params: Arc::new(LonesonmePickerParams::default()),
            active_input_notes: Vec::new(),
            chord_notes: Vec::new(),
            current_melody_note: None,
            last_melody_note: None,
            drone_active: false,
            pattern_position: 0,
            pick_timer: 0.0,
            beat_position: 0.0,
            phrase_notes: Vec::new(),
            note_history: Vec::new(),
            sample_rate: 44100.0,
            samples_per_beat: 44100.0,
            current_sample: 0,
            next_pick_sample: 0,
        }
    }
}

impl Default for LonesonmePickerParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            
            style: EnumParam::new("Style", PickingStyle::Clawhammer),
            
            density: FloatParam::new(
                "Density",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            drone: FloatParam::new(
                "Drone",
                0.6,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            melodic: FloatParam::new(
                "Melodic",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            articulation: FloatParam::new(
                "Articulation",
                0.6,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            sparseness: FloatParam::new(
                "Sparseness",
                0.4,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            auto_transpose: BoolParam::new("Auto Transpose", true),
        }
    }
}

impl LonesomePicker {
    /// Transpose note to banjo range
    fn transpose_to_banjo(&self, note: u8) -> Option<u8> {
        if !self.params.auto_transpose.value() {
            return Some(note);
        }
        
        let mut transposed = note;
        
        // Shift down to range
        while transposed > BANJO_SWEET_MAX {
            transposed -= 12;
        }
        
        // Shift up if too low
        while transposed < BANJO_MIN_NOTE {
            transposed += 12;
        }
        
        // Clamp to valid range
        if transposed >= BANJO_MIN_NOTE && transposed <= BANJO_MAX_NOTE {
            Some(transposed)
        } else {
            None
        }
    }
    
    /// Get next melody note from chord/input
    fn get_melody_note(&mut self) -> Option<u8> {
        if self.active_input_notes.is_empty() {
            return None;
        }
        
        let melodic = self.params.melodic.value() as f64;
        let rand = ((self.pattern_position as u64 * 2654435761 + self.current_sample) % 100) as f64 / 100.0;
        
        // For sparse/melodic styles, prefer highest note
        let base_note = if self.params.style.value() == PickingStyle::MelodicRun
            || self.params.style.value() == PickingStyle::Sparse {
            *self.active_input_notes.last()?
        } else {
            // For patterns, use various chord tones
            let idx = (rand * self.active_input_notes.len() as f64) as usize % self.active_input_notes.len();
            self.active_input_notes[idx]
        };
        
        let mut note = self.transpose_to_banjo(base_note)?;
        
        // Add melodic movement (passing tones)
        if melodic > 0.5 && rand > 0.7 {
            if let Some(last) = self.last_melody_note {
                let interval = (note as i16 - last as i16).abs();
                // Add chromatic passing tone for larger intervals
                if interval >= 3 && interval <= 5 {
                    if note > last {
                        note = (last + (interval / 2) as u8).min(BANJO_MAX_NOTE);
                    } else {
                        note = (last.saturating_sub((interval / 2) as u8)).max(BANJO_MIN_NOTE);
                    }
                }
            }
        }
        
        Some(note)
    }
    
    /// Determine if we should add drone string
    fn should_add_drone(&self) -> bool {
        let drone_param = self.params.drone.value() as f64;
        let rand = ((self.pattern_position as u64 * 1103515245 + self.current_sample) % 100) as f64 / 100.0;
        
        // Drone is more common in sparse styles
        let style_multiplier = match self.params.style.value() {
            PickingStyle::Sparse => 1.5,
            PickingStyle::Clawhammer => 1.2,
            PickingStyle::MelodicRun => 0.8,
            _ => 1.0,
        };
        
        rand < (drone_param * style_multiplier).min(1.0)
    }
    
    /// Determine articulation for note transition
    fn get_articulation(&self, new_note: u8) -> (bool, bool) {
        // Returns (use_hammer_on, use_slide)
        let artic = self.params.articulation.value() as f64;
        
        if let Some(last) = self.last_melody_note {
            let interval = (new_note as i16 - last as i16).abs();
            let rand = ((new_note as u64 * 7919 + self.current_sample) % 100) as f64 / 100.0;
            
            // Hammer-on for small ascending intervals
            let use_hammer = interval >= 1 && interval <= 3 
                && new_note > last 
                && artic > 0.5 
                && rand > 0.6;
            
            // Slide for medium intervals
            let use_slide = interval >= 3 && interval <= 7 
                && artic > 0.6 
                && rand > 0.7;
            
            (use_hammer, use_slide)
        } else {
            (false, false)
        }
    }
    
    /// Get pick timing based on style and parameters
    fn get_pick_timing(&mut self, style: PickingStyle) -> f64 {
        let density = self.params.density.value() as f64;
        let sparseness = self.params.sparseness.value() as f64;
        
        let base_timing = match style {
            PickingStyle::Melody => 1.0,          // Whole beat (was way too fast at 0.5!)
            PickingStyle::Clawhammer => 0.5,      // Half beat (was insane at 0.25!)
            PickingStyle::ForwardRoll => 0.5,     // Half beat for alt-country (not rapid bluegrass)
            PickingStyle::Alternating => 0.75,    // Three-quarter beat (slower, contemplative)
            PickingStyle::Sparse => 2.0 + sparseness, // 2+ beats - very sparse (lonesome prairie)
            PickingStyle::MelodicRun => 0.5 + (1.0 - density) * 0.5,  // Half to whole notes
        };
        
        // Sparseness adds space
        let timing = base_timing * (1.0 + sparseness * 0.5);
        
        // Density can speed things up
        timing * (1.0 - density * 0.3).max(0.5)
    }
    
    /// Handle note on
    fn handle_note_on(&mut self, note: u8, _velocity: u8) {
        if !self.active_input_notes.contains(&note) {
            self.active_input_notes.push(note);
            self.active_input_notes.sort();
        }
        
        // Update chord
        self.chord_notes = self.active_input_notes.clone();
        
        // Reset pattern
        self.pattern_position = 0;
        self.beat_position = 0.0;
        self.next_pick_sample = self.current_sample;
    }
    
    /// Handle note off
    fn handle_note_off(&mut self, note: u8, context: &mut impl ProcessContext<Self>) {
        self.active_input_notes.retain(|&n| n != note);
        
        // If all notes released, stop
        if self.active_input_notes.is_empty() {
            if let Some(melody) = self.current_melody_note {
                context.send_event(NoteEvent::NoteOff {
                    timing: 0,
                    voice_id: None,
                    channel: 0,
                    note: melody,
                    velocity: 0.0,
                });
                self.current_melody_note = None;
            }
            
            if self.drone_active {
                context.send_event(NoteEvent::NoteOff {
                    timing: 0,
                    voice_id: None,
                    channel: 0,
                    note: BANJO_DRONE_NOTE,
                    velocity: 0.0,
                });
                self.drone_active = false;
            }
            
            self.chord_notes.clear();
        }
    }
    
    /// Process picking pattern
    fn process_picking(&mut self, samples: u32, context: &mut impl ProcessContext<Self>) {
        if self.active_input_notes.is_empty() {
            return;
        }
        
        let style = self.params.style.value();
        
        for i in 0..samples {
            let sample_time = self.current_sample + i as u64;
            
            // Check if it's time for next pick
            if sample_time >= self.next_pick_sample {
                // Stop previous melody note
                if let Some(current) = self.current_melody_note {
                    context.send_event(NoteEvent::NoteOff {
                        timing: i,
                        voice_id: None,
                        channel: 0,
                        note: current,
                        velocity: 0.0,
                    });
                }
                
                // Generate next melody note
                if let Some(melody_note) = self.get_melody_note() {
                    let (use_hammer, use_slide) = self.get_articulation(melody_note);
                    
                    // Determine velocity based on articulation
                    let base_vel = if use_hammer {
                        0.5  // Softer for hammer-on
                    } else if use_slide {
                        0.65 // Medium for slide
                    } else {
                        0.75 + ((sample_time % 13) as f32 / 100.0)  // Normal with variation
                    };
                    
                    // Send melody note
                    context.send_event(NoteEvent::NoteOn {
                        timing: i,
                        voice_id: None,
                        channel: 0,
                        note: melody_note,
                        velocity: base_vel,
                    });
                    
                    self.current_melody_note = Some(melody_note);
                    self.last_melody_note = Some(melody_note);
                    self.note_history.push((melody_note, self.beat_position));
                    
                    // Trim history
                    if self.note_history.len() > 12 {
                        self.note_history.remove(0);
                    }
                }
                
                // Add drone string if appropriate
                if self.should_add_drone() && !self.drone_active {
                    context.send_event(NoteEvent::NoteOn {
                        timing: i,
                        voice_id: None,
                        channel: 0,
                        note: BANJO_DRONE_NOTE,
                        velocity: 0.4,  // Quieter drone
                    });
                    self.drone_active = true;
                } else if !self.should_add_drone() && self.drone_active {
                    // Stop drone if no longer needed
                    context.send_event(NoteEvent::NoteOff {
                        timing: i,
                        voice_id: None,
                        channel: 0,
                        note: BANJO_DRONE_NOTE,
                        velocity: 0.0,
                    });
                    self.drone_active = false;
                }
                
                // Calculate next pick timing
                let next_timing = self.get_pick_timing(style);
                let samples_until_next = (next_timing * self.samples_per_beat) as u64;
                self.next_pick_sample = sample_time + samples_until_next;
                
                self.pattern_position += 1;
                self.beat_position += next_timing;
            }
        }
    }
    
    fn update_tempo(&mut self, bpm: f64) {
        self.samples_per_beat = (60.0 / bpm) * self.sample_rate;
    }
}

impl Plugin for LonesomePicker {
    const NAME: &'static str = "Lonesome Picker";
    const VENDOR: &'static str = "Audio Forge RS";
    const URL: &'static str = "https://github.com/audio-forge-rs/plugins";
    const EMAIL: &'static str = "info@audio-forge-rs.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: None,
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;
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
        self.sample_rate = buffer_config.sample_rate as f64;
        self.update_tempo(120.0); // Default tempo, will be updated from DAW
        true
    }

    fn reset(&mut self) {
        self.active_input_notes.clear();
        self.chord_notes.clear();
        self.current_melody_note = None;
        self.last_melody_note = None;
        self.drone_active = false;
        self.pattern_position = 0;
        self.pick_timer = 0.0;
        self.beat_position = 0.0;
        self.phrase_notes.clear();
        self.note_history.clear();
        self.current_sample = 0;
        self.next_pick_sample = 0;
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Get transport info from DAW
        let transport = context.transport();
        let tempo = transport.tempo.unwrap_or(120.0);
        let playing = transport.playing;
        
        // Update tempo from DAW
        self.update_tempo(tempo);
        
        // Process incoming MIDI
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, velocity, .. } => {
                    let vel_u8 = (velocity * 127.0) as u8;
                    self.handle_note_on(note, vel_u8);
                }
                NoteEvent::NoteOff { note, .. } => {
                    self.handle_note_off(note, context);
                }
                _ => {}  // Drop other MIDI
            }
        }
        
        // Generate banjo picking pattern only if DAW is playing
        if playing {
            let samples = _buffer.samples() as u32;
            self.process_picking(samples, context);
            self.current_sample += samples as u64;
        }
        
        ProcessStatus::Normal
    }
}

impl ClapPlugin for LonesomePicker {
    const CLAP_ID: &'static str = "com.audio-forge-rs.lonesome-picker";
    const CLAP_DESCRIPTION: Option<&'static str> = 
        Some("Intelligent banjo MIDI processor for Ample Ethno Banjo - Alt-country picking");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::NoteEffect,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for LonesomePicker {
    const VST3_CLASS_ID: [u8; 16] = *b"AFLonesomePickXX";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Tools,
    ];
}

nih_export_clap!(LonesomePicker);
nih_export_vst3!(LonesomePicker);
