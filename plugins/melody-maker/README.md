# Melody Maker

**Production-Quality Intelligent Melody Generator for Alt-Country/Americana**

Generates musical, harmonically-aware melodic phrases based on key, mode, and chord progression. Multiple instances automatically sync their harmonic framework while generating unique melodies.

## The Problem It Solves

Writing multiple melodic parts that harmonize correctly is hard:
- **Harmonic awareness** - Staying in key across multiple instruments
- **Chord tone selection** - Hitting the right notes for each chord
- **Musical variation** - Creating different but compatible melodies
- **Scale knowledge** - Understanding modes and progressions
- **Multi-track harmony** - Keeping bass, guitar, and other parts in sync

**Melody Maker solves all of this automatically.**

---

## Core Philosophy

### Shared Harmonic Framework

**All plugin instances share:**
- ✅ Key (C, D, E, F, G, A, B + flats/sharps)
- ✅ Mode (Major, Minor, Dorian, Mixolydian, Pentatonic)
- ✅ Chord Progression (I-IV-V-I, I-V-vi-IV, custom, etc.)

Change the key in ANY instance → ALL instances update immediately.

This is implemented using **global shared state with thread-safe read/write locks**.

### Independent Melody Generation

**Each instance has unique:**
- ✅ Melody Style (9 types optimized for alt-country/Americana)
- ✅ Density (how busy the melody is)
- ✅ Range (melodic interval span)
- ✅ Variation (randomness amount)
- ✅ Octave (output range)
- ✅ Random seed (each instance generates different melodies)

**Result:** Harmonically perfect, melodically varied multi-track arrangements.

---

## Production-Quality Features

### 1. Global Progression Sync

```rust
// Thread-safe global state using RwLock
lazy_static! {
    static ref GLOBAL_PROGRESSION: Arc<RwLock<SharedProgression>> = 
        Arc::new(RwLock::new(SharedProgression::default()));
}
```

- Uses `parking_lot` for high-performance locks
- Read/write access across plugin instances
- No race conditions, thread-safe
- Updates propagate instantly

### 2. Intelligent Note Selection

Each melody style uses different algorithms:

**Sparse & Lonesome:**
- Chord tones only
- Long gaps between notes
- Minimal movement

**Melodic & Flowing:**
- Stepwise motion (prefers small intervals)
- All scale tones available
- Smooth melodic contours

**Rhythmic & Driving:**
- Repeated notes for rhythm
- Jumps between chord tones
- Strong rhythmic pulse

**Contemplative:**
- Very slow movement
- Often stays on same note
- Step-wise when it moves

**Pedal Steel Bends:**
- Chromatic approaches to targets
- Sliding motion simulation
- Smooth glides between notes

**Telecaster Twang:**
- Jumpy intervals
- Bright, energetic
- Random within chord tones

**Slide Guitar:**
- Smooth glides
- Bluesy chromatic notes
- Targeting chord tones

**Prairie Wind:**
- Prefers upper register
- Open, airy spacing
- Minimal notes

**Heartland Rock:**
- Pentatonic focus
- Driving feel
- Classic rock intervals

### 3. Harmonic Intelligence

**Chord Tone Detection:**
```rust
fn get_chord_notes(&self, key: Key, mode: Mode, chord: ChordType) -> Vec<u8> {
    // Calculate scale degrees
    // Find chord root in current scale
    // Add third and fifth
    // Include passing tones based on style
    // Transpose to correct octave
}
```

**Scale Modes Supported:**
- Major (Ionian) - 7 note scale
- Natural Minor (Aeolian) - 7 note scale
- Dorian - 7 note scale (minor with raised 6th)
- Mixolydian - 7 note scale (major with flat 7th)
- Pentatonic Major - 5 note scale
- Pentatonic Minor - 5 note scale

### 4. Musical Timing

**Density Control:**
- 0% = Very sparse (notes every 4+ beats)
- 50% = Moderate (notes every 2 beats)
- 100% = Dense (notes every beat or faster)

**Phrase Length:**
- 2-16 bars per phrase
- Affects pattern repetition
- Creates musical structure

**Tempo-Aware:**
- Syncs to DAW tempo (planned feature)
- Currently uses internal tempo parameter
- Samples-per-beat calculation for timing

---

## Technical Implementation

### Architecture

