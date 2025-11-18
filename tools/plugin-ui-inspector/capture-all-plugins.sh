#!/bin/bash
# Capture all Audio Forge plugin UIs
# Opens each plugin and prompts for screenshot

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

OUTPUT_DIR="/tmp/plugin-screenshots/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$OUTPUT_DIR"

PLUGINS=(
    "twang-machine"
    "low-rider"
    "lonesome-picker"
    "gospel-wheels"
    "tubescreamer"
)

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}  Capture All Plugin UIs${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo
echo -e "Output: ${CYAN}$OUTPUT_DIR${NC}"
echo

for plugin in "${PLUGINS[@]}"; do
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}Plugin: $plugin${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo
    echo "1. Load $plugin in your DAW"
    echo "2. Position window clearly visible"
    echo "3. Set parameters to interesting/default values"
    echo
    read -p "Press RETURN when ready to capture..."
    
    echo
    echo -e "${CYAN}→${NC} Capturing in 3 seconds..."
    sleep 3
    
    # Capture with window selection
    screencapture -i -o "$OUTPUT_DIR/${plugin}.png"
    
    if [ -f "$OUTPUT_DIR/${plugin}.png" ]; then
        echo -e "${GREEN}✓${NC} Captured: ${plugin}.png"
    else
        echo -e "${YELLOW}⚠${NC} Skipped: ${plugin}.png"
    fi
    
    echo
done

echo
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✓${NC} All screenshots saved to:"
echo -e "  ${CYAN}$OUTPUT_DIR${NC}"
echo

# Generate index HTML
cat > "$OUTPUT_DIR/index.html" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>Audio Forge Plugin UIs</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            background: #1a1a1a;
            color: #e0e0e0;
            padding: 40px;
            max-width: 1200px;
            margin: 0 auto;
        }
        h1 {
            color: #4a9eff;
            border-bottom: 2px solid #333;
            padding-bottom: 20px;
        }
        .plugin {
            margin: 40px 0;
            padding: 20px;
            background: #252525;
            border-radius: 8px;
            border: 1px solid #333;
        }
        .plugin h2 {
            color: #ffa500;
            margin-top: 0;
        }
        .plugin img {
            max-width: 100%;
            border: 2px solid #333;
            border-radius: 4px;
            box-shadow: 0 4px 8px rgba(0,0,0,0.3);
        }
        .info {
            background: #2a2a2a;
            padding: 15px;
            border-radius: 4px;
            margin: 10px 0;
            border-left: 4px solid #4a9eff;
        }
    </style>
</head>
<body>
    <h1>🎸 Audio Forge Plugin UIs</h1>
    <div class="info">
        <strong>Captured:</strong> $(date)<br>
        <strong>Purpose:</strong> Visual inspection and documentation
    </div>
EOF

for plugin in "${PLUGINS[@]}"; do
    if [ -f "$OUTPUT_DIR/${plugin}.png" ]; then
        cat >> "$OUTPUT_DIR/index.html" << EOF
    <div class="plugin">
        <h2>${plugin}</h2>
        <img src="${plugin}.png" alt="${plugin} UI">
    </div>
EOF
    fi
done

cat >> "$OUTPUT_DIR/index.html" << 'EOF'
</body>
</html>
EOF

echo -e "${GREEN}✓${NC} Generated HTML index"
echo
echo "Open in browser:"
echo -e "  ${CYAN}open $OUTPUT_DIR/index.html${NC}"
echo

# Open index
open "$OUTPUT_DIR/index.html"
