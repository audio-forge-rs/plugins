# Low Rider

Intelligent bass line generator designed for **Scarbee Rickenbacker Bass**. Transforms guitar chords, simple melodies, or basic bass ideas into compelling, musically intelligent bass parts with authentic alt-country/rock feel.

## The Problem It Solves

Creating great bass lines is hard:
- **Guitar parts don't translate** - Chords are too high, too dense
- **Simple roots are boring** - Just playing the root note lacks interest
- **Timing is everything** - Bass needs rhythm, syncopation, groove
- **Articulation matters** - Slides, hammer-ons, mutes create authenticity
- **Musical knowledge required** - Knowing when to use fifth, passing tones, etc.

**Low Rider does the heavy lifting automatically.**

## Core Philosophy

### Musical Intelligence

Unlike simple arpeggiat ors, Low Rider:
- **Analyzes chord quality** (major, minor, dominant)
- **Selects appropriate notes** (root, fifth, third, passing tones, chromatic approaches)
- **Creates rhythmic patterns** (not just on-the-beat notes)
- **Adds syncopation** naturally
- **Uses articulations** musically (not randomly)
- **Remembers what it played** to create coherent lines

### Liberal Input → Compelling Output

**Accepts:**
- Guitar chords (any octave)
- Simple melodies
- Basic bass notes
- Chord progressions

**Generates:**
- Walking bass lines
- Driving rhythmic patterns
- Sparse, spacious parts
- Melodic bass movements
- With proper timing, articulation, and musical sense

## Features

### 🎸 Bass Styles

**Roots**
- Simple root notes on downbeats
- Solid, steady foundation
- Great for verses, simple sections
- Minimal movement

**Root-Fifth**
- Alternates between root and perfect fifth
- Classic rock bass pattern
- More movement than just roots
- Drives the groove forward

**Walking**
- Walking bass style with passing tones
- Uses root, third, fifth, and chromatic approaches
- Jazz/blues influenced
- Smooth, melodic movement

**Driving**
- Syncopated, rhythmic patterns
- Mostly roots with occasional fifths
- Punchy, energetic
- Perfect for upbeat alt-country/rock

**Sparse**
- Long sustained notes
- Space between notes
- Minimal, atmospheric
- Lets other instruments breathe

**Melodic**
- Busy, melodic bass lines
- Uses passing tones, seconds, fourths
- More complex harmonic movement
- Engaging, interesting parts

### ⚙️ Controls

#### Activity (0-100%)
How busy/active the bass line is
- **Low (0-30%)**: Fewer notes, more space
- **Medium (40-70%)**: Balanced, musical
- **High (70-100%)**: Busy, constant movement

Affects note frequency and duration.

#### Syncopation (0-100%)
Amount of rhythmic offset from the beat
- **Low (0-30%)**: On-the-beat, straight
- **Medium (40-70%)**: Some syncopation (default)
- **High (70-100%)**: Heavy syncopation, funky

Adds rhythmic interest without chaos.

#### Movement (0-100%)
Use of fifths, passing tones, and melodic movement
- **Low (0-30%)**: Mostly roots
- **Medium (40-70%)**: Some fifths, occasional passing tones
- **High (70-100%)**: Active bass lines, walking patterns

Controls harmonic complexity.

#### Articulation (0-100%)
Variety of playing techniques
- **Low (0-30%)**: Mostly sustained notes
- **Medium (40-70%)**: Some slides and articulations
- **High (70-100%)**: Aggressive use of slides, mutes, hammer-ons

Adds authenticity and expression.

#### Sustain (0-100%)
Note length and legato vs. staccato
- **Low (0-30%)**: Short, punchy notes
- **Medium (40-70%)**: Balanced sustain (default)
- **High (70-100%)**: Long, sustained notes

Affects feel and groove.

#### Tempo (60-180 BPM)
Internal tempo for pattern generation
- Should match your project tempo
- Affects timing of notes
- Critical for groove

#### Octave Preference
Where bass notes sit in register
- **Lower**: E1-B1 range (deeper, more sub)
- **Auto**: E1-B1 (default, balanced)
- **Higher**: E2-B2 range (brighter, more midrange)

## How It Works

### Chord Analysis Engine

