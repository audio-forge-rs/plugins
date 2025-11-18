use anyhow::Result;
use colored::*;
use hound::WavReader;
use std::path::Path;

/// Analyze audio file statistics
pub fn analyze_stats(input: &Path) -> Result<()> {
    let mut reader = WavReader::open(input)?;
    let spec = reader.spec();
    
    println!("\n{} Audio File Statistics", "📊".bold());
    println!("  Sample Rate: {} Hz", spec.sample_rate);
    println!("  Channels: {}", spec.channels);
    println!("  Bits per Sample: {}", spec.bits_per_sample);
    println!("  Duration: {:.2} seconds", 
        reader.duration() as f32 / spec.sample_rate as f32);
    
    // Calculate RMS and peak
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();
    
    let rms = (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt();
    let peak = samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
    
    println!("\n{} Levels", "🎚️ ".bold());
    println!("  RMS: {:.2} dB", 20.0 * rms.log10());
    println!("  Peak: {:.2} dB", 20.0 * peak.log10());
    println!("  Crest Factor: {:.2} dB", 20.0 * (peak / rms).log10());
    
    Ok(())
}

/// Analyze frequency spectrum using FFT
pub fn analyze_spectrum(input: &Path) -> Result<()> {
    use rustfft::{FftPlanner, num_complex::Complex};
    
    let mut reader = WavReader::open(input)?;
    let spec = reader.spec();
    
    // Read first 4096 samples for FFT
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .take(4096)
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();
    
    if samples.len() < 4096 {
        println!("{} Audio file too short for spectrum analysis", "⚠".yellow());
        return Ok(());
    }
    
    // Prepare FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(4096);
    
    let mut buffer: Vec<Complex<f32>> = samples
        .iter()
        .map(|&s| Complex::new(s, 0.0))
        .collect();
    
    fft.process(&mut buffer);
    
    // Calculate magnitude spectrum
    let magnitudes: Vec<f32> = buffer
        .iter()
        .take(2048) // Only first half (Nyquist)
        .map(|c| (c.re * c.re + c.im * c.im).sqrt())
        .collect();
    
    // Find dominant frequencies
    println!("\n{} Frequency Spectrum", "📈".bold());
    
    let mut peaks: Vec<(usize, f32)> = magnitudes
        .iter()
        .enumerate()
        .map(|(i, &mag)| (i, mag))
        .collect();
    
    peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("  Top 5 frequencies:");
    for (i, (bin, magnitude)) in peaks.iter().take(5).enumerate() {
        let freq = *bin as f32 * spec.sample_rate as f32 / 4096.0;
        let db = 20.0 * magnitude.log10();
        println!("    {}. {:.1} Hz ({:.1} dB)", i + 1, freq, db);
    }
    
    Ok(())
}

/// Analyze timing (for MIDI processor output - detect note onsets)
pub fn analyze_timing(input: &Path) -> Result<()> {
    let mut reader = WavReader::open(input)?;
    let spec = reader.spec();
    
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();
    
    // Simple onset detection using energy
    let window_size = 1024;
    let hop_size = 512;
    let threshold = 0.01;
    
    let mut onsets = Vec::new();
    let mut last_energy = 0.0;
    
    for i in (0..samples.len() - window_size).step_by(hop_size) {
        let window = &samples[i..i + window_size];
        let energy: f32 = window.iter().map(|s| s * s).sum::<f32>() / window_size as f32;
        
        // Detect sudden energy increase
        if energy > last_energy + threshold {
            let time = i as f32 / spec.sample_rate as f32;
            onsets.push(time);
        }
        
        last_energy = energy;
    }
    
    println!("\n{} Timing Analysis", "⏱️ ".bold());
    println!("  Detected {} note onsets", onsets.len());
    
    if onsets.len() > 1 {
        // Calculate inter-onset intervals
        let intervals: Vec<f32> = onsets
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        
        let avg_interval = intervals.iter().sum::<f32>() / intervals.len() as f32;
        let estimated_bpm = 60.0 / avg_interval;
        
        println!("  Average interval: {:.3} seconds", avg_interval);
        println!("  Estimated tempo: {:.1} BPM", estimated_bpm);
        
        println!("\n  Note onset times:");
        for (i, time) in onsets.iter().enumerate().take(10) {
            println!("    Note {}: {:.3} s", i + 1, time);
        }
        
        if onsets.len() > 10 {
            println!("    ... and {} more", onsets.len() - 10);
        }
    }
    
    Ok(())
}

/// Analyze dynamics (peak detection, envelope)
pub fn analyze_dynamics(input: &Path) -> Result<()> {
    let mut reader = WavReader::open(input)?;
    let spec = reader.spec();
    
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();
    
    // Calculate envelope using moving average
    let window_size = 4800; // 100ms at 48kHz
    let mut envelope = Vec::new();
    
    for i in 0..samples.len() - window_size {
        let window = &samples[i..i + window_size];
        let rms = (window.iter().map(|s| s * s).sum::<f32>() / window_size as f32).sqrt();
        envelope.push(rms);
    }
    
    println!("\n{} Dynamics Analysis", "📉".bold());
    
    // Find peaks in envelope
    let mut peaks = Vec::new();
    for i in 1..envelope.len() - 1 {
        if envelope[i] > envelope[i - 1] && envelope[i] > envelope[i + 1] && envelope[i] > 0.01 {
            let time = i as f32 / spec.sample_rate as f32;
            peaks.push((time, envelope[i]));
        }
    }
    
    println!("  Detected {} dynamic peaks", peaks.len());
    
    if !peaks.is_empty() {
        println!("\n  Peak levels:");
        for (i, (time, level)) in peaks.iter().enumerate().take(5) {
            println!("    Peak {}: {:.2} s, {:.2} dB", 
                i + 1, time, 20.0 * level.log10());
        }
    }
    
    // Calculate dynamic range
    let min_level = envelope.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_level = envelope.iter().cloned().fold(0.0f32, f32::max);
    let dynamic_range = 20.0 * (max_level / min_level.max(0.0001)).log10();
    
    println!("\n  Dynamic Range: {:.1} dB", dynamic_range);
    
    Ok(())
}
