# Lonesome Picker

Intelligent banjo MIDI processor designed for **Ample Ethno Banjo** (picking mode). Transforms any MIDI input into authentic alt-country/Americana banjo parts with sparse, atmospheric, melodic character - not hokey bluegrass.

## The Banjo's Role in Alt-Country

In alt-country music (think atmospheric, melancholic Americana), banjo serves a unique role:
- **Sparse and atmospheric** - Not rapid-fire bluegrass rolls
- **Melodic and haunting** - Single-note phrases, not chords
- **Drone string character** - That high G string ringing out
- **Space and silence** - Knowing when NOT to play
- **Emotional resonance** - Lonesome, contemplative feel

**Lonesome Picker captures this aesthetic automatically.**

## Core Philosophy

### Not Your Grandpa's Bluegrass

This plugin is designed for:
- ✅ Alt-country, Americana, indie-folk
- ✅ Atmospheric, spacious arrangements  
- ✅ Melodic, singing banjo lines
- ✅ Sparse picking patterns
- ✅ Emotional, contemplative parts

NOT designed for:
- ❌ Rapid bluegrass rolls
- ❌ "Dueling Banjos" style
- ❌ Hokey, novelty sounds
- ❌ Constant picking

### Musical Intelligence

- **Analyzes input** - Understands chords, melodies, simple notes
- **Picks melodically** - Chooses notes that sing
- **Adds space** - Knows when silence is golden
- **Uses drone string** - That characteristic 5th string ring
- **Articulates naturally** - Hammer-ons, slides at musical moments
- **Respects the instrument** - Stays in banjo's sweet spot (G3-D5)

## Features

### 🎸 Picking Styles

**Melody**
- Simple single-note melodies
- Clean, direct
- Perfect for lead lines
- Sparse, spacious

**Clawhammer**
- Traditional down-picking style
- Alternating thumb and finger
- Rhythmic, driving
- Authentic old-time feel

**Forward Roll**
- Triplet-feel picking pattern
- Flowing, circular motion
- Not rapid - tasteful speed
- Good for texture

**Alternating**
- Thumb alternates with fingers
- Steady, hypnotic
- Great for accompaniment
- Americana vibe

**Sparse**
- Maximum space between notes
- Long, sustained notes
- Atmospheric and haunting
- Perfect for intros, outros, ambient sections

**Melodic Run**
- Flowing melodic phrases
- Uses passing tones
- More active than Sparse
- Singing, lyrical quality

### ⚙️ Parameters

#### Density (0-100%)
How many notes in the pattern
- **Low (0-30%)**: Very sparse, lots of space
- **Medium (40-60%)**: Balanced picking
- **High (70-100%)**: More continuous (still tasteful)

Controls note frequency without losing musicality.

#### Drone String (0-100%)
How often the 5th string (high G) rings out
- **Low (0-30%)**: Minimal drone
- **Medium (40-70%)**: Characteristic banjo sound
- **High (70-100%)**: Constant drone presence

The drone adds that quintessential banjo shimmer.

#### Melodic (0-100%)
Use of passing tones and melodic movement
- **Low (0-30%)**: Stick to chord tones
- **Medium (40-60%)**: Some passing notes
- **High (70-100%)**: Active melodic phrases, chromatic approaches

Creates interest without busyness.

#### Articulation (0-100%)
Variety of playing techniques
- **Low (0-30%)**: Mostly straight notes
- **Medium (40-70%)**: Some hammer-ons and slides
- **High (70-100%)**: Expressive articulations

Adds authentic banjo expression.

#### Sparseness (0-100%)
Space and silence between notes
- **Low (0-30%)**: More connected notes
- **Medium (40-60%)**: Balanced space
- **High (70-100%)**: Maximum space, very atmospheric

Critical for alt-country aesthetic - less is more.

#### Tempo (60-180 BPM)
Internal tempo for pattern timing
- Must match your project tempo
- Affects note spacing and rhythm
- Slower tempos (80-110) common for alt-country

