# Audio Forge Plugin Testing Guide

## The Problem

When building MIDI processor plugins, it's critical to **actually hear** the timing and feel of the generated patterns. Initially, Twang Machine, Low Rider, and Lonesome Picker all had timing issues - they were playing WAY too fast because the developer couldn't hear the results during development.

**Never ship a music plugin without testing it with sound!**

## Testing Tools

### 1. Audio Test Harness (PRIMARY TOOL)

We've built a comprehensive CLI test harness in Rust at `tools/audio-test-harness`.

**Quick Start:**
```bash
# Run comprehensive test suite
./tools/test-plugins.sh
```

This generates:
- Test audio files (sine waves, noise, impulses)
- MIDI files at multiple tempos (80, 100, 120 BPM)
- Analysis tools for audio output

**Manual Usage:**

```bash
# Generate sine wave
./target/release/audio-test-harness generate sine --freq 440 --duration 2.0 --output test.wav

# Generate MIDI progression
./target/release/audio-test-harness generate midi --chords "C,F,G,C" --tempo 100 --beats 4 --output test.mid

# Analyze audio
./target/release/audio-test-harness analyze -i test.wav stats
./target/release/audio-test-harness analyze -i test.wav spectrum
./target/release/audio-test-harness analyze -i test.wav timing
./target/release/audio-test-harness analyze -i test.wav dynamics
```

**Analysis Features:**
- 📊 **Stats** - RMS, peak levels, crest factor
- 📈 **Spectrum** - FFT analysis, dominant frequencies
- ⏱️ **Timing** - Note onset detection, tempo estimation
- 📉 **Dynamics** - Peak detection, dynamic range

### 2. DAW Testing (For Actual Listening)

**Required workflow:**
1. Run test suite: `./tools/test-plugins.sh`
2. Build plugin: `cargo xtask bundle <plugin-name> --release`
3. Load plugin in DAW
4. Import test MIDI file from `/tmp/audio-forge-tests/`
5. Route through virtual instrument
6. **LISTEN** - Does timing feel right?
7. Render to WAV
8. Analyze output: `./target/release/audio-test-harness analyze -i OUTPUT.wav timing`

### 3. Test File Locations

After running `./tools/test-plugins.sh`:

**Audio files** (`/tmp/audio-forge-tests/`):
- `sine_110hz.wav`, `sine_220hz.wav`, `sine_440hz.wav` - Pure tones
- `noise.wav` - White noise (2 seconds)
- `impulse.wav` - Impulse response test

**MIDI files** (`/tmp/audio-forge-tests/`):
- `progression_80bpm.mid` - Slow alt-country tempo
- `progression_100bpm.mid` - Medium tempo  
- `progression_120bpm.mid` - Fast tempo
- `progression_complex.mid` - C-Am-F-G progression

## Timing Issues Fixed

### Low Rider Bass (2024-11-17)

**Problem:**
```rust
BassStyle::Walking => 0.25,  // 16th notes - insanely fast!
BassStyle::RootFifth => 0.5, // 8th notes - still too fast
```

**Fix:**
```rust
BassStyle::Walking => 1.0,       // Quarter notes - proper walking bass
BassStyle::RootFifth => 1.0,     // Whole notes alternating
BassStyle::Driving => 0.5-1.0,   // Mix of half and whole notes
```

### Lonesome Picker Banjo (2024-11-17)

**Problem:**
```rust
PickingStyle::Clawhammer => 0.25,  // Too fast - this is bluegrass speed!
PickingStyle::ForwardRoll => 0.166, // Triplets way too fast
```

**Fix:**
```rust
PickingStyle::Clawhammer => 0.5,     // Half beat - contemplative
PickingStyle::ForwardRoll => 0.5,    // Alt-country, not bluegrass
PickingStyle::Sparse => 2.0+,        // Lonesome prairie vibes
```

### Twang Machine Guitar (2024-11-17)

**Problem:**
```rust
strum_speed: 20ms default  // Way too fast
```

**Fix:**
```rust
strum_speed: 60ms default  // Natural strum timing
range: 20-150ms            // Allows fast to very slow
```

## Alt-Country Timing Philosophy

These plugins are designed for **alt-country/Americana**, NOT:
- Prog rock complexity
- Bluegrass speed
- Virtuosic display

**Key principle: SPACE IS MUSICAL**

