# Audio Forge Development Guide

## Complete Development Loop

This project now has a **complete test-driven development cycle** for audio plugins:

```
CODE → BUILD → TEST → MEASURE → ANALYZE → ITERATE
  ↑                                              ↓
  └──────────────── FEEDBACK LOOP ───────────────┘
```

## Quick Start

```bash
# 1. Generate all test files
./tools/test-plugins.sh

# 2. Build your plugin
cargo xtask bundle <plugin-name> --release

# 3. Test in DAW with files from /tmp/audio-forge-tests/

# 4. Analyze output
./target/release/audio-test-harness analyze -i YOUR_OUTPUT.wav timing
```

## Tools We've Built

### Audio Test Harness (`tools/audio-test-harness`)

A complete Rust-based CLI tool for audio generation and analysis.

**Generate Test Signals:**
```bash
# Sine waves (test overdrive, EQ, etc.)
audio-test-harness generate sine --freq 440 --duration 2.0 --output test.wav

# White noise (test noise gates, compression)
audio-test-harness generate noise --duration 2.0 --output noise.wav

# Impulse response (measure plugin latency/response)
audio-test-harness generate impulse --output impulse.wav

# MIDI progressions (test MIDI processors)
audio-test-harness generate midi \
  --chords "C,F,G,C" \
  --tempo 100 \
  --beats 4 \
  --output progression.mid
```

**Analyze Audio:**
```bash
# Basic statistics
audio-test-harness analyze -i input.wav stats
# Output: Sample rate, RMS, peak, crest factor

# Frequency spectrum (FFT)
audio-test-harness analyze -i input.wav spectrum
# Output: Top 5 dominant frequencies

# Timing analysis (MIDI processor output)
audio-test-harness analyze -i input.wav timing
# Output: Note onsets, intervals, estimated BPM

# Dynamics analysis
audio-test-harness analyze -i input.wav dynamics
# Output: Envelope peaks, dynamic range
```

### Test Suite (`tools/test-plugins.sh`)

Automated test file generation:
- ✅ Sine waves at 110, 220, 440 Hz
- ✅ White noise and impulse responses
- ✅ MIDI progressions at 80, 100, 120 BPM
- ✅ Complex chord progressions (C-Am-F-G)

All files saved to `/tmp/audio-forge-tests/`

## Development Workflows

### MIDI Processor Plugins

**Example: Low Rider (Bass Generator)**

1. **Change timing code**
   ```rust
   BassStyle::Walking => 1.0,  // Quarter notes
   ```

2. **Build**
   ```bash
   cargo xtask bundle audio-forge-low-rider --release
   ```

3. **Test in DAW**
   - Load Low Rider plugin
   - Import `/tmp/audio-forge-tests/progression_100bpm.mid`
   - Route to Scarbee Rick Bass
   - Play and LISTEN - does it feel like 100 BPM?

4. **Render and analyze**
   ```bash
   # Render in DAW to /tmp/bass_test.wav
   audio-test-harness analyze -i /tmp/bass_test.wav timing
   ```

5. **Verify results**
   ```
   Expected for Walking bass at 100 BPM:
   - Average interval: ~0.6 seconds (quarter notes)
   - Estimated tempo: 95-105 BPM
   - Regular note spacing
   ```

### Audio Effect Plugins

**Example: Tube Screamer (Overdrive)**

1. **Change gain/EQ code**
   ```rust
   let drive_amount = params.drive.value();
   ```

2. **Build**
   ```bash
   cargo xtask bundle audio-forge-tubescreamer --release
   ```

3. **Test in DAW**
   - Load Tube Screamer
   - Import `/tmp/audio-forge-tests/sine_440hz.wav`
   - Process and LISTEN

4. **Render and analyze**
   ```bash
   # Process clean → /tmp/driven.wav
   audio-test-harness analyze -i /tmp/audio-forge-tests/sine_440hz.wav spectrum
   audio-test-harness analyze -i /tmp/driven.wav spectrum
   ```