#### Auto Transpose (On/Off)
Automatically shifts notes to banjo range
- **On (default)**: Input transposed to G3-D5 sweet spot
- **Off**: Use input notes as-is

Ensures playability in banjo register.

## How It Works

### Banjo Tuning Awareness

Standard 5-string G tuning: **G-D-G-B-D**
- String 5 (drone): G4 (MIDI 67)
- String 1: D4 (MIDI 62)
- String 2: B3 (MIDI 59)  
- String 3: G3 (MIDI 55)
- String 4: D3 (MIDI 50)

Plugin understands this and generates appropriate notes.

### Drone String Intelligence

The 5th string is special:
- Higher pitched than other strings
- Often plays throughout (drone effect)
- Creates banjo's characteristic ring
- More common in sparse, atmospheric styles

Algorithm:
```
If Sparse style: Drone probability * 1.5
If Clawhammer style: Drone probability * 1.2
If Melodic Run: Drone probability * 0.8
Random check against Drone parameter
```

### Melodic Note Selection

From chord/input:
1. Transpose to banjo register (G3-D5 sweet spot)
2. For sparse styles: Prefer highest note (sings better)
3. For patterns: Vary between chord tones
4. If Melodic > 50%: Add passing tones between intervals
5. Avoid repetitive patterns (checks history)

### Picking Timing

Based on style and parameters:
```
Melody:       0.5 beats (half notes) + sparseness
Clawhammer:   0.25 beats (quarter notes)
Forward Roll: 0.166 beats (triplet feel)
Alternating:  0.333 beats (triplet)
Sparse:       1.0 beats + sparseness * 0.5 (whole notes+)
Melodic Run:  0.125-0.25 beats (eighth to quarter notes)
```

Sparseness adds space, Density can shorten.

### Articulation Triggering

**Hammer-On:**
- Small ascending interval (1-3 semitones)
- Articulation > 50%
- Random chance (60%)
- Effect: Lower velocity (~50%)

**Slide:**
- Medium interval (3-7 semitones)
- Articulation > 60%
- Random chance (70%)
- Effect: Slightly reduced velocity (~65%)

Articulations happen at musically appropriate moments.

## Usage Scenarios

### 🎵 Alt-Country Ballad

**Input:** Slow chord progression
**Setup:**
```
Style: Sparse
Density: 30%
Drone: 70%
Sparseness: 70%
Tempo: 85 BPM
```

**Result:** Haunting, spacious banjo with lots of silence and drone string shimmer.

### 🎸 Driving Americana

**Input:** Uptempo chord changes
**Setup:**
```
Style: Clawhammer
Density: 60%
Drone: 50%
Melodic: 40%
Tempo: 110 BPM
```

**Result:** Rhythmic clawhammer pattern following chords with drone presence.

### 🌊 Atmospheric Intro

**Input:** Sustained chord or single note
**Setup:**
```
Style: Sparse or Melody
Density: 20%
Drone: 80%
Sparseness: 85%
Articulation: 40%
```

**Result:** Minimal picking with heavy drone, creates atmosphere.

### 🎼 Melodic Interlude

**Input:** Chord progression or melody
**Setup:**
```
Style: Melodic Run
Density: 65%
Melodic: 75%
Articulation: 60%
Sparseness: 35%
```

**Result:** Flowing melodic phrases with passing tones.

### 🏞️ Lonesome Prairie Sound

**Input:** Simple major chords (G, C, D)
**Setup:**
```
Style: Alternating
Density: 45%
Drone: 65%
Melodic: 50%
Sparseness: 55%
Tempo: 95 BPM
```

**Result:** Authentic Americana feel, contemplative and spacious.

## Ample Ethno Banjo Setup

### In Bitwig Studio

1. **Load Ample Ethno Banjo**
   - Use **Picking Mode** (NOT strummer/pattern mode)
   - Default sustain articulation works great

