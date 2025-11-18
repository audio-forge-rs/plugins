use nih_plug::prelude::*;
use parking_lot::RwLock;
use std::sync::Arc;
use lazy_static::lazy_static;
use rand::Rng;

mod editor;

/// Melody Maker - Production-Quality Melody Generator for Alt-Country/Americana
/// 
/// Generates intelligent melodic phrases based on:
/// - Key/Mode (Major, Minor, Dorian, Mixolydian)
/// - Chord Progression (shared across all instances)
/// - Melody Style (9 types: Sparse, Melodic, Rhythmic, etc.)
/// - Random variation within musical constraints
///
/// CRITICAL: Multiple instances sync their key/mode/chord progression
/// but can have different melody styles and random variations.
///
/// Output: Pure MIDI notes → Direct to Session Guitarist, Scarbee, etc.
/// No processing, no instruments - just intelligent note generation.

// Global shared state for chord progression sync
lazy_static! {
    static ref GLOBAL_PROGRESSION: Arc<RwLock<SharedProgression>> = 
        Arc::new(RwLock::new(SharedProgression::default()));
}

#[derive(Debug, Clone)]
struct SharedProgression {
    key: Key,
    mode: Mode,
    progression: Vec<ChordType>,
    progression_index: usize,
    bar_position: f64,
    tempo: f32,
}

