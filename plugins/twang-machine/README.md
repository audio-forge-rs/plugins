# Twang Machine

Intelligent MIDI processor designed specifically for **Native Instruments Session Guitarist Electric Sunburst Deluxe** in Melody/Mono mode. Transforms any MIDI input into authentic, guitar-like phrases with alt-country/Son Volt vibes.

## The Problem It Solves

Session Guitarist instruments are amazing but finicky:
- **Strict MIDI range requirements** - Notes outside C3-C6 won't play
- **Mono-only in Melody mode** - Multiple simultaneous notes cause issues
- **Articulations need precise timing** - Hammer-ons, slides, etc. require specific MIDI patterns
- **Keyboard players struggle** - Playing guitar-like phrases on keys is hard

**Twang Machine fixes all of this automatically.**

## Core Philosophy

### Liberal Input
- Accepts **any MIDI notes**, **any octave**
- Handles chords, clusters, random notes
- Ignores MIDI CC, pitch bend, etc. (only cares about note on/off)
- Works with any MIDI controller or DAW piano roll

### Intelligent Processing
- **Auto-transposes** to guitar's sweet spot (G3-D5)
- **Monitors recent notes** and shifts octaves intelligently
- **Strums/arpegg iates** chord inputs naturally
- **Adds articulations** automatically (hammer-ons when appropriate)
- **Humanizes** timing and velocity for organic feel

### Strict Output
- **Only mono melody notes** in instrument's range (C3-C6)
- **Proper note-off handling** before new notes (mono compliance)
- **Filtered MIDI** - no control messages passed through
- **Guitar-friendly voicing** - stays in playable register

## Features

### 🎸 Play Modes

**Single Note**
- Highest note priority (typical for lead guitar)
- Immediate response
- Auto hammer-ons on small intervals (1-4 semitones)
- Perfect for solos and melodic lines

**Strum Down**
- Strums through held notes from high to low
- Simulates downstroke guitar strum
- Configurable timing
- Resets pattern on new chord

**Strum Up**
- Strums through held notes from low to high
- Simulates upstroke guitar strum
- Great for fills and variations

**Arpeggio Up**
- Cycles through notes low to high
- Continuous pattern while notes held
- Classic arpeggiator behavior

**Arpeggio Down**
- Cycles through notes high to low
- Descending patterns

**Arpeggio Up-Down**
- Ping-pong pattern through notes
- More melodic, less predictable
- Perfect for ambient/textural parts

### ⚙️ Controls

#### Strum Speed (5-100ms)
Time between notes in strum/arpeggio modes
- **5-15ms**: Fast, almost simultaneous (realistic strum)
- **20-40ms**: Medium speed (default, most musical)
- **50-100ms**: Slow, deliberate (fingerpicking feel)

#### Humanize (0-100%)
Adds random variation to velocity
- **0%**: Robotic, every note same velocity
- **30-50%**: Subtle variation (default)
- **100%**: Wide variation, very human

#### Articulation (0-100%)
Controls automatic articulation detection
- **Low (0-30%)**: Mostly regular notes
- **Medium (40-70%)**: Balanced, musical hammer-ons
- **High (70-100%)**: Aggressive legato, more hammer-ons

#### Auto Transpose (On/Off)
Automatically shifts notes to playable range
- **On (default)**: Analyzes input, transposes intelligently
- **Off**: Manual transpose via Target Note only

#### Target Center Note (C3-C5)
Where auto-transpose aims for
- **C3-G3**: Darker, lower register guitar
- **C4 (default)**: Classic range, versatile
- **C5-C5**: Brighter, higher register (careful - can sound thin)

## How It Works

### Auto-Transpose Intelligence

1. **Analyzes input notes** - Calculates average pitch of held notes
2. **Compares to target** - Determines octave shift needed
3. **Transposes entire chord** - Shifts all notes by same amount
4. **Clamps to range** - Ensures output stays in C3-C6
5. **Updates continuously** - Adapts as you play different registers

Example:
```
Input:  C6, E6, G6 (too high)
Target: C4
Shift:  -2 octaves
Output: C4, E4, G4 (perfect!)
```

### Strum/Arpeggio Logic

**Chord Input** (C, E, G held simultaneously):

**Strum Down mode:**
```
Time: 0ms   → Note On: G (highest)
Time: 20ms  → Note Off: G, Note On: E
Time: 40ms  → Note Off: E, Note On: C
Time: 60ms  → Pattern complete, wait for chord change
```

**Arpeggio Up mode:**
```
Time: 0ms   → Note On: C (lowest)
Time: 20ms  → Note Off: C, Note On: E
Time: 40ms  → Note Off: E, Note On: G
Time: 60ms  → Note Off: G, Note On: C (cycle repeats)
```

### Articulation Detection

**Hammer-On (Legato)**
Triggered when:
- Previous note exists
- Interval is 1-4 semitones
- Articulation parameter > 50%

Output: Lower velocity note (~70% of original)

**Regular Note**
Triggered when:
- No previous note, OR
- Large interval (>4 semitones), OR
- Articulation parameter low

Output: Full velocity note

### Mono Compliance

Session Guitarist in Mono mode requires:
1. **Only one note at a time**
2. **Proper note-off** before new note-on
3. **No overlapping notes**

Twang Machine ensures:
```
✓ CORRECT:
  Note On: C4
  Note Off: C4
  Note On: D4

✗ WRONG:
  Note On: C4
  Note On: D4 (while C4 still on)
```

## Usage Scenarios