2. **Add Lonesome Picker before it**
   ```
   [MIDI Track] → [Lonesome Picker] → [Ample Ethno Banjo]
   ```

3. **Match tempo** in Lonesome Picker to project

4. **Play chords or simple melodies**
   - Plugin generates banjo parts
   - Adjust Style and parameters

### Important Ample Ethno Banjo Notes

- **Mode**: Use Picking mode for individual note control
- **Tuning**: Standard G tuning (G-D-G-B-D)
- **Range**: D3-A5 (plugin keeps you in G3-D5 sweet spot)
- **Articulations**: Handled via velocity and timing by plugin

## Tips & Tricks

### 🎛️ Less is More

Alt-country banjo is about **restraint**:
- Start with low Density (30-40%)
- High Sparseness (60-80%)
- Let the drone string speak
- Embrace silence

### 🎸 The Drone is Your Friend

That 5th string is magic:
- Drone 60-80% for authentic sound
- Higher in Sparse style
- Creates that lonesome quality
- Don't be afraid of constant drone

### 🎵 Tempo Matters

Alt-country is often slower:
- 80-110 BPM is sweet spot
- Slower tempos = more space
- Matches contemplative mood
- Fast tempos lose atmosphere

### 🌊 Building Dynamics

**Verse:**
```
Style: Sparse
Density: 25%
Sparseness: 75%
```

**Chorus:**
```
Style: Clawhammer or Alternating
Density: 55%
Sparseness: 45%
```

**Bridge/Solo:**
```
Style: Melodic Run
Density: 70%
Melodic: 75%
```

### 🔧 Troubleshooting

**Sounds too busy:**
- Increase Sparseness to 70-90%
- Reduce Density to 20-40%
- Try Sparse style

**Not enough notes:**
- Reduce Sparseness to 20-40%
- Increase Density to 60-70%
- Try Clawhammer or Melodic Run style

**Wrong register:**
- Enable Auto Transpose
- Plugin targets G3-D5 automatically

**No drone:**
- Increase Drone parameter to 60-80%
- Some styles use drone more (Sparse, Clawhammer)

**Too bluegrassy:**
- Increase Sparseness
- Reduce Density
- Use Sparse or Melody style
- This plugin is anti-bluegrass by design

## Technical Details

### MIDI Processing
- **Input**: Any MIDI notes, chords, melodies
- **Output**: Picking patterns + optional drone string
- **Range**: D3-A5 (focused on G3-D5)
- **Auto-transpose**: Yes, to banjo sweet spot

### Banjo Characteristics
- 5-string in G tuning (G-D-G-B-D)
- Drone string: G4 (MIDI 67)
- Sweet spot: G3-D5 for melodic playing
- Articulations via velocity and timing

### Pattern Intelligence
- 6 picking styles
- Drone string integration
- Melodic phrase generation
- Sparse timing algorithms
- Musical memory (12-note history)

### Articulations
- Hammer-ons (small ascending intervals)
- Slides (medium intervals)
- Velocity-based expression
- Timing-based legato

## Version History

### v0.1.0 (Initial Release)
- 6 picking styles (Melody, Clawhammer, Forward Roll, Alternating, Sparse, Melodic Run)
- Drone string intelligence (5th string awareness)
- Auto-transpose to banjo range (G3-D5 sweet spot)
- Melodic movement with passing tones
- Articulation system (hammer-ons, slides)
- Sparseness control for atmospheric playing
- Tempo sync (60-180 BPM)
- Optimized for Ample Ethno Banjo picking mode
- CLAP and VST3 formats

## Future Plans

- More picking patterns (thumb-lead, double-thumbing)
- Capo simulation (transpose + drone adjustment)
- Different tunings (double-C, old-time D)
- Ghost notes
- Choke/mute articulations
- Visual feedback (show drone state, picking pattern)
- Preset system for different moods

---

**Audio Forge RS** - Bringing lonesome prairie sounds to your studio.