1. **Detects chord input** - Analyzes notes you're holding
2. **Finds root note** - Lowest note determines root (transposed to bass register)
3. **Determines quality** - Major, minor, dominant, diminished
4. **Calculates intervals** - Available notes (third, fifth, seventh)

Example:
```
Input:  C major chord (C4, E4, G4)
Root:   C (pitch class 0)
Quality: Major (has major third at 4 semitones, fifth at 7)
Available bass notes: C, E (3rd), G (5th)
```

### Note Selection Algorithm

Based on style and parameters:

**Roots Style:**
```
Always output: Root note
Timing: Downbeats
```

**Root-Fifth Style:**
```
Pattern: Root → Fifth → Root → Fifth
If Movement > 50%: Add occasional third
```

**Walking Style:**
```
Beat 1: Root
Beat 2: Third (major or minor based on chord quality)
Beat 3: Fifth
Beat 4: Seventh or chromatic approach to next root
```

**Driving Style:**
```
Base: Root notes
If Movement > random: Substitute fifth
Add syncopation based on Syncopation parameter
```

**Melodic Style:**
```
Choose from: Root, Fifth, Major 2nd, Fourth
Weighted random selection
Creates melodic contour
```

### Rhythm Generation

Timing calculated based on:
- **Style** - Base note divisions (whole, half, quarter)
- **Activity** - More activity = shorter notes, more frequent
- **Syncopation** - Offset from beat (early/late by up to 15%)

Example (Driving style, Activity 60%, Syncopation 40%):
```
Base timing: Quarter notes (0.25 beats)
Activity modifier: 0.25 * (1 - 0.6 * 0.5) = 0.175 beats
Syncopation: Random offset ±0.15 * 0.4 = ±0.06 beats
Actual note: 0.175 + random offset
```

### Articulation Logic

**Slides:**
- Triggered when: Previous note exists, interval 3-7 semitones, Articulation > 50%, random chance
- Effect: Smooth transition between notes

**Muted Notes:**
- Triggered when: Articulation > 70%, style = Driving, random chance
- Effect: Percussive, staccato attack (lower velocity)

**Sustain/Legato:**
- Controlled by Sustain parameter
- Higher sustain = longer note durations
- Creates smoothness or punchiness

### Musical Memory

Plugin tracks:
- **Last 16 notes played** - Avoids repetitive patterns
- **Pattern fragments** - Learns from generated lines
- **Chord history** - Smoother transitions between chords
- **Beat position** - Maintains groove consistency

## Usage Scenarios

### 🎹 From Guitar Chords

**Input:** Guitar part with chord progression
**Setup:**
```
Style: Driving
Activity: 60%
Syncopation: 40%
Movement: 50%
Tempo: Match song tempo
```

**Result:** Driving bass line that follows chord changes with syncopation and occasional fifths.

### 🎸 From Simple Bass Sketch

**Input:** Basic root notes you played on keyboard
**Setup:**
```
Style: Walking or Melodic
Activity: 70%
Movement: 70%
Articulation: 60%
```

**Result:** Enhances your simple ideas with passing tones, rhythm, and articulation.

### 🎼 From MIDI Clip

**Input:** Programmed chord progression
**Setup:**
```
Style: Root-Fifth
Activity: 50%
Syncopation: 30%
Sustain: 70%
```

**Result:** Solid, dependable bass line that locks with drums.

### 🌊 Ambient/Sparse

**Input:** Sustained chords
**Setup:**
```
Style: Sparse
Activity: 20%
Movement: 30%
Sustain: 90%
Articulation: 20%
```

**Result:** Long, sustained bass notes that support without cluttering.

### 🎵 Alt-Country Song

**Input:** Verse/chorus chords
**Setup:**
```
Verse:
  Style: Roots or Root-Fifth
  Activity: 40%
  Syncopation: 20%
  
Chorus:
  Style: Driving
  Activity: 65%
  Syncopation: 50%
```

**Result:** Authentic alt-country bass that builds from verse to chorus.

## Scarbee Rick Setup

### In Bitwig Studio

1. **Add Scarbee Rickenbacker Bass track**
   - Load instrument
   - Default settings work great

2. **Add Low Rider before it**
   ```
   [MIDI Track] → [Low Rider] → [Scarbee Rick Bass]
   ```

3. **Play chords or simple parts**
   - Plugin generates compelling bass
   - Adapt via Style and parameters

### Scarbee Rick Notes

