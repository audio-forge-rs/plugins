use nih_plug::prelude::*;
use std::collections::VecDeque;
use std::sync::Arc;

mod editor;

// Gospel Wheels - Intelligent Hammond Organ MIDI Processor for M-Tron Pro IV
//
// Philosophy:
// Transform any MIDI input into authentic alt-country/Americana Hammond organ parts.
// Think Son Volt, Wilco, Uncle Tupelo - NOT prog rock or gospel rave-ups.
// Sparse, atmospheric, swelling chords with rhythmic comping intelligence.
//
// M-Tron Pro IV Setup:
// - Load Hammond organ tape (B3, C3, M400, etc.)
// - Set to basic mono/poly mode (NOT pattern mode)
// - This plugin handles all musical intelligence
//
// Core Features:
// 1. Chord Detection & Voicing - Analyzes input, creates proper organ voicings
// 2. Harmonic Layering - Simulates drawbar registration (16', 8', 4', 2 2/3')
// 3. Swell Simulation - Dynamic volume swells for atmospheric parts
// 4. Rhythmic Comping - Intelligent rhythmic patterns (not just sustained)
// 5. Voice Leading - Smooth chord transitions (minimal movement)
// 6. Register Control - Low/mid/high organ ranges

const MAX_VOICE_COUNT: usize = 6; // Max simultaneous organ notes
const HISTORY_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
enum PlayStyle {
    Sustained,      // Long held chords with swells
    Comping,        // Rhythmic chord stabs
    Swell,          // Gradual dynamic swells
    Arpeggiated,    // Broken chords, not block
    Bass,           // Left-hand bass notes + right chords
    Atmospheric,    // Sparse, spacious, reverb-friendly
}

#[derive(Debug, Clone, Copy)]
struct ChordAnalysis {
    root: u8,
    quality: ChordQuality,
    notes: [Option<u8>; 7], // Available chord tones
    bass_note: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ChordQuality {
    Major,
    Minor,
    Dominant,
    Diminished,
    Augmented,
    Sus4,
    Unknown,
}

struct GospelWheels {
    params: Arc<GospelWheelsParams>,
    
    // State tracking
    held_input_notes: Vec<(u8, u8)>, // (note, velocity)
    active_output_notes: Vec<(u8, u8)>, // Currently playing output notes
    note_history: VecDeque<u8>,
    
    // Musical intelligence
    current_chord: Option<ChordAnalysis>,
    last_voicing: Vec<u8>,
    
    // Timing for rhythmic patterns
    sample_position: u64,
    samples_per_quarter: f64,
    next_event_sample: u64,
    
    // Swell state
    swell_position: f32, // 0.0 to 1.0
    swell_direction: f32, // -1 or 1
}

#[derive(Params)]
struct GospelWheelsParams {
    /// Play style - how the organ responds
    #[id = "style"]
    pub style: EnumParam<PlayStyle>,

    /// Harmonic richness - simulates drawbar registration (0% = 8' only, 100% = full drawbars)
    #[id = "harmonics"]
    pub harmonics: FloatParam,

    /// Swell amount - dynamic volume swells (0% = static, 100% = dramatic swells)
    #[id = "swell"]
    pub swell: FloatParam,

    /// Rhythmic density - for comping patterns (0% = sparse, 100% = busy)
    #[id = "rhythm"]
    pub rhythm: FloatParam,

    /// Voice leading - smooth transitions vs. fresh voicings (0% = new, 100% = smooth)
    #[id = "voicing"]
    pub voicing: FloatParam,

    /// Register - overall pitch range (0% = low, 50% = mid, 100% = high)
    #[id = "register"]
    pub register: FloatParam,

    /// Tempo for rhythmic patterns
    #[id = "tempo"]
    pub tempo: FloatParam,

