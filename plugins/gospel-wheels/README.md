# Gospel Wheels

**Intelligent Hammond Organ MIDI Processor for M-Tron Pro IV**

Transform any MIDI input into authentic alt-country/Americana Hammond organ parts. Gospel Wheels analyzes your playing, detects chord quality, and generates musically intelligent organ voicings with proper voice leading, harmonic layering (simulating drawbar registration), and dynamic swell simulation.

## The Problem This Solves

Playing convincing Hammond organ requires specific techniques:
- **Proper voicing** - Not just block chords, but thoughtful note selection
- **Register awareness** - Knowing which octaves work for each musical context
- **Voice leading** - Smooth transitions between chords
- **Drawbar thinking** - Understanding harmonic layers (16', 8', 4', 2 2/3')
- **Dynamic control** - Swells and volume manipulations
- **Rhythmic comping** - Not just sustained notes

**Gospel Wheels does all of this automatically.** You play, it creates authentic Hammond organ parts.

---

## Philosophy

This plugin is designed for **alt-country, Americana, indie rock organ** - think Son Volt, Wilco, Uncle Tupelo, The Band, Dr. Dog. NOT:
- Church gospel rave-ups
- Prog rock Hammond solos
- Jazz organ virtuosity

The focus is on **atmospheric, spare, thoughtful organ** that enhances songs without overpowering them.

---

## How It Works

### 1. Chord Analysis Engine

Gospel Wheels continuously analyzes your held notes to determine:
- **Root note** - Foundation of the chord
- **Chord quality** - Major, minor, dominant 7th, diminished, augmented, sus4
- **Available chord tones** - Root, 2nd, 3rd, 5th, 6th, 7th (depending on quality)
- **Bass note** - Lowest input note for bass-style voicings

**Algorithm:**
```
1. Collect all currently held MIDI notes
2. Extract pitch classes (0-11, ignoring octaves)
3. Identify root as lowest pitch class
4. Calculate intervals from root
5. Match interval pattern to chord quality:
   - [4, 7] = Major (M3 + P5)
   - [3, 7] = Minor (m3 + P5)
   - [4, 7, 10] = Dominant (M3 + P5 + m7)
   - [3, 6] = Diminished
   - [4, 8] = Augmented
   - [5, 7] = Sus4
6. Build scale of available chord tones
```

### 2. Voicing Generation

Based on the selected **Style**, Gospel Wheels creates appropriate organ voicings:

#### **Sustained**
Long, held chords with full voicing:
- Root + 3rd + 5th in primary register
- Octave doubling if Harmonics > 50%
- Smooth voice leading from previous chord

#### **Comping**
Rhythmic chord stabs for groove:
- Sparser voicing (root + 3rd or root + 5th)
- Driven by Rhythm parameter (timing density)
- Percussive character

#### **Swell**
Gradual dynamic volume changes:
- Full chord voicing like Sustained
- Velocity modulates with swell LFO
- Creates atmospheric, breathing organ sound

#### **Arpeggiated**
Broken chord patterns:
- Builds full voicing (root + 3rd + 5th + 7th)
- Notes triggered separately over time
- Creates movement without busyness

#### **Bass**
Left-hand bass + right-hand chord:
- Bass note in lower register (E1-C3 range)
- Chord voicing in mid register (C4-C6)
- Classic organ trio approach

#### **Atmospheric**
Sparse, spacious voicings:
- Minimal notes (often just root + 5th)
- Higher register placement
- Maximum space for reverb/ambience

### 3. Harmonic Layering (Drawbar Simulation)

When **Auto-Thicken** is enabled and **Harmonics** is > 30%, Gospel Wheels adds harmonic layers simulating Hammond drawbar registration:

- **16' (Sub-Octave)** - Active when Harmonics > 70%
  - Adds depth and power
  - Octave below fundamental

- **8' (Fundamental)** - Always present
  - Core organ sound

- **4' (Octave)** - Active when Harmonics > 40%
  - Brightness and clarity
  - Octave above fundamental

- **2 2/3' (Quint)** - Active when Harmonics > 80%
  - Adds harmonic complexity
  - Fifth above octave (19 semitones up)

**This creates the characteristic Hammond "drawbar" sound without requiring you to think about it.**

### 4. Voice Leading

When **Voicing** parameter > 50%, Gospel Wheels implements smooth voice leading:

```rust
For each note in new chord:
  1. Find corresponding note in previous chord
  2. Try current octave, octave up, octave down
  3. Choose octave that minimizes movement distance
  4. Update voicing with minimal-movement version
```

