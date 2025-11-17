# Tube Screamer Quick Start Guide

## Installation

### macOS
```bash
cp -r target/bundled/audio-forge-tubescreamer.clap ~/Library/Audio/Plug-Ins/CLAP/
cp -r target/bundled/audio-forge-tubescreamer.vst3 ~/Library/Audio/Plug-Ins/VST3/
```

### Linux
```bash
cp -r target/bundled/audio-forge-tubescreamer.clap ~/.clap/
cp -r target/bundled/audio-forge-tubescreamer.vst3 ~/.vst3/
```

### Windows
Copy the folders to:
- `C:\Program Files\Common Files\CLAP\`
- `C:\Program Files\Common Files\VST3\`

Then rescan plugins in Bitwig.

## Quick Settings Guide

### 🎸 Classic Blues/Rock Tone
**For smooth, singing overdrive**
```
Drive: 50%
Tone:  65%
Level: 50% (adjust to match bypass)
Mix:   100%
```
**Use for**: Blues leads, classic rock rhythm, vintage tones

---

### 🎵 Clean Boost (Stevie Ray Vaughan Style)
**Transparent boost to push your amp**
```
Drive: 10-15%
Tone:  75%
Level: 85%
Mix:   100%
```
**Use for**: Driving tube amps, adding sparkle, tightening tone

---

### 🤘 Metal Rhythm Tightener
**Tightens low end before high-gain amp**
```
Drive: 0-20%
Tone:  30-40%
Level: 75%
Mix:   100%
```
**Use for**: Djent, metal rhythm, palm muting clarity

---

### 🎹 Warm Saturation
**Adds harmonics to synths/keys**
```
Drive: 40%
Tone:  50%
Level: 60%
Mix:   100%
```
**Use for**: Warming digital synths, adding character to clean sounds

---

### 🎛️ Parallel Enhancement
**New York style parallel processing**
```
Drive: 80%
Tone:  50%
Level: 60%
Mix:   35-50%
```
**Use for**: Adding presence without losing clarity, subtle enhancement

---

### 🔥 Maximum Overdrive
**Full saturation and compression**
```
Drive: 90-100%
Tone:  60%
Level: 50%
Mix:   100%
```
**Use for**: Heavy leads, sustained notes, full-on crunch

---

## Parameter Ranges Explained

### Drive (0-100%)
- **0-20%**: Clean boost territory, minimal distortion
- **20-50%**: Light overdrive, guitar character preserved
- **50-70%**: Classic Tube Screamer sweet spot
- **70-100%**: Heavy saturation, compressed sustain

### Tone (0-100%)
- **0-30%**: Dark, warm, vintage tone
- **30-50%**: Balanced, natural voicing
- **50-70%**: Classic TS sound, slight brightness
- **70-100%**: Bright, cutting, articulate

### Level (0-100%)
- **0-25%**: Reduces output (for parallel processing)
- **25-60%**: Unity gain region
- **60-100%**: Boost output (can push amp harder)

### Mix (0-100%)
- **0%**: Fully bypassed
- **25-40%**: Subtle parallel enhancement
- **50%**: Equal wet/dry blend
- **75-90%**: Mostly processed
- **100%**: Full Tube Screamer effect

## Tips & Tricks

### 🎯 Finding Your Sound
1. Start with Drive at 50%, Tone at 50%, Level at 50%, Mix at 100%
2. Adjust Drive to taste (more = more saturation)
3. Dial in Tone to match your guitar/amp
4. Set Level to match bypass volume
5. Experiment with Mix for parallel processing

### 💡 EQ Interaction
The Tube Screamer cuts bass and boosts mids by design:
- **High-pass at 720Hz** - removes mud and rumble
- **Mid boost** - helps guitar cut through mix
- Use EQ **after** the plugin to shape final tone

### 🎚️ Gain Staging
- **Into high-gain amp**: Use low Drive (0-30%), higher Level
- **Into clean amp**: Use higher Drive (50-80%), match Level
- **Into DAW/interface**: Can use full range of settings

### 🔊 Stacking
The Tube Screamer can be stacked with other effects:
- **Before distortion/amp**: Tightens tone, adds mids
- **After distortion/amp**: Adds color and smoothing
- **With compressor**: Put compressor after for more sustain

### ⚡ Automation Ideas
- **Drive**: Build intensity during chorus/solo
- **Tone**: Open up for leads, darken for rhythm
- **Mix**: Gradually blend in for buildups
- **Level**: Boost solos, reduce for verses

## Troubleshooting

### Too Muddy
- Increase Tone to 60-80%
- Consider high-pass EQ before plugin
- Use less Drive

### Too Harsh/Bright
- Decrease Tone to 30-50%
- Reduce Drive slightly
- Add subtle low-pass EQ after

### Not Enough Gain
- Increase Drive to 70-100%
- This is by design - TS is not a high-gain pedal
- Stack with additional overdrive/distortion

### Too Much Noise
- This plugin is noise-free (digital advantage)
- If you want analog-style noise, add noise generator before

### Doesn't Sound "Tube Screamer-ish"
- Try the Classic Blues/Rock preset (Drive 50%, Tone 65%)
- Remember it's mid-focused - cuts bass by design
- Works best with full-range instruments (guitar, bass, keys)

## Technical Notes

### CPU Usage
Very light - suitable for multiple instances. No oversampling or convolution means low latency and efficient processing.

### Latency
Minimal - only biquad filters introduce negligible phase delay. Suitable for live performance and real-time monitoring.

### Automation
All parameters can be automated smoothly without clicks or pops. Sample-accurate automation supported.

### Mono vs Stereo
- Processes stereo signals independently
- Original hardware is mono
- For authentic mono behavior, use mono track or sum to mono before

---

**Enjoy your new Tube Screamer plugin!** 🎸

For technical details and circuit information, see [README.md](README.md)