    /// Auto-thicken - add harmonic layers automatically
    #[id = "thicken"]
    pub thicken: BoolParam,
}

impl Default for GospelWheels {
    fn default() -> Self {
        Self {
            params: Arc::new(GospelWheelsParams::default()),
            held_input_notes: Vec::new(),
            active_output_notes: Vec::new(),
            note_history: VecDeque::new(),
            current_chord: None,
            last_voicing: Vec::new(),
            sample_position: 0,
            samples_per_quarter: 44100.0,
            next_event_sample: 0,
            swell_position: 0.0,
            swell_direction: 1.0,
        }
    }
}

impl Default for GospelWheelsParams {
    fn default() -> Self {
        Self {
            style: EnumParam::new("Style", PlayStyle::Sustained),
            
            harmonics: FloatParam::new(
                "Harmonics",
                60.0,
                FloatRange::Linear { min: 0.0, max: 100.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),

            swell: FloatParam::new(
                "Swell",
                50.0,
                FloatRange::Linear { min: 0.0, max: 100.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),

            rhythm: FloatParam::new(
                "Rhythm",
                50.0,
                FloatRange::Linear { min: 0.0, max: 100.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),

            voicing: FloatParam::new(
                "Voicing",
                70.0,
                FloatRange::Linear { min: 0.0, max: 100.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),

            register: FloatParam::new(
                "Register",
                50.0,
                FloatRange::Linear { min: 0.0, max: 100.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),

            tempo: FloatParam::new(
                "Tempo",
                100.0,
                FloatRange::Linear { min: 60.0, max: 180.0 },
            )
            .with_unit(" BPM")
            .with_value_to_string(formatters::v2s_f32_rounded(0)),

            thicken: BoolParam::new("Auto-Thicken", true),
        }
    }
}

impl GospelWheels {
    /// Analyze held notes and determine chord quality
    fn analyze_chord(&self) -> Option<ChordAnalysis> {
        if self.held_input_notes.is_empty() {
            return None;
        }

        // Get unique pitch classes
        let mut pitches: Vec<u8> = self.held_input_notes
            .iter()
            .map(|(note, _)| note % 12)
            .collect();
        pitches.sort_unstable();
        pitches.dedup();

        if pitches.is_empty() {
            return None;
        }

        // Determine root (lowest note)
        let root = pitches[0];
        
        // Calculate intervals from root
        let intervals: Vec<u8> = pitches
            .iter()
            .map(|&p| (p + 12 - root) % 12)
            .collect();

        // Detect chord quality
        let quality = Self::detect_chord_quality(&intervals);

        // Build available chord tones based on quality
        let notes = Self::build_chord_tones(root, quality);

        // Bass note (lowest input note)
        let bass_note = self.held_input_notes
            .iter()
            .map(|(note, _)| *note)
            .min()
            .unwrap_or(root);

        Some(ChordAnalysis {
            root,
            quality,
            notes,
            bass_note,
        })
    }

    fn detect_chord_quality(intervals: &[u8]) -> ChordQuality {
        let has = |interval| intervals.contains(&interval);

        match () {
            _ if has(4) && has(7) && !has(10) => ChordQuality::Major,
            _ if has(3) && has(7) && !has(10) => ChordQuality::Minor,
            _ if has(4) && has(7) && has(10) => ChordQuality::Dominant,
            _ if has(3) && has(6) => ChordQuality::Diminished,
            _ if has(4) && has(8) => ChordQuality::Augmented,
            _ if has(5) && has(7) => ChordQuality::Sus4,
            _ => ChordQuality::Unknown,
        }
    }

    fn build_chord_tones(root: u8, quality: ChordQuality) -> [Option<u8>; 7] {
        let mut notes = [None; 7];
        notes[0] = Some(root); // Root

        match quality {
            ChordQuality::Major => {
                notes[1] = Some((root + 2) % 12);  // Major 2nd
                notes[2] = Some((root + 4) % 12);  // Major 3rd
                notes[3] = Some((root + 7) % 12);  // Perfect 5th
                notes[4] = Some((root + 9) % 12);  // Major 6th
                notes[5] = Some((root + 11) % 12); // Major 7th
            }
            ChordQuality::Minor => {
                notes[1] = Some((root + 2) % 12);  // Major 2nd
                notes[2] = Some((root + 3) % 12);  // Minor 3rd
                notes[3] = Some((root + 7) % 12);  // Perfect 5th
                notes[4] = Some((root + 8) % 12);  // Minor 6th
                notes[5] = Some((root + 10) % 12); // Minor 7th
            }
            ChordQuality::Dominant => {
                notes[1] = Some((root + 2) % 12);  // Major 2nd
                notes[2] = Some((root + 4) % 12);  // Major 3rd
                notes[3] = Some((root + 7) % 12);  // Perfect 5th
                notes[4] = Some((root + 10) % 12); // Minor 7th
            }
            ChordQuality::Sus4 => {
                notes[1] = Some((root + 2) % 12);  // Major 2nd
                notes[2] = Some((root + 5) % 12);  // Perfect 4th
                notes[3] = Some((root + 7) % 12);  // Perfect 5th
            }
            _ => {
                // Unknown - just use what we have
                notes[1] = Some((root + 7) % 12); // Perfect 5th
            }
        }

        notes
    }

    /// Generate organ voicing based on style and parameters
    fn generate_voicing(&mut self, chord: &ChordAnalysis) -> Vec<u8> {
        let style = self.params.style.value();
        let harmonics = self.params.harmonics.value() / 100.0;
        let register = self.params.register.value() / 100.0;
        let voicing_smoothness = self.params.voicing.value() / 100.0;

        let mut voicing = Vec::new();

        // Determine base octave from register parameter
        // 0% = C2-C4, 50% = C3-C5, 100% = C4-C6
        let base_octave = 24 + (register * 24.0) as u8; // MIDI note offset

        match style {
            PlayStyle::Bass => {
                // Left hand bass + right hand chord
                voicing.push(chord.bass_note.max(28).min(48)); // Bass note (E1-C3)
                
                // Right hand chord in mid register
                if let Some(third) = chord.notes[2] {
                    voicing.push(60 + third);
                }
                if let Some(fifth) = chord.notes[3] {
                    voicing.push(60 + fifth);
                }
                if harmonics > 0.6 {
                    voicing.push(60 + chord.root);
                }
            }

            PlayStyle::Sustained | PlayStyle::Swell => {
                // Full voiced chord
                let base = base_octave + chord.root;
                voicing.push(base);
                
                if let Some(third) = chord.notes[2] {
                    voicing.push(base + third - chord.root);
                }
                if let Some(fifth) = chord.notes[3] {
                    voicing.push(base + fifth - chord.root);
                }
                
                // Add upper voices if harmonics is high
                if harmonics > 0.5 {
                    voicing.push(base + 12); // Octave up
                    if let Some(third) = chord.notes[2] {
                        voicing.push(base + 12 + third - chord.root);
                    }
                }
            }

            PlayStyle::Comping | PlayStyle::Atmospheric => {
                // Sparse voicing - maybe just root and third, or root and fifth
                let base = base_octave + chord.root;
                
                if self.params.rhythm.value() > 50.0 || style == PlayStyle::Atmospheric {
                    // Sparse - root + fifth
                    voicing.push(base);
                    if let Some(fifth) = chord.notes[3] {
                        voicing.push(base + fifth - chord.root);
                    }
                } else {
                    // More complete
                    voicing.push(base);
                    if let Some(third) = chord.notes[2] {
                        voicing.push(base + third - chord.root);
                    }
                    if let Some(fifth) = chord.notes[3] {
                        voicing.push(base + fifth - chord.root);
                    }
                }
            }

            PlayStyle::Arpeggiated => {
                // Build full voicing, will be arpeggiated in event generation
                let base = base_octave + chord.root;
                voicing.push(base);
                
                if let Some(third) = chord.notes[2] {
                    voicing.push(base + third - chord.root);
                }
                if let Some(fifth) = chord.notes[3] {
                    voicing.push(base + fifth - chord.root);
                }
                if harmonics > 0.6 {
                    if let Some(seventh) = chord.notes[5] {
                        voicing.push(base + seventh - chord.root);
                    }
                }
            }
        }

        // Voice leading - try to keep notes close to last voicing
        if voicing_smoothness > 0.5 && !self.last_voicing.is_empty() {
            voicing = self.apply_voice_leading(voicing);
        }

        // Add harmonic layers (drawbar simulation) if thicken is enabled
        if self.params.thicken.value() && harmonics > 0.3 {
            voicing = self.add_harmonic_layers(voicing, harmonics);
        }

        // Clamp to MIDI range
        for note in voicing.iter_mut() {
            *note = (*note).max(21).min(108); // A0 to C8
        }

        voicing
    }

    fn apply_voice_leading(&self, mut voicing: Vec<u8>) -> Vec<u8> {
        // Try to minimize movement from last voicing
        // This is a simplified voice leading algorithm
        
        if voicing.len() != self.last_voicing.len() {
            return voicing;
        }

        // For each note in new voicing, try octave adjustments to get closer to last
        for (i, note) in voicing.iter_mut().enumerate() {
            if i >= self.last_voicing.len() {
                break;
            }
            
            let last = self.last_voicing[i];
            let current = *note;
            
            // Try octave up or down
            let options = [current, current + 12, current.saturating_sub(12)];
            
            *note = *options
                .iter()
                .min_by_key(|&&n| (n as i16 - last as i16).abs())
                .unwrap_or(&current);
        }

        voicing
    }

    fn add_harmonic_layers(&self, mut voicing: Vec<u8>, harmonics: f32) -> Vec<u8> {
        // Simulate Hammond drawbar registration
        // 16' (octave down), 8' (fundamental), 4' (octave up), 2 2/3' (fifth up)
        
        let original = voicing.clone();
        
        // 16' - octave down (if harmonics > 0.7)
        if harmonics > 0.7 {
            for &note in &original {
                if note >= 24 {
                    voicing.push(note - 12);
                }
            }
        }

        // 4' - octave up (if harmonics > 0.4)
        if harmonics > 0.4 {
            for &note in &original {
                if note <= 96 {
                    voicing.push(note + 12);
                }
            }
        }

        // 2 2/3' - fifth up (if harmonics > 0.8)
        if harmonics > 0.8 {
            for &note in &original {
                if note <= 89 {
                    voicing.push(note + 19); // Octave + fifth
                }
            }
        }

        voicing
    }

    fn calculate_swell_velocity(&mut self, base_velocity: u8) -> u8 {
        let swell_amount = self.params.swell.value() / 100.0;
        
        if swell_amount < 0.1 {
            return base_velocity;
        }

        // Swell oscillates between 0.3 and 1.0
        let min_vel = 0.3;
        let max_vel = 1.0;
        let range = max_vel - min_vel;
        
        let swell_mod = min_vel + (self.swell_position * range);
        
        (base_velocity as f32 * swell_mod).min(127.0) as u8
    }

    fn update_swell(&mut self, samples: u32, sample_rate: f32) {
        let swell_amount = self.params.swell.value() / 100.0;
        
        if swell_amount < 0.1 {
            self.swell_position = 1.0;
            return;
        }

        // Swell period based on tempo
        let tempo = self.params.tempo.value();
        let swell_period_samples = (sample_rate * 240.0 / tempo) as f32; // 4 beats
        
        let increment = samples as f32 / swell_period_samples;
        
        self.swell_position += increment * self.swell_direction * swell_amount;
        
        if self.swell_position >= 1.0 {
            self.swell_position = 1.0;
            self.swell_direction = -1.0;
        } else if self.swell_position <= 0.0 {
            self.swell_position = 0.0;
            self.swell_direction = 1.0;
        }
    }

    fn release_all_notes(&mut self, output: &mut Vec<NoteEvent<()>>) {
        for (note, _) in self.active_output_notes.drain(..) {
            output.push(NoteEvent::NoteOff {
                timing: 0,
                voice_id: None,
                channel: 0,
                note,
                velocity: 0.0,
            });
        }
    }

    fn play_voicing(&mut self, voicing: Vec<u8>, velocity: u8, output: &mut Vec<NoteEvent<()>>) {
        // Release notes not in new voicing
        let mut to_release = Vec::new();
        for &(note, _) in &self.active_output_notes {
            if !voicing.contains(&note) {
                to_release.push(note);
            }
        }
        
        for note in to_release {
            output.push(NoteEvent::NoteOff {
                timing: 0,
                voice_id: None,
                channel: 0,
                note,
                velocity: 0.0,
            });
            self.active_output_notes.retain(|(n, _)| *n != note);
        }

        // Play new notes
        let adjusted_velocity = self.calculate_swell_velocity(velocity);
        
        for &note in &voicing {
            if !self.active_output_notes.iter().any(|(n, _)| *n == note) {
                output.push(NoteEvent::NoteOn {
                    timing: 0,
                    voice_id: None,
                    channel: 0,
                    note,
                    velocity: adjusted_velocity as f32 / 127.0,
                });
                self.active_output_notes.push((note, adjusted_velocity));
            }
        }

        self.last_voicing = voicing;
    }
}

impl Plugin for GospelWheels {
    const NAME: &'static str = "Gospel Wheels";
    const VENDOR: &'static str = "Audio Forge";
    const URL: &'static str = "https://github.com/bedwards/audio-forge-rs";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
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
        editor::create(self.params.clone())
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut output_events = Vec::new();
        
        // Update timing
        let sample_rate = context.transport().sample_rate;
        let tempo = self.params.tempo.value();
        self.samples_per_quarter = sample_rate as f64 * 60.0 / tempo as f64;

        // Process input MIDI events
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, velocity, .. } => {
                    let vel = (velocity * 127.0) as u8;
                    self.held_input_notes.push((note, vel));
                    
                    // Analyze new chord
                    self.current_chord = self.analyze_chord();
                    
                    if let Some(chord) = self.current_chord.clone() {
                        let voicing = self.generate_voicing(&chord);
                        self.play_voicing(voicing, vel, &mut output_events);
                    }
                    
                    // Update history
                    self.note_history.push_back(note);
                    if self.note_history.len() > HISTORY_SIZE {
                        self.note_history.pop_front();
                    }
                }
                
                NoteEvent::NoteOff { note, .. } => {
                    self.held_input_notes.retain(|(n, _)| *n != note);
                    
                    if self.held_input_notes.is_empty() {
                        // All notes released - release all output
                        self.release_all_notes(&mut output_events);
                        self.current_chord = None;
                    } else {
                        // Re-analyze remaining chord
                        self.current_chord = self.analyze_chord();
                        
                        if let Some(chord) = self.current_chord.clone() {
                            let avg_vel = self.held_input_notes
                                .iter()
                                .map(|(_, v)| *v as u32)
                                .sum::<u32>() / self.held_input_notes.len().max(1) as u32;
                            
                            let voicing = self.generate_voicing(&chord);
                            self.play_voicing(voicing, avg_vel as u8, &mut output_events);
                        }
                    }
                }
                
                _ => {}
            }
        }

        // Update swell modulation
        self.update_swell(_buffer.samples() as u32, sample_rate);

        // Send output events
        for event in output_events {
            context.send_event(event);
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for GospelWheels {
    const CLAP_ID: &'static str = "com.audio-forge.gospel-wheels";
    const CLAP_DESCRIPTION: Option<&'static str> = 
        Some("Intelligent Hammond organ processor for M-Tron Pro IV");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for GospelWheels {
    const VST3_CLASS_ID: [u8; 16] = *b"GospelWheelsAFRG";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
    ];
}

nih_export_clap!(GospelWheels);
nih_export_vst3!(GospelWheels);