This creates professional-sounding chord transitions where individual voices move as little as possible - a hallmark of good organ playing.

### 5. Register Control

The **Register** parameter controls overall pitch placement:
- **0%** - Low register (C2-C4) - dark, powerful
- **50%** - Mid register (C3-C5) - balanced, classic
- **100%** - High register (C4-C6) - bright, airy

Gospel Wheels automatically transposes generated voicings to the selected register.

### 6. Swell Simulation

The **Swell** parameter creates dynamic volume changes:

```rust
Swell LFO oscillates: 0.0 → 1.0 → 0.0 (4-beat cycle at current Tempo)
Velocity scaling: base_velocity * (0.3 + swell_position * 0.7)

Result: Velocity ranges from 30% to 100% of input
```

Creates the characteristic Hammond swell pedal effect automatically.

---

## Parameters

| Parameter | Range | Description |
|-----------|-------|-------------|
| **Style** | 6 options | Sustained, Comping, Swell, Arpeggiated, Bass, Atmospheric |
| **Harmonics** | 0-100% | Drawbar richness (16', 8', 4', 2 2/3' layering) |
| **Swell** | 0-100% | Dynamic volume swell amount |
| **Rhythm** | 0-100% | Rhythmic density for Comping style |
| **Voicing** | 0-100% | Voice leading smoothness (0% = fresh, 100% = smooth) |
| **Register** | 0-100% | Pitch range (0% = low, 50% = mid, 100% = high) |
| **Tempo** | 60-180 BPM | Timing reference for rhythmic patterns |
| **Auto-Thicken** | On/Off | Enable harmonic layering (drawbar simulation) |

---

## Usage Scenarios

### Scenario 1: Playing Live from Keyboard

**Setup:**
```
Keyboard → Gospel Wheels → M-Tron Pro IV (Hammond B3 tape)
```

**Settings:**
- Style: Sustained or Swell
- Harmonics: 60%
- Swell: 50% (adds breathing to sustained parts)
- Voicing: 70% (smooth transitions)
- Register: 50% (mid-range)
- Auto-Thicken: ON

**Usage:**
- Play basic chords (triads, 7ths)
- Gospel Wheels handles voicing and register
- Focus on timing and musical phrasing
- Let swells create dynamic movement

### Scenario 2: Programming from Guitar Chords

**Setup:**
```
MIDI Track (guitar chords) → Gospel Wheels → M-Tron Pro IV
```

**Settings:**
- Style: Atmospheric or Sustained
- Harmonics: 40% (sparse)
- Swell: 30%
- Voicing: 80% (very smooth)
- Register: 60% (slightly higher)
- Auto-Thicken: OFF

**Usage:**
- Record or program guitar chord progression
- Gospel Wheels extracts roots and creates organ voicings
- Perfect for building from scratch when you have guitar but need organ

### Scenario 3: Rhythmic Comping

**Setup:**
```
MIDI Controller/Pattern → Gospel Wheels → M-Tron Pro IV
```

**Settings:**
- Style: Comping
- Harmonics: 50%
- Rhythm: 70% (busy comping)
- Voicing: 40% (some variation)
- Register: 55%
- Tempo: Match song tempo
- Auto-Thicken: ON

**Usage:**
- Send sustained chord MIDI
- Gospel Wheels creates rhythmic pattern automatically
- Adjust Rhythm parameter to control busyness
- Perfect for adding groove without sustained pads

### Scenario 4: Bass + Organ

**Setup:**
```
Keyboard (split at C3) → Gospel Wheels → M-Tron Pro IV
```

**Settings:**
- Style: Bass
- Harmonics: 60%
- Voicing: 70%
- Register: 50%
- Auto-Thicken: ON

**Usage:**
- Play left hand bass notes below C3
- Play right hand chords above C3
- Gospel Wheels routes bass to low register, chords to mid
- Creates classic organ trio sound

---

## Song Building Guide

### Verse - Sparse and Atmospheric
```
Style: Atmospheric
Harmonics: 40%
Swell: 60%
Register: 55% (slightly high)
```
Creates space, doesn't compete with vocals.

### Chorus - Full and Rich
```
Style: Sustained
Harmonics: 70%
Swell: 40%
Register: 50%
Auto-Thicken: ON
```
Adds fullness and harmonic complexity.

### Bridge - Dynamic Movement
```
Style: Swell
Harmonics: 60%
Swell: 80% (dramatic swells)
Voicing: 80%
```
Creates dynamic contrast and builds energy.

### Breakdown - Rhythmic Interest
```
Style: Comping
Harmonics: 50%
Rhythm: 65%
Tempo: 120 BPM (adjust to song)
```
Adds groove without sustaining.

