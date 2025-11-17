# Audio Forge Plugin Testing Guide

## The Problem

When building MIDI processor plugins, it's critical to **actually hear** the timing and feel of the generated patterns. Initially, Twang Machine, Low Rider, and Lonesome Picker all had timing issues - they were playing WAY too fast because the developer couldn't hear the results during development.

**Never ship a music plugin without testing it with sound!**

## Testing Tools

### 1. MIDI Test File Generator

Use `test_midi.py` to create simple chord progression MIDI files:

```bash
python3 test_midi.py
```

This creates:
- `test_slow.mid` (80 BPM)
- `test_medium.mid` (100 BPM)  
- `test_fast.mid` (120 BPM)

Each contains a simple I-IV-V-I progression in C major with 4-beat chord changes.

### 2. DAW Testing (Primary Method)

**Best approach:**
1. Build plugin: `cargo xtask bundle <plugin-name> --release`
2. Copy to plugin folder: `cp target/bundled/*.clap ~/Library/Audio/Plug-Ins/CLAP/`
3. Open Bitwig/Ableton/Reaper
4. Create MIDI track with test progression
5. Route through plugin → virtual instrument
6. **LISTEN** to the timing and adjust

### 3. CLI Testing (Future)

Currently limited due to plugin format requirements. Potential options:
- **FluidSynth** - MIDI playback with SoundFonts
- **SoX** - Audio manipulation
- **abc2midi** - Create test MIDI from ABC notation

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
