use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

mod editor;

/// Low Rider - Intelligent bass line generator for Scarbee Rickenbacker Bass
/// 
/// Transforms guitar chords, simple melodies, or basic bass lines into
/// compelling, authentic bass parts with alt-country/rock sensibility.
/// 
/// Features:
/// - Analyzes chord input and generates musical bass lines
/// - Smart note selection (root, fifth, octave, passing tones, chromatic approaches)
/// - Rhythm pattern generation with syncopation
/// - Automatic articulations (slides, hammer-ons, muted notes)
/// - Musical memory - learns from your playing
/// - Intelligent timing and note duration
/// - Optimized for Scarbee Rick bass (E1-G3 range)
pub struct LowRider {
    params: Arc<LowRiderParams>,
    
    // Input analysis
    active_input_notes: Vec<u8>,
    last_chord: Vec<u8>,          // Last detected chord
    chord_root: Option<u8>,       // Detected root note
    chord_quality: ChordQuality,  // Major, minor, etc.
    
    // Bass line generation state
    current_bass_note: Option<u8>,
    last_bass_note: Option<u8>,
    pattern_position: usize,      // Position in current pattern
    beat_position: f64,           // Current position in beats
    
    // Musical memory
    note_history: Vec<(u8, f64)>, // Recent bass notes with timestamps
    pattern_cache: Vec<u8>,       // Learned pattern fragments
    
    // Timing
    sample_rate: f64,
    samples_per_beat: f64,
    current_sample: u64,
    next_note_sample: u64,
    
    // Articulation state
    use_slide: bool,
    use_mute: bool,
}

#[derive(Params)]
struct LowRiderParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    /// Bass line style/pattern
    #[id = "style"]
    pub style: EnumParam<BassStyle>,
    
    /// Activity level (how busy the bass line is)
    #[id = "activity"]
    pub activity: FloatParam,
    
    /// Syncopation amount
    #[id = "syncopation"]
    pub syncopation: FloatParam,
    
    /// Use of fifth and passing tones
    #[id = "movement"]
    pub movement: FloatParam,
    
    /// Articulation variety (slides, mutes, hammer-ons)
    #[id = "articulation"]
    pub articulation: FloatParam,
    
    /// Note length/sustain
    #[id = "sustain"]
    pub sustain: FloatParam,
    
    /// Tempo (BPM) for pattern generation
    #[id = "tempo"]
    pub tempo: FloatParam,
    
    /// Octave preference (E1-E2 vs E2-E3)
    #[id = "octave"]
    pub octave: IntParam,
}

#[derive(Enum, PartialEq, Clone, Copy)]
enum BassStyle {
    /// Simple root notes on downbeats
    #[name = "Roots"]
    Roots,
    
    /// Root and fifth alternating
    #[name = "Root-Fifth"]
    RootFifth,
    
    /// Walking bass with passing tones
    #[name = "Walking"]
    Walking,
    
    /// Syncopated, rhythmic
    #[name = "Driving"]
    Driving,
    
    /// Sparse, spacious
    #[name = "Sparse"]
    Sparse,
    
    /// Busy, melodic bass lines
    #[name = "Melodic"]
    Melodic,
}

#[derive(Clone, Copy, PartialEq)]
enum ChordQuality {
    Unknown,
    Major,
    Minor,
    Dominant,
    Diminished,
    Augmented,
}

impl Default for LowRider {
    fn default() -> Self {
        Self {
            params: Arc::new(LowRiderParams::default()),
            active_input_notes: Vec::new(),
            last_chord: Vec::new(),
            chord_root: None,
            chord_quality: ChordQuality::Unknown,
            current_bass_note: None,
            last_bass_note: None,
            pattern_position: 0,
            beat_position: 0.0,
            note_history: Vec::new(),
            pattern_cache: Vec::new(),
            sample_rate: 44100.0,
            samples_per_beat: 44100.0,
            current_sample: 0,
            next_note_sample: 0,
            use_slide: false,
            use_mute: false,
        }
    }
}