5. **Verify results**
   ```
   Expected for overdrive:
   - More harmonic content (880 Hz, 1320 Hz, 1760 Hz)
   - Higher RMS level
   - Compressed dynamics (lower crest factor)
   ```

## Why This Matters

### The Problem We Solved

Initially, plugins had **severe timing issues**:
- Bass was playing at 16th note speed (insane!)
- Banjo was playing bluegrass rolls instead of sparse alt-country
- Guitar strums were 20ms (unrealistically fast)

**Root cause:** No way to hear plugins during development.

### The Solution

1. **Test Harness** - Generate audio and MIDI programmatically
2. **Analysis Tools** - Measure timing, frequency, dynamics
3. **Automated Suite** - One command generates everything
4. **Documentation** - Clear workflows for each plugin type

Now we have a **complete feedback loop**:
- ✅ Hear the plugin (DAW testing)
- ✅ Measure the output (timing analysis)
- ✅ Verify correctness (automated checks)
- ✅ Iterate quickly (one command rebuild)

## Best Practices

### Before Pushing Code

1. ✅ Run test suite: `./tools/test-plugins.sh`
2. ✅ Build plugin: `cargo xtask bundle <plugin> --release`
3. ✅ Load in DAW with appropriate test file
4. ✅ **LISTEN** - Does it sound right?
5. ✅ Render output audio
6. ✅ Analyze with test harness
7. ✅ Verify timing/frequency/dynamics match expectations
8. ✅ Document any new parameters or features

### Timing Verification Checklist

For MIDI processors at 100 BPM:

| Duration | Beats | Seconds | Check |
|----------|-------|---------|-------|
| Whole note | 4.0 | 2.4s | ⏱️ Sparse bass roots |
| Half note | 2.0 | 1.2s | ⏱️ Sparse patterns |
| Quarter note | 1.0 | 0.6s | ⏱️ Walking bass |
| Eighth note | 0.5 | 0.3s | ⏱️ Busy patterns |

**If timing < 0.5 beats, it's probably too fast for alt-country!**

### Audio Quality Checklist

For audio effects:

- ✅ No clipping (peak < 0 dBFS)
- ✅ Appropriate RMS level (-20 to -6 dBFS for most material)
- ✅ Expected frequency response (measure with spectrum analysis)
- ✅ Appropriate dynamics (check crest factor)
- ✅ No DC offset (check stats)

## Future Enhancements

### Automated CI/CD Testing

```rust
#[test]
fn test_low_rider_walking_bass_timing() {
    let midi = generate_midi("C,F,G,C", 100);
    let plugin = load_plugin("low-rider");
    let audio = process(plugin, midi);
    let timing = analyze_timing(audio);
    
    assert!(timing.avg_interval >= 0.55 && timing.avg_interval <= 0.65);
    assert!(timing.estimated_bpm >= 95.0 && timing.estimated_bpm <= 105.0);
}
```

### Plugin Hosting

Directly load and process CLAP plugins without DAW:
```bash
audio-test-harness test-midi \
  --plugin target/bundled/audio-forge-low-rider.clap \
  --midi /tmp/progression.mid \
  --output /tmp/bass_output.wav \
  --analyze
```

### Regression Testing

Track plugin output over time:
```bash
audio-test-harness regression \
  --plugin audio-forge-tubescreamer \
  --reference tests/reference/tubescreamer_440hz.wav \
  --tolerance 0.1
```

## Key Lessons

1. **Always test with sound** - No exceptions
2. **Measure objectively** - Don't trust your ears alone
3. **Document assumptions** - What does "0.25 beats" mean?
4. **Test at multiple tempos** - 80, 100, 120 BPM minimum
5. **Alt-country is SLOW** - Embrace space and sparseness

---

**Remember: If you can't measure it, you can't optimize it.**
