# Audio Forge Tube Screamer

Professional emulation of the classic Ibanez Tube Screamer (TS808/TS9) overdrive pedal.

## Overview

This plugin provides an authentic digital recreation of one of the most iconic guitar overdrive pedals ever made. The Tube Screamer is renowned for its smooth, mid-focused overdrive that has been used by countless legendary guitarists.

## Features

### Authentic Circuit Emulation

- **Asymmetric Soft Clipping**: Models the original's diode clipping circuit with asymmetric behavior (silicon diode on positive side, LED + diode on negative side)
- **Mid-Focused EQ**: Characteristic 720Hz high-pass filter and frequency-dependent tone shaping
- **Active Tone Control**: Emulates the original's active filter topology
- **Signal Chain Accuracy**: Input buffering → Pre-emphasis → Clipping → Tone → Output buffering

### Technical Implementation

- **Biquad Filters**: Accurate filter implementations for all stages
- **Waveshaping**: Precise soft-clipping algorithm modeling JRC4558 op-amp behavior
- **DC Blocking**: Prevents DC offset buildup
- **Anti-Aliasing**: Post-clipping low-pass filtering to reduce digital artifacts
- **Smooth Parameters**: All controls use smoothing to prevent zipper noise

### Controls

#### Drive (0-100%)
Controls the amount of gain before the clipping stage. Maps to the original pedal's DRIVE knob.
- **Low (0-30%)**: Clean boost with subtle warmth
- **Medium (30-70%)**: Classic overdrive, smooth saturation
- **High (70-100%)**: Heavy clipping, more compressed and sustained

#### Tone (0-100%)
Active shelving filter that shapes the frequency response. Maps to the original's TONE knob.
- **Low (0%)**: Dark, warm tone with rolled-off highs
- **Medium (50%)**: Balanced, classic Tube Screamer voicing
- **High (100%)**: Bright, articulate with enhanced presence

#### Level (0-100%)
Output volume control. Maps to the original's LEVEL knob.
- Compensates for gain changes
- Can boost signal above unity (up to 2x)
- Use to match bypassed signal level or push your amp harder

#### Mix (0-100%)
Parallel processing control (not on the original pedal).
- **0%**: Fully dry (bypassed)
- **50%**: 50/50 blend of dry and overdriven signal
- **100%**: Fully wet (original Tube Screamer behavior)
- Use for subtle enhancement or "New York style" parallel compression

## Sound Characteristics

### What Makes the Tube Screamer Special

1. **Mid-Range Focus**: Cuts bass (720Hz HPF) and boosts upper mids, helping guitars cut through dense mixes
2. **Asymmetric Clipping**: Creates even-order harmonics for a warmer, more musical distortion
3. **Smooth Overdrive**: Soft clipping produces gentle saturation rather than harsh fuzz
4. **Transparent**: Maintains the character of your guitar and amp while adding grit

### Best Use Cases

- **Blues & Rock**: Classic overdriven tones
- **Metal**: Tightens low end when used as a boost (Drive low, Level high)
- **Rhythm Guitar**: Adds body and sustain to chord work
- **Lead Guitar**: Smooths out notes for singing solos
- **Amp Pushing**: Use as a clean boost to drive your amp harder

## Usage Tips

### Classic Tube Screamer Settings
```
Drive: 40-60%
Tone: 50-70%
Level: Adjust to match bypass level
Mix: 100%
```

### Clean Boost (SRV Style)
```
Drive: 0-20%
Tone: 60-80%
Level: 80-100%
Mix: 100%
```

### Tight Metal Rhythm
```
Drive: 0-30%
Tone: 20-40% (keep it dark)
Level: 70-90%
Mix: 100%
```

### Parallel Saturation
```
Drive: 70-100%
Tone: 50%
Level: 50-70%
Mix: 30-50%
```

## Technical Details

### Signal Flow

```
Input → 720Hz HPF → Pre-emphasis (+6dB @ 1kHz) → 
Drive Gain → Soft Clipping → Tone Filter → 
Post LPF (8kHz) → DC Blocker → Level → Mix → Output
```

### Filter Specifications

- **Input HPF**: 720Hz, 12dB/octave, Q=0.707
- **Pre-emphasis**: Shelving filter, +6dB @ 1kHz
- **Tone Control**: Variable shelving, 500Hz-4kHz, ±12dB
- **Post LPF**: 8kHz, 12dB/octave, Q=0.707
- **DC Blocker**: 20Hz, 12dB/octave

### Clipping Characteristics

- **Positive Threshold**: 0.7V (silicon diode model)
- **Negative Threshold**: 1.2V (LED + diode model)
- **Gain Range**: 1x to 100x (0dB to 40dB)
- **Clipping Type**: Hyperbolic tangent (smooth transition)

## CPU Usage

Very efficient - suitable for multiple instances:
- Low CPU usage thanks to optimized biquad filters
- No convolution or oversampling (by design for character)
- Suitable for real-time performance

## Differences from Hardware

### Faithful to Original
- Circuit topology accurately modeled
- Frequency response matches original
- Clipping behavior recreated
- Tone control characteristic preserved

### Digital Enhancements
- **Mix Control**: Enables parallel processing
- **No Noise**: Cleaner than original (no op-amp hiss)
- **Perfect Recall**: Settings saved with projects
- **Automation**: All parameters can be automated
- **Stereo**: Processes stereo signals (original is mono)

### Not Modeled
- Component tolerances and variances
- Power supply sag
- Temperature drift
- True bypass switching clicks
- Enclosure resonances

## Version History

### v0.1.0 (Initial Release)
- Complete circuit emulation
- All four controls (Drive, Tone, Level, Mix)
- Classic green GUI
- CLAP and VST3 formats
- Stereo and mono support

## Credits

Based on the legendary Ibanez TS808/TS9 Tube Screamer, designed by Susumu Tamura in the late 1970s.

This is a digital emulation created for educational and creative purposes. Ibanez and Tube Screamer are trademarks of Hoshino Gakki.

---

**Audio Forge RS** - Open source audio plugins for Bitwig, crafted in Rust.
