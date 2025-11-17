# Twang Machine - Quick Start

Get authentic guitar phrases from Session Guitarist in 60 seconds.

## Setup

### 1. In Bitwig Studio

Create this signal chain:
```
[MIDI Track] → [Twang Machine Plugin] → [NI Session Guitarist]
```

### 2. Configure Session Guitarist

**CRITICAL SETTINGS:**
- **Mode**: Melody (NOT Chord mode)
- **Voicing**: Mono (NOT Poly)

Location: Top of Session Guitarist interface

### 3. Play Something

- Play any MIDI notes on any octave
- Twang Machine automatically handles the rest
- You should hear guitar!

## Common Setups

### 🎸 Rhythm Guitar (Strumming Chords)

**Twang Machine Settings:**
```
Mode: Strum Down
Strum Speed: 25ms
Humanize: 40%
Auto Transpose: ON
```

**How to play:**
- Play chords on keyboard (any octave)
- Each chord triggers a strum
- Change chords for rhythm part

---

### 🎵 Lead Guitar (Single Note Melodies)

**Twang Machine Settings:**
```
Mode: Single Note
Articulation: 70%
Humanize: 30%
Auto Transpose: ON
```

**How to play:**
- Play single note melodies
- Small intervals become hammer-ons automatically
- Works like a lead guitar

---

### 🌊 Ambient Textures

**Twang Machine Settings:**
```
Mode: Arpeggio Up-Down
Strum Speed: 80ms
Humanize: 50%
Auto Transpose: ON
```

**How to play:**
- Hold a chord
- Plugin creates evolving arpeggio
- Change chords slowly for pads

---

## Troubleshooting

### No Sound?
1. Check Session Guitarist is in **Melody mode**
2. Check Session Guitarist is in **Mono mode**
3. Verify MIDI is reaching Twang Machine

### Wrong Octave?
1. Enable **Auto Transpose**
2. Adjust **Target Center Note** (try C4)

### Sounds Robotic?
1. Increase **Humanize** to 40-60%
2. Try **Strum mode** instead of Single Note

## Parameter Cheat Sheet

| Parameter | Default | What It Does |
|-----------|---------|--------------|
| Mode | Strum Down | How notes are played |
| Strum Speed | 20ms | Time between strummed notes |
| Humanize | 30% | Random velocity variation |
| Articulation | 70% | Hammer-on sensitivity |
| Auto Transpose | ON | Automatic octave shifting |
| Target Note | C4 | Where to transpose to |

## Quick Tips

✅ **DO:**
- Play any notes, any octave
- Use chords in Strum/Arpeggio modes
- Experiment with different modes
- Trust the auto-transpose

❌ **DON'T:**
- Worry about playing in "guitar range"
- Manually transpose MIDI clips
- Use Session Guitarist in Chord mode
- Disable Auto Transpose (unless you know why)

## Next Steps

1. Read [README.md](README.md) for detailed documentation
2. Experiment with different Play Modes
3. Try adjusting Strum Speed for feel
4. Use Humanize for more organic sound

That's it! You're ready to create guitar parts without owning a guitar. 🎸
