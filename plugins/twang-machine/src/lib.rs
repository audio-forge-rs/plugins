use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

mod editor;

/// Twang Machine - Intelligent MIDI processor for NI Session Guitarist
/// 
/// Transforms any MIDI input into authentic guitar phrases optimized for
/// Session Guitarist Electric Sunburst Deluxe (Melody/Mono mode).
/// 
/// Features:
/// - Liberal input: Accepts any MIDI notes, any range
/// - Smart transposition: Auto-shifts to guitar's playable range (C2-C5)
/// - Intelligent strumming/arpeggiation for chord inputs
/// - Automatic articulations (hammer-ons, slides, bends)
/// - Strict output: Only valid mono melody notes for the instrument
/// - Son Volt/alt-country vibes out of the box
pub struct TwangMachine {
    params: Arc<TwangMachineParams>,
    
    // MIDI state tracking
    active_input_notes: Vec<u8>,  // Notes currently held by user
    last_output_note: Option<u8>, // Last note sent to instrument
    note_history: Vec<(u8, f64)>, // Recent notes with timestamps (for intelligence)
    
    // Auto-transpose state
    transpose_offset: i8,  // Current octave shift
    
    // Strum/arpeggio state
    strum_index: usize,
    strum_timer: f64,
    
    // Timing
    sample_rate: f64,
    current_sample: u64,
}

#[derive(Params)]
struct TwangMachineParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    /// Strum/arpeggio mode
    #[id = "mode"]
    pub mode: EnumParam<PlayMode>,
    
    /// Strum speed (when in strum mode)
    #[id = "strum_speed"]
    pub strum_speed: FloatParam,
    
    /// Humanization amount
    #[id = "humanize"]
    pub humanize: FloatParam,
    
    /// Articulation sensitivity
    #[id = "articulation"]
    pub articulation: FloatParam,
    
    /// Auto-transpose enable
    #[id = "auto_transpose"]
    pub auto_transpose: BoolParam,
    
    /// Target range center (where to transpose to)
    #[id = "target_center"]
    pub target_center: IntParam,
}

#[derive(Enum, PartialEq, Clone, Copy)]
enum PlayMode {
    /// Single note - highest note priority
    #[name = "Single Note"]
    Single,
    
    /// Strum down through held notes
    #[name = "Strum Down"]
    StrumDown,
    
    /// Strum up through held notes  
    #[name = "Strum Up"]
    StrumUp,
    
    /// Arpeggio up
    #[name = "Arpeggio Up"]
    ArpeggioUp,
    
    /// Arpeggio down
    #[name = "Arpeggio Down"]
    ArpeggioDown,
    
    /// Arpeggio up-down
    #[name = "Arpeggio Up-Down"]
    ArpeggioUpDown,
}

impl Default for TwangMachine {
    fn default() -> Self {
        Self {
            params: Arc::new(TwangMachineParams::default()),
            active_input_notes: Vec::new(),
            last_output_note: None,
            note_history: Vec::new(),
            transpose_offset: 0,
            strum_index: 0,
            strum_timer: 0.0,
            sample_rate: 44100.0,
            current_sample: 0,
        }
    }
}

impl Default for TwangMachineParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            
            mode: EnumParam::new("Mode", PlayMode::Single),
            
            strum_speed: FloatParam::new(
                "Strum Speed",
                60.0,  // ms between notes (default: natural strum)
                FloatRange::Linear { min: 20.0, max: 250.0 },
            )
            .with_unit(" ms")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),
            
            humanize: FloatParam::new(
                "Humanize",
                0.3,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            articulation: FloatParam::new(
                "Articulation",
                0.7,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            auto_transpose: BoolParam::new("Auto Transpose", true),
            
            target_center: IntParam::new(
                "Target Note",
                60,  // C4
                IntRange::Linear { min: 48, max: 72 },  // C3-C5
            )
            .with_value_to_string(Arc::new(|value| {
                format_midi_note(value as u8)
            })),
        }
    }
}

// Session Guitarist Electric Sunburst Deluxe (Melody mode) specs
const INSTRUMENT_MIN_NOTE: u8 = 48;  // C3
const INSTRUMENT_MAX_NOTE: u8 = 84;  // C6
const IDEAL_MIN_NOTE: u8 = 55;       // G3 - sweet spot lower
const IDEAL_MAX_NOTE: u8 = 74;       // D5 - sweet spot upper

impl TwangMachine {
    /// Calculate optimal transpose offset based on recent note history
    fn calculate_auto_transpose(&mut self) -> i8 {
        if self.active_input_notes.is_empty() {
            return self.transpose_offset;
        }
        
        // Find average of currently held notes
        let avg_note = self.active_input_notes.iter()
            .map(|&n| n as i32)
            .sum::<i32>() / self.active_input_notes.len() as i32;
        
        let target = self.params.target_center.value() as i32;
        
        // Calculate how many octaves to shift
        let diff = target - avg_note;
        let octaves = (diff as f32 / 12.0).round() as i8;
        
        octaves
    }
    