- **Range**: E1 (MIDI 28) to G3 (MIDI 55)
- **Articulations**: Handled via velocity and timing
- **Best in**: E1-E2 range (Low Rider default)
- **Character**: Punchy, defined low-end, great for rock

## Tips & Tricks

### 🎛️ Start Simple
1. Load plugin with defaults
2. Set Style to "Driving" or "Root-Fifth"
3. Match Tempo to your project
4. Play chords
5. Adjust Activity and Syncopation to taste

### 🎸 Getting Realistic Bass
- **Use Walking style** for blues/jazz feels
- **Use Driving style** for rock/alt-country
- **Higher Activity** for busy sections
- **Lower Activity** for verses, calm parts
- **Syncopation 30-60%** is musical sweet spot

### 🎵 Building Dynamics
- **Verse**: Roots style, Activity 40%, Syncopation 20%
- **Pre-Chorus**: Root-Fifth, Activity 55%, Syncopation 35%
- **Chorus**: Driving, Activity 70%, Syncopation 50%
- **Bridge**: Walking or Melodic, Activity 60%, Movement 70%

### 🔧 Troubleshooting

**Bass line too busy:**
- Reduce Activity to 30-50%
- Try Roots or Sparse style
- Reduce Syncopation

**Bass line too simple:**
- Increase Activity to 70-80%
- Try Walking or Melodic style
- Increase Movement to 60-80%

**Wrong octave:**
- Adjust Octave Preference (Lower/Auto/Higher)
- Check input chord octave (plugin transposes to bass range)

**Timing feels off:**
- Verify Tempo matches project
- Adjust Syncopation (try 0% for on-beat)
- Try different Style

**Not enough variation:**
- Increase Movement (adds fifths, passing tones)
- Try Walking or Melodic style
- Increase Articulation

## Technical Details

### MIDI Processing
- **Input**: Any MIDI notes (chords, melodies, bass sketches)
- **Output**: Mono bass notes in E1-G3 range
- **Analysis**: Real-time chord detection and quality analysis
- **Latency**: Minimal (pattern generation is predictive)

### Note Range
- **Instrument range**: E1-G3 (MIDI 28-55)
- **Optimal range**: E1-E2 (MIDI 28-40)
- **Auto-transpose**: Yes (input chords transposed to bass register)

### Musical Intelligence
- Chord quality detection (major, minor, dominant, diminished)
- Root note extraction from any chord voicing
- Interval-based note selection
- Pattern memory (16-note history)
- Rhythmic variation algorithms

### Articulations
- Slides (velocity + timing based)
- Mutes (lower velocity, shorter duration)
- Legato/sustain (note duration control)
- Future: Hammer-ons, harmonics

## Comparison

### vs. Simple Arpeggiator
- ✅ Understands chord quality
- ✅ Selects musical notes (not just all chord tones)
- ✅ Rhythmic intelligence
- ✅ Articulation variety
- ✅ Musical memory

### vs. Manual Bass Programming
- ✅ Instant results
- ✅ Musical even without bass knowledge
- ✅ Real-time jamming
- ✅ Variation with one parameter change
- ❌ Less control over specific notes (trade-off)

### vs. Playing Bass
- ✅ No bass skills needed
- ✅ Perfect timing
- ✅ Consistent execution
- ❌ Less human feel (but Syncopation helps!)

## Version History

### v0.1.0 (Initial Release)
- 6 bass styles (Roots, Root-Fifth, Walking, Driving, Sparse, Melodic)
- Intelligent chord analysis (major, minor, dominant, diminished)
- Note selection algorithm with fifths, thirds, passing tones
- Rhythm generation with syncopation
- Articulation support (slides, mutes)
- Musical memory (16-note history)
- Tempo sync (60-180 BPM)
- Octave preference control
- Optimized for Scarbee Rickenbacker Bass
- CLAP and VST3 formats

## Future Plans

- More bass styles (Reggae, Funk, Motown)
- Keyswitch articulation support
- Swing/shuffle grooves
- Pattern save/recall
- MIDI learn for style switching
- Visual feedback (show current note, chord analysis)
- Humanization (timing micro-variations)
- Ghost notes
- Harmonic detection (9ths, 11ths, 13ths)

---

**Audio Forge RS** - Making professional bass lines accessible to everyone.