- Bass: Quarter notes to whole notes (not 16ths!)
- Banjo: Half notes to whole notes (sparse, atmospheric)
- Guitar: 60ms strums (not 20ms flurries)
- Organ: Sustained chords, occasional comping (not constant)

## Testing Checklist for New Plugins

Before pushing any MIDI processor plugin:

- [ ] Build and bundle plugin
- [ ] Load in actual DAW
- [ ] Route to real virtual instrument
- [ ] Play test chord progression
- [ ] **LISTEN** to the timing - does it feel musical?
- [ ] Check at 80 BPM (slow), 100 BPM (medium), 120 BPM (fast)
- [ ] Verify timing feels natural for the style (alt-country)
- [ ] Test different activity/density parameters
- [ ] Make sure "sparse" settings are actually sparse
- [ ] Verify "busy" settings don't become unusable

## Realistic Timing Values

For 4/4 time at 100 BPM:

| Duration | Beats | Seconds | Use Case |
|----------|-------|---------|----------|
| Whole note | 4.0 | 2.4s | Sparse bass, sustained organ |
| Half note | 2.0 | 1.2s | Bass roots, sparse banjo |
| Quarter note | 1.0 | 0.6s | Walking bass, normal banjo |
| Eighth note | 0.5 | 0.3s | Busy patterns, faster picking |
| 16th note | 0.25 | 0.15s | **TOO FAST** for alt-country! |

**If you're using values < 0.5 beats, you're probably making it too fast.**

## Future Improvements

1. **Automated testing harness**
   - Generate MIDI → Process through plugin → Render audio
   - Measure note timing programmatically
   - Verify tempo accuracy

2. **Unit tests for timing**
   ```rust
   #[test]
   fn test_walking_bass_timing() {
       // Verify Walking style produces quarter notes
       assert_eq!(get_timing(BassStyle::Walking), 1.0);
   }
   ```

3. **CI/CD audio validation**
   - Render test patterns in CI
   - Compare timing against reference
   - Fail build if timing drifts

## Lessons Learned

1. **Always test with sound** - No exceptions
2. **Alt-country is SLOW** - Embrace space
3. **Default values matter** - Most users won't adjust
4. **Document your assumptions** - What does "0.25 beats" mean?
5. **Test at multiple tempos** - 80, 100, 120 BPM minimum

---

**Remember: If it sounds too busy, it probably is. Alt-country is about space, feel, and restraint.**

## Development Workflow with Test Harness

### For MIDI Processor Plugins

1. **Make code changes** to plugin timing/pattern generation
2. **Build plugin:** `cargo xtask bundle <plugin-name> --release`
3. **Test in DAW:**
   - Load plugin
   - Import `/tmp/audio-forge-tests/progression_100bpm.mid`
   - Route to appropriate virtual instrument
   - **LISTEN** - Does it sound right?
4. **Render audio** to `/tmp/bass_test.wav` (or similar)
5. **Analyze timing:**
   ```bash
   ./target/release/audio-test-harness analyze -i /tmp/bass_test.wav timing
   ```
6. **Check results:**
   - Note onset times should match expected rhythm
   - Estimated BPM should be close to 100
   - Intervals should match your style (1.0 = quarter note, 0.5 = eighth, etc.)

### For Audio Effect Plugins

1. **Make code changes** to effect processing
2. **Build plugin:** `cargo xtask bundle <plugin-name> --release`
3. **Test in DAW:**
   - Load plugin
   - Import test tone (e.g., `/tmp/audio-forge-tests/sine_440hz.wav`)
   - Process through plugin
   - **LISTEN** - Does it sound right?
4. **Render audio** to `/tmp/effect_test.wav`
5. **Analyze output:**
   ```bash
   # Check levels and clipping
   ./target/release/audio-test-harness analyze -i /tmp/effect_test.wav stats
   
   # Check frequency response
   ./target/release/audio-test-harness analyze -i /tmp/effect_test.wav spectrum
   
   # Check dynamics
   ./target/release/audio-test-harness analyze -i /tmp/effect_test.wav dynamics
   ```

### Example: Testing Low Rider Bass