```
Plugin Instance 1                Plugin Instance 2
       │                                │
       ├─ Read Global Progression       ├─ Read Global Progression
       │  (Key, Mode, Chord)            │  (Key, Mode, Chord)
       │                                │
       ├─ Generate Melody               ├─ Generate Melody
       │  (Style: Melodic)              │  (Style: Sparse)
       │  (Seed: 12345)                 │  (Seed: 67890)
       │                                │
       └─→ MIDI Out                     └─→ MIDI Out
           (Harmonize!)                     (Harmonize!)
```

### Data Flow

1. **Parameter Change** (any instance)
   - User changes Key/Mode/Progression
   - Instance writes to `GLOBAL_PROGRESSION`
   - All instances read on next process() call

2. **Note Generation** (per instance)
   - Read current chord from global state
   - Calculate available notes for this chord
   - Apply melody style algorithm
   - Apply density/range/variation
   - Output MIDI note

3. **Timing** (per instance)
   - Track samples since last note
   - When threshold reached, generate new note
   - Turn off previous note (mono output)
   - Turn on new note

### Thread Safety

- **RwLock** allows multiple readers, one writer
- **Arc** provides shared ownership across instances
- **lazy_static** ensures single global instance
- No data races, no deadlocks

---

## Usage Scenarios

### Scenario 1: Solo Lead Generation

```
Track 1: Melody Maker
  Key: G Major
  Progression: I-IV-V-I
  Style: Melodic & Flowing
  Density: 60%
  Range: 70%
  Octave: 4

→ Session Guitarist Electric Sunburst (Melody Mono)
```

**Result:** Flowing lead guitar melody in G major

### Scenario 2: Bass + Guitar Harmony

```
Track 1 (Bass): Melody Maker
  Key: C Major
  Progression: I-V-vi-IV
  Style: Sparse & Lonesome
  Density: 30%
  Octave: 2

→ Scarbee Rickenbacker Bass

Track 2 (Guitar): Melody Maker  
  Key: C Major (synced!)
  Progression: I-V-vi-IV (synced!)
  Style: Melodic & Flowing
  Density: 60%
  Octave: 4

→ Session Guitarist
```

**Result:** Bass and guitar playing different melodies, perfectly harmonized

### Scenario 3: Full Arrangement

```
Track 1 (Lead): Melody Maker (Telecaster Twang) → Guitar
Track 2 (Bass): Melody Maker (Sparse) → Bass
Track 3 (Harmony): Melody Maker (Contemplative) → Organ
Track 4 (Texture): Melody Maker (Prairie Wind) → Banjo
```

Set key/mode/progression ONCE in any track.  
All four tracks harmonize automatically!

---

## Chord Progressions Explained

### I-IV-V-I (Classic Country)
In C Major: C → F → G → C  
**Use:** Traditional country, rock, folk  
**Feel:** Resolved, complete, familiar

### I-V-vi-IV (Pop Progression)
In C Major: C → G → Am → F  
**Use:** Modern pop, radio-friendly  
**Feel:** Uplifting, contemporary

### I-vi-IV-V (50s Progression)
In C Major: C → Am → F → G  
**Use:** Nostalgic, classic  
**Feel:** Retro, familiar

### I-IV-I-V (Simple)
In C Major: C → F → C → G  
**Use:** Simple songs, driving parts  
**Feel:** Straightforward, energetic

### vi-IV-I-V (Sad Progression)
In C Major: Am → F → C → G  
**Use:** Emotional, minor feel  
**Feel:** Melancholic, contemplative

### I-iii-IV-V (Bright)
In C Major: C → Em → F → G  
**Use:** Uplifting, positive  
**Feel:** Hopeful, bright

---

## Parameters Reference

### Global (Shared Across Instances)

**Key**
- 12 options: C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B
- Sets harmonic center
- Change in ANY instance updates ALL

**Mode**
- Major (Ionian), Natural Minor, Dorian, Mixolydian, Pentatonic Major/Minor
- Determines available notes
- Affects melodic character

**Progression Type**
- 6 preset progressions + Custom
- Defines chord sequence
- All instances follow same progression

### Per-Instance (Independent)

**Melody Style**
- 9 types optimized for alt-country/Americana
- Determines note selection algorithm
- Each track can have different style

**Density** (0-100%)
- How often notes play
- 0% = very sparse, 100% = constant notes
- Adjust per instrument role

**Range** (0-100%)
- Melodic interval span
- 0% = narrow, 100% = wide leaps
- Lower for bass, higher for leads

**Variation** (0-100%)
- Randomness amount
- 0% = predictable, 100% = very random
- Moderate values (40-60%) work best

**Phrase Length** (2-16 bars)
- How long phrases are before variation
- Affects musical structure
- Longer = more coherent