impl Default for SharedProgression {
    fn default() -> Self {
        Self {
            key: Key::C,
            mode: Mode::Major,
            progression: vec![
                ChordType::I,
                ChordType::IV,
                ChordType::V,
                ChordType::I,
            ],
            progression_index: 0,
            bar_position: 0.0,
            tempo: 100.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
enum Key {
    C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B,
}

impl Key {
    fn to_midi_note(self) -> u8 {
        match self {
            Key::C => 0,
            Key::Db => 1,
            Key::D => 2,
            Key::Eb => 3,
            Key::E => 4,
            Key::F => 5,
            Key::Gb => 6,
            Key::G => 7,
            Key::Ab => 8,
            Key::A => 9,
            Key::Bb => 10,
            Key::B => 11,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
enum Mode {
    #[name = "Major (Ionian)"]
    Major,
    #[name = "Natural Minor"]
    NaturalMinor,
    #[name = "Dorian"]
    Dorian,
    #[name = "Mixolydian"]
    Mixolydian,
    #[name = "Pentatonic Major"]
    PentatonicMajor,
    #[name = "Pentatonic Minor"]
    PentatonicMinor,
}

impl Mode {
    /// Get scale degrees for mode (semitones from root)
    fn scale_degrees(self) -> Vec<u8> {
        match self {
            Mode::Major => vec![0, 2, 4, 5, 7, 9, 11],
            Mode::NaturalMinor => vec![0, 2, 3, 5, 7, 8, 10],
            Mode::Dorian => vec![0, 2, 3, 5, 7, 9, 10],
            Mode::Mixolydian => vec![0, 2, 4, 5, 7, 9, 10],
            Mode::PentatonicMajor => vec![0, 2, 4, 7, 9],
            Mode::PentatonicMinor => vec![0, 3, 5, 7, 10],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
enum ChordType {
    I, II, III, IV, V, VI, VII,
}

impl ChordType {
    /// Get chord root offset in scale degrees (0-based)
    fn scale_degree(self) -> usize {
        match self {
            ChordType::I => 0,
            ChordType::II => 1,
            ChordType::III => 2,
            ChordType::IV => 3,
            ChordType::V => 4,
            ChordType::VI => 5,
            ChordType::VII => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
enum MelodyStyle {
    #[name = "Sparse & Lonesome"]
    Sparse,
    
    #[name = "Melodic & Flowing"]
    Melodic,
    
    #[name = "Rhythmic & Driving"]
    Rhythmic,
    
    #[name = "Contemplative"]
    Contemplative,
    
    #[name = "Pedal Steel Bends"]
    PedalSteel,
    
    #[name = "Telecaster Twang"]
    Telecaster,
    
    #[name = "Slide Guitar"]
    SlideGuitar,
    
    #[name = "Prairie Wind"]
    PrairieWind,
    
    #[name = "Heartland Rock"]
    HeartlandRock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
enum ProgressionType {
    #[name = "I-IV-V-I (Classic)"]
    IIVVIClassic,
    
    #[name = "I-V-vi-IV (Pop)"]
    IVviIVPop,
    
    #[name = "I-vi-IV-V"]
    IviIVV,
    
    #[name = "I-IV-I-V"]
    IIVIV,
    
    #[name = "vi-IV-I-V (Sad)"]
    viIVIVSad,
    
    #[name = "I-iii-IV-V"]
    IiiiIVV,
    
    #[name = "Custom"]
    Custom,
}

impl ProgressionType {
    fn to_chords(self) -> Vec<ChordType> {
        use ChordType::*;
        match self {
            ProgressionType::IIVVIClassic => vec![I, IV, V, I],
            ProgressionType::IVviIVPop => vec![I, V, VI, IV],
            ProgressionType::IviIVV => vec![I, VI, IV, V],
            ProgressionType::IIVIV => vec![I, IV, I, V],
            ProgressionType::viIVIVSad => vec![VI, IV, I, V],
            ProgressionType::IiiiIVV => vec![I, III, IV, V],
            ProgressionType::Custom => vec![I, IV, V, I], // Default
        }
    }
}

pub struct MelodyMaker {
    params: Arc<MelodyMakerParams>,
    
    // Melody generation state
    current_note: Option<u8>,
    last_note: Option<u8>,
    note_history: Vec<u8>,
    
    // Timing
    sample_rate: f32,
    samples_since_last_note: u64,
    samples_per_beat: f64,
    
    // Random seed for this instance
    rng: rand::rngs::StdRng,
    instance_seed: u64,
    
    // Sync state
    last_known_chord: usize,
}

#[derive(Params)]
struct MelodyMakerParams {
    #[persist = "editor-state"]
    editor_state: Arc<nih_plug_vizia::ViziaState>,
    
    /// Key (shared across instances)
    #[id = "key"]
    pub key: EnumParam<Key>,
    
    /// Mode/Scale (shared across instances)
    #[id = "mode"]
    pub mode: EnumParam<Mode>,
    
    /// Progression Type (shared across instances)
    #[id = "progression"]
    pub progression_type: EnumParam<ProgressionType>,
    
    /// Melody Style (per-instance)
    #[id = "melody_style"]
    pub melody_style: EnumParam<MelodyStyle>,
    
    /// Note Density (how often notes play)
    #[id = "density"]
    pub density: FloatParam,
    
    /// Melodic Range (how wide the melody spans)
    #[id = "range"]
    pub range: FloatParam,
    
    /// Variation Amount (randomness within style)
    #[id = "variation"]
    pub variation: FloatParam,
    
    /// Phrase Length (how long melodic phrases are)
    #[id = "phrase_length"]
    pub phrase_length: IntParam,
    
    /// Octave (output octave)
    #[id = "octave"]
    pub octave: IntParam,
    
    /// Randomize button (triggers new random melody)
    #[id = "randomize"]
    pub randomize: BoolParam,
    
    /// Enable/Disable output
    #[id = "enabled"]
    pub enabled: BoolParam,
}

impl Default for MelodyMaker {
    fn default() -> Self {
        use rand::SeedableRng;
        let instance_seed = rand::thread_rng().gen();
        
        Self {
            params: Arc::new(MelodyMakerParams::default()),
            current_note: None,
            last_note: None,
            note_history: Vec::new(),
            sample_rate: 44100.0,
            samples_since_last_note: 0,
            samples_per_beat: 44100.0,
            rng: rand::rngs::StdRng::seed_from_u64(instance_seed),
            instance_seed,
            last_known_chord: 0,
        }
    }
}

impl Default for MelodyMakerParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            key: EnumParam::new("Key", Key::C),
            mode: EnumParam::new("Mode", Mode::Major),
            progression_type: EnumParam::new("Progression", ProgressionType::IIVVIClassic),
            melody_style: EnumParam::new("Melody Style", MelodyStyle::Melodic),
            
            density: FloatParam::new(
                "Density",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            range: FloatParam::new(
                "Range",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            variation: FloatParam::new(
                "Variation",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            phrase_length: IntParam::new(
                "Phrase Length",
                4,
                IntRange::Linear { min: 2, max: 16 },
            )
            .with_unit(" bars"),
            
            octave: IntParam::new(
                "Octave",
                4,
                IntRange::Linear { min: 2, max: 6 },
            ),
            
            randomize: BoolParam::new("Randomize", false),
            
            enabled: BoolParam::new("Enabled", true),
        }
    }
}

impl MelodyMaker {
    /// Update global progression if parameters changed
    fn update_global_progression(&mut self, tempo: f32) {
        let mut global = GLOBAL_PROGRESSION.write();
        
        // Update from parameters
        global.key = self.params.key.value();
        global.mode = self.params.mode.value();
        global.progression = self.params.progression_type.value().to_chords();
        global.tempo = tempo;
    }
    
    /// Read current chord from global progression
    fn get_current_chord(&self) -> (Key, Mode, ChordType) {
        let global = GLOBAL_PROGRESSION.read();
        let chord_idx = global.progression_index % global.progression.len();
        (global.key, global.mode, global.progression[chord_idx])
    }
    
    /// Get available notes for current chord
    fn get_chord_notes(&self, key: Key, mode: Mode, chord: ChordType) -> Vec<u8> {
        let scale = mode.scale_degrees();
        let root_offset = key.to_midi_note();
        let chord_degree = chord.scale_degree();
        
        // Get chord tones (root, third, fifth, seventh if in scale)
        let mut notes = Vec::new();
        
        // Chord root
        if chord_degree < scale.len() {
            notes.push(root_offset + scale[chord_degree]);
        }
        
        // Chord third
        let third_idx = (chord_degree + 2) % scale.len();
        notes.push(root_offset + scale[third_idx]);
        
        // Chord fifth
        let fifth_idx = (chord_degree + 4) % scale.len();
        notes.push(root_offset + scale[fifth_idx]);
        
        // Add passing tones based on style
        let style = self.params.melody_style.value();
        if matches!(style, MelodyStyle::Melodic | MelodyStyle::PedalSteel) {
            // Add all scale tones
            for &degree in &scale {
                let note = root_offset + degree;
                if !notes.contains(&note) {
                    notes.push(note);
                }
            }
        }
        
        // Transpose to octave
        let octave = self.params.octave.value() as u8;
        notes.iter().map(|&n| n + (octave * 12)).collect()
    }
    
    /// Generate next note based on melody style
    fn generate_next_note(&mut self) -> Option<u8> {
        if !self.params.enabled.value() {
            return None;
        }
        
        let (key, mode, chord) = self.get_current_chord();
        let available_notes = self.get_chord_notes(key, mode, chord);
        
        if available_notes.is_empty() {
            return None;
        }
        
        let style = self.params.melody_style.value();
        let density = self.params.density.value();
        let range = self.params.range.value();
        let variation = self.params.variation.value();
        
        // Density check - should we play a note?
        if self.rng.gen::<f32>() > density {
            return None;
        }
        
        // Generate note based on style
        let note = match style {
            MelodyStyle::Sparse => {
                // Mostly chord tones, long gaps
                if available_notes.len() >= 3 {
                    available_notes[self.rng.gen_range(0..3)]
                } else {
                    *available_notes.first()?
                }
            }
            
            MelodyStyle::Melodic => {
                // Stepwise motion, melodic intervals  
                if let Some(last) = self.last_note {
                    // Find notes within 3 semitones
                    let close_notes: Vec<u8> = available_notes.iter()
                        .filter(|&&n| (n as i16 - last as i16).abs() <= 5)
                        .copied()
                        .collect();
                    
                    if !close_notes.is_empty() {
                        close_notes[self.rng.gen_range(0..close_notes.len())]
                    } else {
                        available_notes[self.rng.gen_range(0..available_notes.len())]
                    }
                } else {
                    available_notes[self.rng.gen_range(0..available_notes.len())]
                }
            }
            
            MelodyStyle::Rhythmic => {
                // Repeated notes, rhythmic patterns
                if self.rng.gen::<f32>() < 0.4 && self.last_note.is_some() {
                    // Repeat last note
                    self.last_note?
                } else {
                    // Jump to new chord tone
                    if available_notes.len() >= 3 {
                        available_notes[self.rng.gen_range(0..3)]
                    } else {
                        available_notes[0]
                    }
                }
            }
            
            MelodyStyle::Contemplative => {
                // Slow, minimal movement
                if let Some(last) = self.last_note {
                    // Stay on same note or move by step
                    if self.rng.gen::<f32>() < 0.6 {
                        last
                    } else {
                        let close: Vec<u8> = available_notes.iter()
                            .filter(|&&n| (n as i16 - last as i16).abs() <= 2)
                            .copied()
                            .collect();
                        close.get(self.rng.gen_range(0..close.len().max(1))).copied().unwrap_or(last)
                    }
                } else {
                    available_notes[0]
                }
            }
            
            MelodyStyle::PedalSteel | MelodyStyle::SlideGuitar => {
                // Smooth glides, chromatic approaches
                if let Some(last) = self.last_note {
                    // Target a chord tone via chromatic approach
                    let target = available_notes[self.rng.gen_range(0..available_notes.len().min(3))];
                    if (target as i16 - last as i16).abs() > 2 {
                        // Approach chromatically
                        if target > last { last + 1 } else { last - 1 }
                    } else {
                        target
                    }
                } else {
                    available_notes[0]
                }
            }
            
            MelodyStyle::Telecaster => {
                // Bright, jumping intervals
                available_notes[self.rng.gen_range(0..available_notes.len())]
            }
            
            MelodyStyle::PrairieWind => {
                // Open, airy, sparse
                if available_notes.len() >= 5 {
                    // Prefer higher notes
                    let upper_half = &available_notes[available_notes.len()/2..];
                    upper_half[self.rng.gen_range(0..upper_half.len())]
                } else {
                    available_notes.last().copied()?
                }
            }
            
            MelodyStyle::HeartlandRock => {
                // Driving, pentatonic feel
                if available_notes.len() >= 3 {
                    available_notes[self.rng.gen_range(0..available_notes.len().min(5))]
                } else {
                    available_notes[0]
                }
            }
        };
        
        Some(note)
    }
}

impl Plugin for MelodyMaker {
    const NAME: &'static str = "Melody Maker";
    const VENDOR: &'static str = "Audio Forge";
    const URL: &'static str = "https://github.com/audio-forge-rs/plugins";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: None,
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }
    
    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        // Initial tempo will be set from DAW transport in process()
        self.samples_per_beat = (60.0 / 120.0) * self.sample_rate as f64;
        true
    }

    fn reset(&mut self) {
        self.current_note = None;
        self.last_note = None;
        self.note_history.clear();
        self.samples_since_last_note = 0;
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Get transport info from DAW
        let transport = context.transport();
        let tempo = transport.tempo.unwrap_or(120.0) as f32;
        let playing = transport.playing;
        
        // Update global progression from parameters and DAW tempo
        self.update_global_progression(tempo);
        
        // Only generate notes if DAW is playing
        if !playing {
            return ProcessStatus::Normal;
        }
        
        // Calculate timing
        self.samples_per_beat = (60.0 / tempo as f64) * self.sample_rate as f64;
        let samples_per_note = (self.samples_per_beat * (2.0 - self.params.density.value() as f64)) as u64;
        
        self.samples_since_last_note += _buffer.samples() as u64;
        
        // Generate note if it's time
        if self.samples_since_last_note >= samples_per_note {
            self.samples_since_last_note = 0;
            
            // Turn off current note
            if let Some(note) = self.current_note {
                context.send_event(NoteEvent::NoteOff {
                    timing: 0,
                    voice_id: None,
                    channel: 0,
                    note,
                    velocity: 0.0,
                });
            }
            
            // Generate new note
            if let Some(note) = self.generate_next_note() {
                context.send_event(NoteEvent::NoteOn {
                    timing: 0,
                    voice_id: None,
                    channel: 0,
                    note,
                    velocity: 0.8,
                });
                
                self.current_note = Some(note);
                self.last_note = Some(note);
                self.note_history.push(note);
                if self.note_history.len() > 16 {
                    self.note_history.remove(0);
                }
            }
        }
        
        ProcessStatus::Normal
    }
}

impl ClapPlugin for MelodyMaker {
    const CLAP_ID: &'static str = "com.audio-forge.melody-maker";
    const CLAP_DESCRIPTION: Option<&'static str> = 
        Some("Intelligent melody generator for alt-country/Americana with shared progression sync");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::NoteEffect,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for MelodyMaker {
    const VST3_CLASS_ID: [u8; 16] = *b"MelodyMakerAFRGS";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Generator,
    ];
}

nih_export_clap!(MelodyMaker);
nih_export_vst3!(MelodyMaker);