### 🎹 Keyboard Player Mode
**Problem**: Piano player wants to add guitar to track
**Solution**: 
1. Play chords naturally on keyboard
2. Set mode to "Strum Down"
3. Strum Speed ~25ms
4. Let Twang Machine handle the guitar parts

### 🎼 MIDI Clip Mode
**Problem**: Programmed MIDI notes in wrong octave
**Solution**:
1. Enable Auto Transpose
2. Set Target Note to C4
3. Plugin shifts everything automatically
4. No need to rewrite MIDI

### 🎸 Lead Guitar Mode
**Problem**: Need expressive lead lines
**Solution**:
1. Mode: Single Note
2. Articulation: 70%
3. Play melodically, hammer-ons happen automatically
4. Humanize: 40% for organic feel

### 🌊 Ambient Textures
**Problem**: Want evolving, atmospheric guitar
**Solution**:
1. Mode: Arpeggio Up-Down
2. Strum Speed: 80-100ms (slow)
3. Hold chord, let it evolve
4. Change chords slowly for pads

### 🎵 Alt-Country Song
**Problem**: Want Son Volt / Wilco / Uncle Tupelo vibe
**Solution**:
1. Mode: Strum Down (verses), Single Note (leads)
2. Target Note: G3 (darker, earthier)
3. Humanize: 50% (loose, human feel)
4. Articulation: 60% (some slides and hammer-ons)

## Session Guitarist Setup

### In Bitwig Studio

1. **Add Session Guitarist track**
   - Load "NI Session Guitarist - Electric Sunburst Deluxe"
   - Set to **Melody mode** (not Chords!)
   - Set to **Mono** (not Poly!)

2. **Add Twang Machine before it**
   ```
   [MIDI Track] → [Twang Machine] → [Session Guitarist]
   ```

3. **Play any MIDI**
   - Piano roll, MIDI controller, whatever
   - Twang Machine handles the translation

### Important Session Guitarist Settings

- **Mode**: Melody (not Chord mode)
- **Voicing**: Mono (absolutely critical!)
- **Amp**: Any (doesn't affect MIDI processing)
- **FX**: Any (doesn't affect MIDI processing)

### Keyswitches (Optional)

Session Guitarist uses C0-B0 for articulations. Twang Machine currently:
- ✅ Handles basic articulations via velocity/timing
- ❌ Doesn't send keyswitches (future feature)

For now, use Session Guitarist's default articulation settings.

## Tips & Tricks

### 🎛️ Getting Started Fast
1. Load plugin → defaults are already configured
2. Set mode (usually "Strum Down" or "Single Note")
3. Play - it just works!

### 🎹 For Keyboard Players
- Play chords in any octave, plugin transposes
- Use Strum modes for rhythm guitar parts
- Use Single mode for leads
- Ignore the fact it's a guitar - just play music

### 🖱️ For MIDI Programmers
- Draw notes anywhere in piano roll
- Auto-transpose handles octaves
- Use velocity for dynamics (gets humanized)
- Program chords, get strums

### 🎸 Getting Realistic Guitar
1. **Don't play too many notes** - Real guitarists are limited
2. **Use strum modes** - Guitars don't play chords instantly
3. **Vary dynamics** - Use Humanize 30-60%
4. **Stay in key** - Session Guitarist sounds best with musical input

### 🔧 Troubleshooting

**No sound from Session Guitarist:**
- Check it's in Melody mode (not Chord)
- Check it's in Mono mode (not Poly)
- Verify Twang Machine is actually receiving MIDI

**Notes sound wrong:**
- Check Auto Transpose is enabled
- Try adjusting Target Center Note
- Verify you're not in a weird mode

**Too robotic:**
- Increase Humanize to 40-60%
- Use strum/arpeggio modes instead of Single
- Vary your playing dynamics

**Articulations not working:**
- Increase Articulation parameter
- Use smaller intervals (closer notes)
- Mode must be Single Note for legato

## Technical Specs

### MIDI Processing
- **Input**: Any MIDI notes, any octave
- **Output**: Mono melody, C3-C6 range only
- **Latency**: Minimal (< 5ms typical)
- **CPU**: Very light

### Note Range
- **Instrument accepts**: C3-C6 (MIDI 48-84)
- **Optimal range**: G3-D5 (MIDI 55-74)
- **Plugin enforces**: Notes always in valid range

### Modes
- 6 play modes (Single, 2 strums, 3 arpeggios)
- Seamless mode switching
- Pattern reset on chord change

## Comparison to Alternatives

### vs. Standard Arpeggiator
- ✅ Guitar-specific intelligence
- ✅ Auto-transpose to playable range
- ✅ Mono compliance built-in
- ✅ Articulation detection

### vs. Manual MIDI Editing
- ✅ Instant results
- ✅ No tedious octave shifting
- ✅ Automatic humanization
- ✅ Realt ime playability

### vs. Playing Guitar
- ✅ No guitar skills needed
- ✅ Unlimited takes
- ✅ Perfect timing
- ❌ Less expressive (but getting there!)

## Version History

### v0.1.0 (Initial Release)
- 6 play modes (single, strum, arpeggio)
- Auto-transpose with intelligent octave detection
- Automatic articulation (hammer-ons)
- Humanization and timing control
- Optimized for Session Guitarist Electric Sunburst
- CLAP and VST3 formats

## Future Plans

- Keyswitch support for advanced articulations
- Preset system for different guitar styles
- Visual feedback (show current note, transpose amount)
- Swing/groove quantization
- More arpeggio patterns
- Strum pattern variations

---

**Audio Forge RS** - Making guitar sounds accessible to everyone.