**Octave** (2-6)
- Output octave range
- 2-3 for bass, 3-4 for guitar, 4-5 for high parts
- Matches instrument's playable range

**Randomize** (Button)
- Trigger new random melody
- Uses new random seed
- Keeps key/mode/progression

**Tempo** (60-180 BPM)
- Internal timing reference
- Will sync to DAW in future version
- Affects note timing

**Enabled** (On/Off)
- Turn output on/off
- Useful for muting tracks
- Keeps sync with other instances

---

## Comparison to Similar Tools

| Feature | Melody Maker | Other Generators |
|---------|--------------|------------------|
| **Multi-instance sync** | ✅ Global state sharing | ❌ Independent |
| **Production quality** | ✅ Thread-safe, robust | ⚠️ Varies |
| **Alt-country focus** | ✅ 9 specialized styles | ❌ Generic |
| **Pure MIDI output** | ✅ No instruments | ⚠️ Often bundled |
| **Mode support** | ✅ 6 modes | ⚠️ Limited |
| **Chord progression** | ✅ 6 presets + custom | ⚠️ Basic |
| **Per-instance variation** | ✅ Independent melodies | ❌ Same melody |

---

## Future Enhancements

### Planned Features

1. **DAW Tempo Sync**
   - Read tempo from host
   - Follow tempo changes
   - Grid-locked timing

2. **MIDI Learn**
   - Trigger chord changes
   - Control parameters via MIDI
   - Performance mode

3. **Pattern Recording**
   - Capture generated melodies
   - Export as MIDI clips
   - Edit and refine

4. **Advanced GUI**
   - Visual feedback of notes
   - Chord progression timeline
   - Real-time visualization

5. **More Progressions**
   - User-defined progressions
   - Modal interchange
   - Secondary dominants

6. **Humanization**
   - Velocity variation
   - Timing imperfections
   - Organic feel

---

## Tips for Best Results

### Arrangement Strategy

1. **Start with one instance** - Set key/mode/progression
2. **Add bass** - Sparse style, low octave, low density
3. **Add lead** - Melodic style, mid octave, moderate density
4. **Add texture** - Contemplative/Prairie Wind, high octave, very sparse
5. **Add rhythm** - Rhythmic style, mid-low octave, higher density

### Parameter Combinations

**Ambient/Atmospheric:**
- Style: Prairie Wind or Contemplative
- Density: 20-30%
- Range: 60-80%
- Variation: 30-40%

**Driving/Energetic:**
- Style: Rhythmic or Heartland Rock
- Density: 60-80%
- Range: 40-60%
- Variation: 50-70%

**Sparse/Minimal:**
- Style: Sparse & Lonesome
- Density: 10-30%
- Range: 30-50%
- Variation: 20-40%

**Melodic/Singing:**
- Style: Melodic & Flowing or Pedal Steel
- Density: 40-60%
- Range: 50-70%
- Variation: 40-60%

---

## Technical Specs

- **Plugin Format:** CLAP, VST3
- **Platform:** macOS, Linux, Windows
- **MIDI I/O:** Output only (no input processing)
- **Audio I/O:** None (pure MIDI generator)
- **Latency:** < 1ms (note generation is instant)
- **CPU Usage:** Minimal (simple calculations)
- **Thread Safety:** Full (parking_lot RwLock)
- **State Sharing:** Global across all instances

---

## Troubleshooting

**Q: Multiple instances have different keys**  
A: This is expected behavior. Last changed parameter wins. Set key ONCE.

**Q: Melodies sound too random**  
A: Lower Variation parameter (try 30-40%). Use more structured styles (Melodic, Rhythmic).

**Q: No notes playing**  
A: Check Enabled parameter. Check Density isn't too low. Ensure instrument is loaded after plugin.

**Q: Notes outside instrument range**  
A: Adjust Octave parameter. Session Guitarist works best at Octave 3-4.

**Q: All tracks playing same melody**  
A: Each instance uses different random seed. Try different Melody Styles. Hit Randomize button.

**Q: Melodies don't follow chords**  
A: This is a bug - report it! Should always play chord tones for current progression position.

---

## Credits

Melody Maker is part of the **Audio Forge** plugin suite.

- Built with **nih-plug** framework
- Thread-safe state using **parking_lot**
- Random generation with **rand**
- Designed for alt-country/Americana production

---

## License

AGPL-3.0 - See LICENSE file

---

**Melody Maker - Infinite melodies, always in key**