---

## M-Tron Pro IV Setup

1. **Load M-Tron Pro IV** in your DAW
2. **Select Hammond Organ tape**:
   - B3 Mk1 (classic)
   - B3 Mk2 (brighter)
   - C3 (darker)
   - M400 (mellow)
3. **Set to basic mode** (not tape manipulation mode)
4. **Ensure polyphony** is set to at least 6 voices
5. **Route MIDI**: Gospel Wheels → M-Tron Pro IV
6. **Mix settings** (in M-Tron):
   - Tone: 50-60% (adjust to taste)
   - Tape Wear: 10-20% (adds vintage character)
   - Wow/Flutter: 5-10% (subtle movement)

**Gospel Wheels handles all the musical intelligence. M-Tron provides the sound.**

---

## Tips

### DO:
- ✅ Start with default settings (Sustained, 60% Harmonics)
- ✅ Use Atmospheric for verses, Sustained for choruses
- ✅ Experiment with Register for different song sections
- ✅ Enable Auto-Thicken for fuller sound
- ✅ Use Voicing > 70% for smooth ballads
- ✅ Try Bass style for organ trio vibes

### DON'T:
- ❌ Set Harmonics to 100% (too dense, sounds muddy)
- ❌ Use Comping style with Rhythm > 80% (too busy)
- ❌ Ignore Register parameter (crucial for tone)
- ❌ Expect prog-rock solos (this is for songwriting)
- ❌ Disable Auto-Thicken if you want full organ sound

---

## Musical Intelligence Summary

Gospel Wheels is **not just a MIDI effect**. It's a musical assistant that understands:

1. **Harmony** - Chord qualities, available extensions
2. **Voicing** - Proper note placement, register awareness
3. **Voice Leading** - Smooth transitions between chords
4. **Timbre** - Harmonic layers, drawbar simulation
5. **Dynamics** - Swells, expression, breathing
6. **Rhythm** - Comping patterns, syncopation awareness

**You focus on the song. Gospel Wheels focuses on the organ.**

---

## Comparison to Alternatives

| Feature | Gospel Wheels | Manual MIDI | Other MIDI Tools |
|---------|---------------|-------------|------------------|
| Chord Analysis | ✅ Automatic | ❌ Manual | ⚠️ Basic |
| Voice Leading | ✅ Intelligent | ❌ Manual | ❌ None |
| Drawbar Simulation | ✅ Yes (layering) | ❌ None | ❌ None |
| Register Control | ✅ Parameter-driven | ⚠️ Manual transpose | ❌ None |
| Swell Simulation | ✅ Automatic LFO | ⚠️ CC automation | ⚠️ Generic LFO |
| Style Switching | ✅ 6 organ-specific | ❌ None | ⚠️ Generic presets |

---

## Technical Specs

- **Platform**: macOS (CLAP, VST3)
- **Input**: MIDI notes (any range, any voicing)
- **Output**: MIDI notes (optimized organ voicings)
- **Latency**: < 1ms (sample-accurate MIDI processing)
- **Polyphony**: Up to 12 simultaneous output notes (with layering)
- **Memory**: Tracks 16-note history for pattern coherence

---

## Troubleshooting

**Q: Gospel Wheels isn't outputting any notes**  
A: Check MIDI routing. Gospel Wheels is a MIDI processor, not a sound source. Ensure M-Tron Pro IV is receiving MIDI from Gospel Wheels.

**Q: Sound is too dense/muddy**  
A: Lower Harmonics to 40-50%. Disable Auto-Thicken. Try Atmospheric style.

**Q: Voice leading sounds jumpy**  
A: Increase Voicing parameter to 80-90% for smoother transitions.

**Q: Comping style isn't creating rhythm**  
A: Ensure Tempo parameter matches your project tempo. Increase Rhythm parameter.

**Q: Swells aren't working**  
A: Check that Swell parameter > 20%. Ensure Style is set to Swell or Sustained. Swells won't work in Comping or Arpeggiated styles.

**Q: Bass notes aren't low enough**  
A: Ensure Style is set to Bass. Lower Register parameter to 0-30%.

---

## Credits

Gospel Wheels is part of the **Audio Forge** plugin suite.

- Built with **nih-plug** framework
- Designed for **M-Tron Pro IV** by GForce Software
- Inspired by alt-country, Americana, and indie rock organ traditions

---

## License

AGPL-3.0 - See LICENSE file

---

**Gospel Wheels - Open Road Hymns**  
*Intelligent Hammond organ for the modern songwriter*