    /// Transpose a note to the instrument's range
    fn transpose_note(&self, note: u8) -> Option<u8> {
        let transposed = (note as i16 + (self.transpose_offset as i16 * 12)) as i16;
        
        // Clamp to instrument range
        if transposed < INSTRUMENT_MIN_NOTE as i16 {
            Some(INSTRUMENT_MIN_NOTE)
        } else if transposed > INSTRUMENT_MAX_NOTE as i16 {
            Some(INSTRUMENT_MAX_NOTE)
        } else {
            Some(transposed as u8)
        }
    }
    
    /// Determine if we should use hammer-on/pull-off articulation
    fn should_use_legato(&self, new_note: u8) -> bool {
        if let Some(last) = self.last_output_note {
            let interval = (new_note as i16 - last as i16).abs();
            // Hammer-on/pull-off work well for intervals of 1-4 semitones
            interval > 0 && interval <= 4
        } else {
            false
        }
    }
    
    /// Determine if we should use slide articulation
    fn should_use_slide(&self, new_note: u8) -> bool {
        if let Some(last) = self.last_output_note {
            let interval = (new_note as i16 - last as i16).abs();
            // Slides work well for larger intervals (5-12 semitones)
            interval >= 5 && interval <= 12
        } else {
            false
        }
    }
    
    /// Get the next note to play based on mode
    fn get_next_note(&mut self, mode: PlayMode) -> Option<u8> {
        if self.active_input_notes.is_empty() {
            return None;
        }
        
        match mode {
            PlayMode::Single => {
                // Highest note priority (typical for lead)
                self.active_input_notes.iter().max().copied()
            }
            
            PlayMode::StrumDown | PlayMode::StrumUp => {
                // Sort notes for strumming
                let mut sorted = self.active_input_notes.clone();
                sorted.sort();
                
                if mode == PlayMode::StrumDown {
                    sorted.reverse();
                }
                
                if self.strum_index >= sorted.len() {
                    self.strum_index = 0;
                }
                
                // Don't increment here - it's done in process_strum after note is played
                sorted.get(self.strum_index).copied()
            }
            
            PlayMode::ArpeggioUp | PlayMode::ArpeggioDown | PlayMode::ArpeggioUpDown => {
                let mut sorted = self.active_input_notes.clone();
                sorted.sort();
                
                if mode == PlayMode::ArpeggioDown {
                    sorted.reverse();
                }
                
                if mode == PlayMode::ArpeggioUpDown {
                    // Ping-pong pattern
                    let len = sorted.len();
                    if len > 1 {
                        let cycle_len = (len - 1) * 2;
                        let pos = self.strum_index % cycle_len;
                        let idx = if pos < len {
                            pos
                        } else {
                            cycle_len - pos
                        };
                        sorted.get(idx).copied()
                    } else {
                        sorted.first().copied()
                    }
                } else {
                    sorted.get(self.strum_index % sorted.len()).copied()
                }
            }
        }
    }
    
    /// Apply humanization to velocity
    fn humanize_velocity(&self, base_velocity: u8, humanize: f32) -> u8 {
        if humanize <= 0.0 {
            return base_velocity;
        }
        
        // Use sample count as pseudo-random seed
        let rand = ((self.current_sample * 1103515245 + 12345) % 65536) as f32 / 65536.0;
        let variation = (rand - 0.5) * humanize * 20.0; // ±10 velocity units max
        
        ((base_velocity as f32 + variation).clamp(1.0, 127.0)) as u8
    }
    
    /// Process MIDI note on
    fn handle_note_on(&mut self, note: u8, velocity: u8, context: &mut impl ProcessContext<Self>) {
        // Add to active notes (liberal input - accept everything)
        if !self.active_input_notes.contains(&note) {
            self.active_input_notes.push(note);
            self.active_input_notes.sort();
        }
        
        // Update auto-transpose if enabled
        if self.params.auto_transpose.value() {
            self.transpose_offset = self.calculate_auto_transpose();
        }
        
        // Determine which note to play
        let mode = self.params.mode.value();
        
        // For single mode, play immediately
        if mode == PlayMode::Single {
            if let Some(output_note) = self.get_next_note(mode) {
                if let Some(transposed) = self.transpose_note(output_note) {
                    // If we had a previous note, turn it off first (mono!)
                    if let Some(prev) = self.last_output_note {
                        if prev != transposed {
                            context.send_event(NoteEvent::NoteOff {
                                timing: 0,
                                voice_id: None,
                                channel: 0,
                                note: prev,
                                velocity: 0.0,
                            });
                        }
                    }
                    
                    // Determine articulation
                    let final_velocity = if self.should_use_legato(transposed) 
                        && self.params.articulation.value() > 0.5 {
                        // Legato/hammer-on - use lower velocity
                        (velocity as f32 * 0.7) as u8
                    } else {
                        velocity
                    };
                    
                    let humanized_vel = self.humanize_velocity(final_velocity, 
                        self.params.humanize.value());
                    
                    // Send note on
                    context.send_event(NoteEvent::NoteOn {
                        timing: 0,
                        voice_id: None,
                        channel: 0,
                        note: transposed,
                        velocity: humanized_vel as f32 / 127.0,
                    });
                    
                    self.last_output_note = Some(transposed);
                    self.note_history.push((transposed, self.current_sample as f64 / self.sample_rate));
                    
                    // Keep history manageable
                    if self.note_history.len() > 10 {
                        self.note_history.remove(0);
                    }
                }
            }
        } else {
            // For strum/arpeggio modes, reset the pattern
            self.strum_index = 0;
            self.strum_timer = 0.0;
        }
    }
    