```bash
# 1. Generate test MIDI
./target/release/audio-test-harness generate midi \
  --chords "C,F,G,C" \
  --tempo 100 \
  --beats 4 \
  --output /tmp/test_chords.mid

# 2. Load in DAW:
#    MIDI Track → Low Rider → Scarbee Rick Bass → Render to /tmp/bass_output.wav

# 3. Analyze timing
./target/release/audio-test-harness analyze -i /tmp/bass_output.wav timing

# Expected output for Walking bass style at 100 BPM:
#   - Note onsets every ~0.6 seconds (quarter notes)
#   - Estimated tempo: ~100 BPM
#   - Regular intervals (not random)
```

### Example: Testing Tube Screamer

```bash
# 1. Generate test tone
./target/release/audio-test-harness generate sine \
  --freq 440 \
  --duration 2.0 \
  --output /tmp/clean_guitar.wav

# 2. Load in DAW:
#    Audio Track (/tmp/clean_guitar.wav) → Tube Screamer → Render to /tmp/driven_guitar.wav

# 3. Analyze frequency response
./target/release/audio-test-harness analyze -i /tmp/clean_guitar.wav spectrum
./target/release/audio-test-harness analyze -i /tmp/driven_guitar.wav spectrum

# Expected: More harmonic content in driven signal
# Should see harmonics at 880 Hz, 1320 Hz, etc.

# 4. Check levels
./target/release/audio-test-harness analyze -i /tmp/driven_guitar.wav stats

# Expected: Higher RMS, compressed dynamics (lower crest factor)
```

## Automated Testing (Future)

The test harness is designed to support automated testing:

```rust
// Future test example
#[test]
fn test_walking_bass_timing() {
    let midi = generate_test_midi("C,F,G,C", 100);
    let audio = process_through_plugin("low-rider", midi);
    let timing = analyze_timing(audio);
    
    // Verify quarter note spacing (0.6s at 100 BPM)
    assert!(timing.avg_interval >= 0.55 && timing.avg_interval <= 0.65);
    assert!(timing.estimated_bpm >= 95.0 && timing.estimated_bpm <= 105.0);
}
```

This would enable CI/CD testing to catch timing regressions.


## UI Inspection

### Why Inspect UI

Plugin UIs need visual verification:
- ✅ Correct colors and themes
- ✅ Readable text and labels
- ✅ Proper parameter layout
- ✅ Consistent branding
- ✅ Professional appearance

### Quick UI Capture

```bash
# Capture single plugin
./target/release/audio-test-harness capture-ui --plugin twang-machine

# Capture all plugins
./target/release/audio-test-harness capture-ui --all
```

### UI Inspection Workflow

1. **Build plugin**
   ```bash
   cargo xtask bundle audio-forge-twang-machine --release
   ```

2. **Capture UI**
   ```bash
   ./target/release/audio-test-harness capture-ui --plugin twang-machine
   ```

3. **Visual inspection checklist:**
   - ✅ All parameters visible
   - ✅ Text readable at plugin size
   - ✅ Colors match theme (check editor_style.css)
   - ✅ Layout clean and organized
   - ✅ No cut-off elements
   - ✅ Consistent with other plugins

4. **Fix issues and repeat**

### UI Testing Tools

- **screenshot-plugin.sh** - Single plugin capture
- **capture-all-plugins.sh** - All plugins with HTML index
- **test harness integration** - `capture-ui` subcommand

See `tools/plugin-ui-inspector/README.md` for details.

---

## Complete Testing Cycle

```
┌─────────────────────────────────────────────────┐
│  GENERATE TEST FILES                            │
│  ./tools/test-plugins.sh                        │
└────────────────┬────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────┐
│  BUILD PLUGIN                                   │
│  cargo xtask bundle <plugin> --release          │
└────────────────┬────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────┐
│  CAPTURE UI                                     │
│  audio-test-harness capture-ui --plugin <name>  │
│  → Visual inspection                            │
└────────────────┬────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────┐
│  TEST IN DAW                                    │
│  → Load plugin                                  │
│  → Import test MIDI/audio                       │
│  → LISTEN to output                             │
│  → Render audio                                 │
└────────────────┬────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────┐
│  ANALYZE OUTPUT                                 │
│  audio-test-harness analyze -i output.wav      │
│  → Check timing, frequency, dynamics            │
└────────────────┬────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────┐
│  VERIFY & ITERATE                               │
│  → Does it sound right?                         │
│  → Does it look right?                          │
│  → Do measurements match expectations?          │
│  → If not, fix and repeat                       │
└─────────────────────────────────────────────────┘
```

**Full sensory testing:** HEAR the audio, SEE the UI, MEASURE the output!

