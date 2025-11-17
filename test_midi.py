#!/usr/bin/env python3
"""
Simple MIDI test generator for testing Audio Forge plugins
Generates MIDI files with chord progressions at various tempos
"""

import sys
from midiutil import MIDIFile

def create_test_midi(filename, tempo=100):
    """Create a simple chord progression MIDI file"""
    track = 0
    channel = 0
    time = 0
    duration = 4  # 4 beats per chord
    volume = 100
    
    midi = MIDIFile(1)
    midi.addTempo(track, time, tempo)
    
    # Simple I-IV-V-I progression in C major
    # Each chord held for 4 beats
    chords = [
        [60, 64, 67],  # C major (C-E-G)
        [65, 69, 72],  # F major (F-A-C)
        [67, 71, 74],  # G major (G-B-D)
        [60, 64, 67],  # C major (C-E-G)
    ]
    
    for i, chord in enumerate(chords):
        chord_time = i * duration
        for note in chord:
            midi.addNote(track, channel, note, chord_time, duration, volume)
    
    with open(filename, 'wb') as f:
        midi.writeFile(f)
    
    print(f"Created {filename}")
    print(f"Tempo: {tempo} BPM")
    print(f"Progression: C - F - G - C (4 beats each)")
    print(f"Total duration: 16 beats ({16 * 60 / tempo:.1f} seconds)")

if __name__ == "__main__":
    # Create test files at different tempos
    create_test_midi("test_slow.mid", tempo=80)
    create_test_midi("test_medium.mid", tempo=100)
    create_test_midi("test_fast.mid", tempo=120)
    
    print("\nUse these MIDI files to test bass/guitar/banjo timing")
    print("Import into your DAW and route through plugins")
