#!/bin/bash
# Audio Forge Plugin Testing Script
# Tests all plugins with audio generation and analysis

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

TEST_DIR="/tmp/audio-forge-tests"
HARNESS="./target/release/audio-test-harness"

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}  Audio Forge Plugin Test Suite${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo

# Create test directory
mkdir -p "$TEST_DIR"
echo -e "${GREEN}✓${NC} Created test directory: $TEST_DIR"

# Build test harness
echo -e "\n${CYAN}→${NC} Building test harness..."
cargo build --release -p audio-test-harness
echo -e "${GREEN}✓${NC} Test harness built"

# Generate test audio
echo -e "\n${CYAN}→${NC} Generating test audio files..."

# 1. Sine waves at different frequencies (for Tube Screamer testing)
$HARNESS generate sine --freq 110 --duration 2.0 --output "$TEST_DIR/sine_110hz.wav"
$HARNESS generate sine --freq 220 --duration 2.0 --output "$TEST_DIR/sine_220hz.wav"
$HARNESS generate sine --freq 440 --duration 2.0 --output "$TEST_DIR/sine_440hz.wav"
echo -e "${GREEN}✓${NC} Generated sine waves"

# 2. White noise (for effect testing)
$HARNESS generate noise --duration 2.0 --output "$TEST_DIR/noise.wav"
echo -e "${GREEN}✓${NC} Generated white noise"

# 3. Impulse (for measuring plugin response)
$HARNESS generate impulse --output "$TEST_DIR/impulse.wav"
echo -e "${GREEN}✓${NC} Generated impulse response"

# Generate MIDI test files
echo -e "\n${CYAN}→${NC} Generating MIDI test files..."

# Alt-country progressions at different tempos
$HARNESS generate midi --chords "C,F,G,C" --tempo 80 --beats 4 --output "$TEST_DIR/progression_80bpm.mid"
$HARNESS generate midi --chords "C,F,G,C" --tempo 100 --beats 4 --output "$TEST_DIR/progression_100bpm.mid"
$HARNESS generate midi --chords "C,F,G,C" --tempo 120 --beats 4 --output "$TEST_DIR/progression_120bpm.mid"
echo -e "${GREEN}✓${NC} Generated MIDI progressions"

# More complex progression
$HARNESS generate midi --chords "C,Am,F,G" --tempo 100 --beats 4 --output "$TEST_DIR/progression_complex.mid"
echo -e "${GREEN}✓${NC} Generated complex progression"

echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}  Test Files Generated${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo
echo "Audio files:"
ls -lh "$TEST_DIR"/*.wav
echo
echo "MIDI files:"
ls -lh "$TEST_DIR"/*.mid

echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}  Testing Instructions${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo
echo -e "${YELLOW}MIDI Processor Plugins${NC} (Twang Machine, Low Rider, Lonesome Picker, Gospel Wheels):"
echo "  1. Load plugin in your DAW"
echo "  2. Import MIDI file (e.g., $TEST_DIR/progression_100bpm.mid)"
echo "  3. Route plugin → virtual instrument (Scarbee Rick, Session Guitarist, etc.)"
echo "  4. Play and listen - check timing feels RIGHT"
echo "  5. Verify tempo matches (100 BPM should feel like 100 BPM)"
echo
echo -e "${YELLOW}Audio Effect Plugins${NC} (Tube Screamer):"
echo "  1. Load plugin in your DAW"
echo "  2. Import test audio (e.g., $TEST_DIR/sine_440hz.wav)"
echo "  3. Process through plugin"
echo "  4. Listen for expected character"
echo "  5. Analyze output:"
echo "     $HARNESS analyze -i OUTPUT.wav stats"
echo "     $HARNESS analyze -i OUTPUT.wav spectrum"
echo
echo -e "${YELLOW}Quick Analysis Commands:${NC}"
echo "  # Analyze sine wave"
echo "  $HARNESS analyze -i $TEST_DIR/sine_440hz.wav stats"
echo "  $HARNESS analyze -i $TEST_DIR/sine_440hz.wav spectrum"
echo
echo "  # Check dynamics"
echo "  $HARNESS analyze -i YOUR_RENDERED_FILE.wav dynamics"
echo
echo "  # Measure timing (for bass/banjo/guitar output)"
echo "  $HARNESS analyze -i YOUR_RENDERED_FILE.wav timing"
echo
echo -e "${GREEN}✓${NC} Test suite ready!"
echo -e "  Test files: $TEST_DIR"
echo
