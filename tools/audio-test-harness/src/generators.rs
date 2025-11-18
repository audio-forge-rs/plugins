use anyhow::Result;
use hound::{WavSpec, WavWriter};
use std::f32::consts::PI;
use std::path::Path;

const SAMPLE_RATE: u32 = 48000;

/// Generate a sine wave test tone
pub fn generate_sine(freq: f32, duration: f32, output: &Path) -> Result<()> {
    let spec = WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = WavWriter::create(output, spec)?;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    
    for t in 0..num_samples {
        let time = t as f32 / SAMPLE_RATE as f32;
        let sample = (time * freq * 2.0 * PI).sin();
        let amplitude = (sample * i16::MAX as f32) as i16;
        
        // Write stereo
        writer.write_sample(amplitude)?;
        writer.write_sample(amplitude)?;
    }
    
    writer.finalize()?;
    Ok(())
}

/// Generate white noise
pub fn generate_noise(duration: f32, output: &Path) -> Result<()> {
    use rand::Rng;
    
    let spec = WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = WavWriter::create(output, spec)?;
    let mut rng = rand::thread_rng();
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    
    for _ in 0..num_samples {
        let sample: f32 = rng.gen_range(-1.0..1.0);
        let amplitude = (sample * i16::MAX as f32 * 0.5) as i16; // Scale down to avoid clipping
        
        writer.write_sample(amplitude)?;
        writer.write_sample(amplitude)?;
    }
    
    writer.finalize()?;
    Ok(())
}

/// Generate impulse response test signal
pub fn generate_impulse(output: &Path) -> Result<()> {
    let spec = WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = WavWriter::create(output, spec)?;
    let duration = 2.0; // 2 seconds
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    
    for t in 0..num_samples {
        // Single impulse at start
        let amplitude = if t == 0 { i16::MAX } else { 0 };
        
        writer.write_sample(amplitude)?;
        writer.write_sample(amplitude)?;
    }
    
    writer.finalize()?;
    Ok(())
}

/// Generate MIDI file with chord progression
pub fn generate_midi(chords: &str, tempo: u32, beats_per_chord: u32, output: &Path) -> Result<()> {
    use midly::{Smf, Header, Track, TrackEvent, TrackEventKind, MidiMessage, MetaMessage};
    use midly::num::{u7, u4, u28, u24, u15};
    
    let mut tracks = Vec::new();
    let mut track = Vec::new();
    
    // Set tempo (microseconds per quarter note)
    let us_per_quarter = 60_000_000 / tempo;
    track.push(TrackEvent {
        delta: u28::from(0),
        kind: TrackEventKind::Meta(MetaMessage::Tempo(u24::from(us_per_quarter))),
    });
    
    // Parse chords and generate notes
    let chord_list: Vec<&str> = chords.split(',').collect();
    let ticks_per_beat = 480;
    let chord_duration = ticks_per_beat * beats_per_chord;
    
    for (i, chord_name) in chord_list.iter().enumerate() {
        let notes = parse_chord(chord_name.trim());
        let start_time = i as u32 * chord_duration;
        
        // Note on events
        for (j, note) in notes.iter().enumerate() {
            let delta = if i == 0 && j == 0 { 0 } else { 0 };
            track.push(TrackEvent {
                delta: u28::from(delta),
                kind: TrackEventKind::Midi {
                    channel: u4::from(0),
                    message: MidiMessage::NoteOn {
                        key: u7::from(*note),
                        vel: u7::from(100),
                    },
                },
            });
        }
        
        // Note off events
        track.push(TrackEvent {
            delta: u28::from(chord_duration),
            kind: TrackEventKind::Midi {
                channel: u4::from(0),
                message: MidiMessage::NoteOff {
                    key: u7::from(notes[0]),
                    vel: u7::from(0),
                },
            },
        });
    }
    
    // End of track
    track.push(TrackEvent {
        delta: u28::from(0),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });
    
    tracks.push(Track::from(track));
    
    let smf = Smf {
        header: Header {
            format: midly::Format::SingleTrack,
            timing: midly::Timing::Metrical(u15::from(ticks_per_beat as u16)),
        },
        tracks,
    };
    
    smf.save(output)?;
    Ok(())
}

/// Parse chord name to MIDI notes
fn parse_chord(name: &str) -> Vec<u8> {
    let root = match name.chars().next() {
        Some('C') => 60,
        Some('D') => 62,
        Some('E') => 64,
        Some('F') => 65,
        Some('G') => 67,
        Some('A') => 69,
        Some('B') => 71,
        _ => 60,
    };
    
    // Simple major triad
    vec![root, root + 4, root + 7]
}