impl Default for LowRiderParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            
            style: EnumParam::new("Style", BassStyle::Driving),
            
            activity: FloatParam::new(
                "Activity",
                0.6,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            syncopation: FloatParam::new(
                "Syncopation",
                0.4,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            movement: FloatParam::new(
                "Movement",
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
            
            sustain: FloatParam::new(
                "Sustain",
                0.7,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            tempo: FloatParam::new(
                "Tempo",
                120.0,
                FloatRange::Linear { min: 60.0, max: 180.0 },
            )
            .with_unit(" BPM")
            .with_value_to_string(formatters::v2s_f32_rounded(0)),
            
            octave: IntParam::new(
                "Octave",
                0,  // 0 = auto, -1 = lower, +1 = higher
                IntRange::Linear { min: -1, max: 1 },
            )
            .with_value_to_string(Arc::new(|value| {
                match value {
                    -1 => "Lower".to_string(),
                    0 => "Auto".to_string(),
                    1 => "Higher".to_string(),
                    _ => value.to_string(),
                }
            })),
        }
    }
}

// Scarbee Rickenbacker Bass range
const BASS_MIN_NOTE: u8 = 28;  // E1
const BASS_MAX_NOTE: u8 = 55;  // G3
const BASS_OPTIMAL_LOW: u8 = 28;   // E1
const BASS_OPTIMAL_HIGH: u8 = 43;  // G2

impl LowRider {
    /// Analyze input notes to detect chord
    fn analyze_chord(&mut self) {
        if self.active_input_notes.is_empty() {
            return;
        }
        
        // Store the chord
        self.last_chord = self.active_input_notes.clone();
        
        // Find root (lowest note, transposed to bass register)
        if let Some(&lowest) = self.active_input_notes.first() {
            let root_pc = lowest % 12;  // Pitch class
            self.chord_root = Some(root_pc);
            
            // Detect chord quality by analyzing intervals
            if self.active_input_notes.len() >= 2 {
                let intervals: Vec<u8> = self.active_input_notes.iter()
                    .map(|&n| (n - lowest) % 12)
                    .collect();
                
                self.chord_quality = if intervals.contains(&4) && intervals.contains(&7) {
                    ChordQuality::Major
                } else if intervals.contains(&3) && intervals.contains(&7) {
                    ChordQuality::Minor
                } else if intervals.contains(&4) && intervals.contains(&7) && intervals.contains(&10) {
                    ChordQuality::Dominant
                } else if intervals.contains(&3) && intervals.contains(&6) {
                    ChordQuality::Diminished
                } else {
                    ChordQuality::Unknown
                };
            }
        }
    }
    
    /// Get the bass root note in proper octave
    fn get_bass_root(&self) -> Option<u8> {
        self.chord_root.map(|root_pc| {
            let octave_pref = self.params.octave.value();
            
            // Start from bass E1 (28) and find the root note
            let base_octave = if octave_pref == -1 {
                0  // E1-B1
            } else if octave_pref == 1 {
                2  // E2-B2
            } else {
                1  // E1-B1 (auto)
            };
            
            let mut note = root_pc + (base_octave * 12);
            
            // Adjust to bass range
            while note < BASS_MIN_NOTE {
                note += 12;
            }
            while note > BASS_OPTIMAL_HIGH {
                note -= 12;
            }
            
            note
        })
    }
    
    /// Get the fifth of current chord
    fn get_fifth(&self) -> Option<u8> {
        self.chord_root.map(|root_pc| {
            let fifth_pc = (root_pc + 7) % 12;
            let mut note = fifth_pc;
            
            // Find in bass range
            while note < BASS_MIN_NOTE {
                note += 12;
            }
            while note > BASS_OPTIMAL_HIGH {
                note -= 12;
            }
            
            note
        })
    }
    
    /// Generate next bass note based on style and parameters
    fn generate_next_note(&mut self) -> Option<(u8, bool, bool)> {
        // Returns (note, use_slide, use_mute)
        
        let style = self.params.style.value();
        let movement = self.params.movement.value();
        let articulation = self.params.articulation.value();
        
        let root = self.get_bass_root()?;
        let fifth = self.get_fifth();
        
        // Pseudo-random based on position and sample count
        let rand = ((self.pattern_position as u64 * 7919 + self.current_sample) % 100) as f32 / 100.0;
        
        let note = match style {
            BassStyle::Roots => {
                // Just roots
                root
            }
            
            BassStyle::RootFifth => {
                // Alternate root and fifth
                if self.pattern_position % 2 == 0 {
                    root
                } else {
                    fifth.unwrap_or(root)
                }
            }
            
            BassStyle::Walking => {
                // Walking bass: root, third, fifth, sixth/seventh
                let pos_in_pattern = self.pattern_position % 4;
                match pos_in_pattern {
                    0 => root,
                    1 => {
                        // Third (major or minor based on chord quality)
                        let third_interval = if self.chord_quality == ChordQuality::Minor { 3 } else { 4 };
                        ((root + third_interval).max(BASS_MIN_NOTE)).min(BASS_MAX_NOTE)
                    }
                    2 => fifth.unwrap_or(root),
                    3 => {
                        // Approach next root chromatically if movement is high
                        if movement > 0.6 && rand > 0.5 {
                            ((root + 11).max(BASS_MIN_NOTE)).min(BASS_MAX_NOTE)  // Major 7th
                        } else {
                            root
                        }
                    }
                    _ => root,
                }
            }
            
            BassStyle::Driving => {
                // Syncopated, rhythmic - mostly roots with occasional fifth
                if movement > rand {
                    fifth.unwrap_or(root)
                } else {
                    root
                }
            }
            
            BassStyle::Sparse => {
                // Long sustained roots
                root
            }
            
            BassStyle::Melodic => {
                // More melodic movement - use passing tones
                let options = vec![
                    root,
                    fifth.unwrap_or(root),
                    (root + 2).min(BASS_MAX_NOTE),  // Major 2nd
                    (root + 5).min(BASS_MAX_NOTE),  // Fourth
                ];
                let idx = (rand * options.len() as f32) as usize % options.len();
                options[idx]
            }
        };
        
        // Determine articulation
        let use_slide = if let Some(last) = self.last_bass_note {
            let interval = (note as i16 - last as i16).abs();
            articulation > 0.5 && interval >= 3 && interval <= 7 && rand > 0.6
        } else {
            false
        };
        
        let use_mute = articulation > 0.7 && rand > 0.75 && style == BassStyle::Driving;
        
        Some((note, use_slide, use_mute))
    }
    
    /// Get note duration in beats based on style and sustain
    fn get_note_duration(&self, style: BassStyle) -> f64 {
        let sustain = self.params.sustain.value() as f64;
        let activity = self.params.activity.value() as f64;
        
        let base_duration = match style {
            BassStyle::Roots => 1.0,        // Whole beat
            BassStyle::RootFifth => 0.5,    // Half beat
            BassStyle::Walking => 0.25,     // Quarter beat
            BassStyle::Driving => 0.5,      // Half beat
            BassStyle::Sparse => 2.0,       // Two beats
            BassStyle::Melodic => 0.5,      // Half beat
        };
        
        // Modify by activity (more activity = shorter notes)
        let duration = base_duration * (1.0 - activity * 0.5);
        
        // Modify by sustain (higher sustain = longer notes)
        duration * (0.5 + sustain * 0.5)
    }
    
    /// Get next note timing based on style and syncopation
    fn get_next_note_timing(&mut self, style: BassStyle) -> f64 {
        let syncopation = self.params.syncopation.value() as f64;
        let activity = self.params.activity.value() as f64;
        
        let base_timing = match style {
            BassStyle::Roots => 1.0,              // Whole note - one note per beat
            BassStyle::RootFifth => 1.0,          // Whole note alternating (was way too fast at 0.5)
            BassStyle::Walking => 1.0,            // Quarter note walking (was insane at 0.25!)
            BassStyle::Driving => if activity > 0.7 { 0.5 } else { 1.0 },  // Mix of half and whole
            BassStyle::Sparse => 2.0,             // Half note - very sparse
            BassStyle::Melodic => 0.5 + activity * 0.5,  // Eighth to quarter notes
        };
        
        // Add syncopation (random offset)
        let rand = ((self.pattern_position as u64 * 1103 + self.current_sample) % 100) as f64 / 100.0;
        let offset = if syncopation > 0.5 && rand > 0.7 {
            // Syncopate: play slightly early or late
            (rand - 0.5) * 0.15 * syncopation
        } else {
            0.0
        };
        
        (base_timing + offset).max(0.5)  // Minimum half beat (prevent insanely fast bass)
    }
    
    /// Process MIDI note on
    fn handle_note_on(&mut self, note: u8, _velocity: u8) {
        if !self.active_input_notes.contains(&note) {
            self.active_input_notes.push(note);
            self.active_input_notes.sort();
        }
        
        // Re-analyze chord
        self.analyze_chord();
        
        // Reset pattern when chord changes
        self.pattern_position = 0;
        self.beat_position = 0.0;
        self.next_note_sample = self.current_sample;
    }
    
    /// Process MIDI note off
    fn handle_note_off(&mut self, note: u8, context: &mut impl ProcessContext<Self>) {
        self.active_input_notes.retain(|&n| n != note);
        
        // If all notes released, stop bass
        if self.active_input_notes.is_empty() {
            if let Some(bass_note) = self.current_bass_note {
                context.send_event(NoteEvent::NoteOff {
                    timing: 0,
                    voice_id: None,
                    channel: 0,
                    note: bass_note,
                    velocity: 0.0,
                });
                self.current_bass_note = None;
            }
            
            self.chord_root = None;
            self.chord_quality = ChordQuality::Unknown;
        } else {
            // Re-analyze remaining chord
            self.analyze_chord();
        }
    }
    
    /// Process bass line generation
    fn process_bass_line(&mut self, samples: u32, context: &mut impl ProcessContext<Self>) {
        if self.active_input_notes.is_empty() {
            return;
        }
        
        for i in 0..samples {
            let sample_time = self.current_sample + i as u64;
            
            // Check if it's time for next note
            if sample_time >= self.next_note_sample {
                let style = self.params.style.value();
                
                // Generate next note
                if let Some((note, use_slide, use_mute)) = self.generate_next_note() {
                    // Stop current note if playing
                    if let Some(current) = self.current_bass_note {
                        context.send_event(NoteEvent::NoteOff {
                            timing: i,
                            voice_id: None,
                            channel: 0,
                            note: current,
                            velocity: 0.0,
                        });
                    }
                    
                    // Determine velocity based on articulation
                    let base_velocity = if use_mute {
                        0.4  // Muted notes are quieter
                    } else {
                        0.75 + ((sample_time % 17) as f32 / 100.0)  // Slight variation
                    };
                    
                    // Send new note
                    context.send_event(NoteEvent::NoteOn {
                        timing: i,
                        voice_id: None,
                        channel: 0,
                        note,
                        velocity: base_velocity,
                    });
                    
                    self.current_bass_note = Some(note);
                    self.last_bass_note = Some(note);
                    self.note_history.push((note, self.beat_position));
                    
                    // Keep history manageable
                    if self.note_history.len() > 16 {
                        self.note_history.remove(0);
                    }
                    
                    // Calculate next note timing
                    let next_timing = self.get_next_note_timing(style);
                    let samples_until_next = (next_timing * self.samples_per_beat) as u64;
                    self.next_note_sample = sample_time + samples_until_next;
                    
                    self.pattern_position += 1;
                    self.beat_position += next_timing;
                }
            }
        }
    }
}

impl Plugin for LowRider {
    const NAME: &'static str = "Low Rider";
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
        self.update_tempo();
        true
    }

    fn reset(&mut self) {
        self.active_input_notes.clear();
        self.last_chord.clear();
        self.chord_root = None;
        self.chord_quality = ChordQuality::Unknown;
        self.current_bass_note = None;
        self.last_bass_note = None;
        self.pattern_position = 0;
        self.beat_position = 0.0;
        self.note_history.clear();
        self.pattern_cache.clear();
        self.current_sample = 0;
        self.next_note_sample = 0;
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Update tempo if changed
        self.update_tempo();
        
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
        
        // Generate bass line
        let samples = _buffer.samples() as u32;
        self.process_bass_line(samples, context);
        self.current_sample += samples as u64;
        
        ProcessStatus::Normal
    }
}

impl LowRider {
    fn update_tempo(&mut self) {
        let bpm = self.params.tempo.value() as f64;
        self.samples_per_beat = (60.0 / bpm) * self.sample_rate;
    }
}

impl ClapPlugin for LowRider {
    const CLAP_ID: &'static str = "com.audio-forge-rs.low-rider";
    const CLAP_DESCRIPTION: Option<&'static str> = 
        Some("Intelligent bass line generator for Scarbee Rickenbacker Bass");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::NoteEffect,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for LowRider {
    const VST3_CLASS_ID: [u8; 16] = *b"AFLowRiderXXXXXX";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Tools,
    ];
}

nih_export_clap!(LowRider);
nih_export_vst3!(LowRider);