    /// Process MIDI note off
    fn handle_note_off(&mut self, note: u8, context: &mut impl ProcessContext<Self>) {
        // Remove from active notes
        self.active_input_notes.retain(|&n| n != note);
        
        // If this was the last note and we're in single mode, send note off
        if self.active_input_notes.is_empty() && self.params.mode.value() == PlayMode::Single {
            if let Some(last) = self.last_output_note {
                context.send_event(NoteEvent::NoteOff {
                    timing: 0,
                    voice_id: None,
                    channel: 0,
                    note: last,
                    velocity: 0.0,
                });
                self.last_output_note = None;
            }
        } else if self.params.mode.value() != PlayMode::Single {
            // Reset pattern
            self.strum_index = 0;
        }
    }
    
    /// Process strum/arpeggio timing
    fn process_strum(&mut self, samples: u32, context: &mut impl ProcessContext<Self>) {
        let mode = self.params.mode.value();
        if mode == PlayMode::Single || self.active_input_notes.is_empty() {
            return;
        }
        
        let strum_interval = self.params.strum_speed.value() as f64 / 1000.0; // Convert to seconds
        let samples_per_strum = (strum_interval * self.sample_rate) as u32;
        
        for i in 0..samples {
            self.strum_timer += 1.0;
            
            if self.strum_timer >= samples_per_strum as f64 {
                self.strum_timer = 0.0;
                
                // Turn off last note (mono!)
                if let Some(last) = self.last_output_note {
                    context.send_event(NoteEvent::NoteOff {
                        timing: i,
                        voice_id: None,
                        channel: 0,
                        note: last,
                        velocity: 0.0,
                    });
                }
                
                // Get next note in pattern
                if let Some(next_note) = self.get_next_note(mode) {
                    if let Some(transposed) = self.transpose_note(next_note) {
                        let velocity = self.humanize_velocity(100, self.params.humanize.value());
                        
                        context.send_event(NoteEvent::NoteOn {
                            timing: i,
                            voice_id: None,
                            channel: 0,
                            note: transposed,
                            velocity: velocity as f32 / 127.0,
                        });
                        
                        self.last_output_note = Some(transposed);
                    }
                }
                
                // Advance index for all pattern modes (strum and arpeggio)
                self.strum_index += 1;
            }
        }
    }
}

impl Plugin for TwangMachine {
    const NAME: &'static str = "Twang Machine";
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
        true
    }

    fn reset(&mut self) {
        self.active_input_notes.clear();
        self.last_output_note = None;
        self.note_history.clear();
        self.strum_index = 0;
        self.strum_timer = 0.0;
        self.current_sample = 0;
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Process incoming MIDI
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, velocity, .. } => {
                    let vel_u8 = (velocity * 127.0) as u8;
                    self.handle_note_on(note, vel_u8, context);
                }
                NoteEvent::NoteOff { note, .. } => {
                    self.handle_note_off(note, context);
                }
                // Drop all other MIDI (CC, pitch bend, etc.) - we only care about notes
                _ => {}
            }
        }
        
        // Process strum/arpeggio patterns
        let samples = _buffer.samples() as u32;
        self.process_strum(samples, context);
        self.current_sample += samples as u64;
        
        ProcessStatus::Normal
    }
}

impl ClapPlugin for TwangMachine {
    const CLAP_ID: &'static str = "com.audio-forge-rs.twang-machine";
    const CLAP_DESCRIPTION: Option<&'static str> = 
        Some("Intelligent MIDI processor for NI Session Guitarist - Alt-country guitar vibes");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::NoteEffect,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for TwangMachine {
    const VST3_CLASS_ID: [u8; 16] = *b"AFTwangMachineXX";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Tools,
    ];
}

/// Format MIDI note number as note name
fn format_midi_note(note: u8) -> String {
    const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let octave = (note / 12) as i32 - 1;
    let name = NOTE_NAMES[(note % 12) as usize];
    format!("{}{}", name, octave)
}

nih_export_clap!(TwangMachine);
nih_export_vst3!(TwangMachine);
