#!/bin/bash
# Plugin UI Screenshot Tool
# Captures plugin UIs for visual inspection and documentation

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="${1:-/tmp/plugin-screenshots}"
PLUGIN_NAME="${2:-all}"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}  Audio Forge Plugin UI Inspector${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo

# Create output directory
mkdir -p "$OUTPUT_DIR"
echo -e "${GREEN}✓${NC} Output directory: $OUTPUT_DIR"
echo

echo -e "${YELLOW}Instructions for Screenshot Capture:${NC}"
echo "1. Open your DAW (Bitwig, Ableton, Reaper, etc.)"
echo "2. Load the plugin you want to capture"
echo "3. Position the plugin window clearly visible"
echo "4. Press RETURN when ready to capture..."
echo
read -p "Press RETURN to continue..."

echo
echo -e "${CYAN}→${NC} Capturing screenshot in 3 seconds..."
sleep 3

# Capture screenshot
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
FILENAME="${OUTPUT_DIR}/plugin_${PLUGIN_NAME}_${TIMESTAMP}.png"

# Interactive mode - let user select window
screencapture -i -o "$FILENAME"

if [ -f "$FILENAME" ]; then
    echo -e "${GREEN}✓${NC} Screenshot saved: $FILENAME"
    
    # Get image dimensions
    if command -v sips &> /dev/null; then
        DIMENSIONS=$(sips -g pixelWidth -g pixelHeight "$FILENAME" | grep -E "pixelWidth|pixelHeight" | awk '{print $2}')
        WIDTH=$(echo "$DIMENSIONS" | sed -n 1p)
        HEIGHT=$(echo "$DIMENSIONS" | sed -n 2p)
        echo -e "${CYAN}  Dimensions: ${WIDTH}x${HEIGHT}${NC}"
    fi
    
    # Open for preview
    echo
    echo -e "${CYAN}→${NC} Opening preview..."
    open "$FILENAME"
    
    echo
    echo -e "${GREEN}✓${NC} Screenshot complete!"
    echo
    echo "To capture another plugin:"
    echo "  $0 $OUTPUT_DIR [plugin-name]"
else
    echo -e "${RED}✗${NC} Screenshot failed or cancelled"
    exit 1
fi
