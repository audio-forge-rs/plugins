# Plugin UI Inspector

Tools for capturing and inspecting Audio Forge plugin UIs during development.

## Why This Matters

When developing plugins, I need to **SEE** the UI to verify:
- ✅ Parameters are displayed correctly
- ✅ Colors match the intended theme
- ✅ Layout is clean and professional
- ✅ Text is readable
- ✅ Controls are appropriately sized
- ✅ Plugin branding is consistent

Without visual inspection, UI bugs can ship to production.

## Tools Provided

### 1. Single Plugin Screenshot

Capture one plugin UI at a time:

```bash
# Using test harness
./target/release/audio-test-harness capture-ui --plugin twang-machine

# Or directly
./tools/plugin-ui-inspector/screenshot-plugin.sh /tmp/screenshots twang-machine
```

**Workflow:**
1. Script prompts you
2. Load plugin in DAW
3. Position window
4. Press RETURN
5. Script captures after 3 seconds
6. Window selection mode (click to select)
7. Screenshot saved and opened

### 2. Capture All Plugins

Systematically capture all plugin UIs:

```bash
# Using test harness
./target/release/audio-test-harness capture-ui --all

# Or directly
./tools/plugin-ui-inspector/capture-all-plugins.sh
```

**Workflow:**
1. Script lists all plugins
2. For each plugin:
   - Load in DAW
   - Position window
   - Press RETURN
   - Capture window
3. Generates HTML index page
4. Opens browser with all screenshots

### 3. Automated Inspection

Screenshots are saved with metadata:
- Timestamp
- Plugin name
- Dimensions
- Organized in dated directories

## Output Locations

**Default:**
```
/tmp/plugin-screenshots/
├── YYYYMMDD_HHMMSS/
│   ├── twang-machine.png
│   ├── low-rider.png
│   ├── lonesome-picker.png
│   ├── gospel-wheels.png
│   ├── tubescreamer.png
│   └── index.html          # Visual index
```

**Custom:**
```bash
audio-test-harness capture-ui --plugin NAME --output ~/my-screenshots
```

## Usage Examples

### Verify UI After Code Changes

```bash
# 1. Make UI changes to plugin
vim plugins/twang-machine/src/editor.rs

# 2. Build plugin
cargo xtask bundle audio-forge-twang-machine --release

# 3. Capture UI
./target/release/audio-test-harness capture-ui --plugin twang-machine

# 4. Visual inspection
# Screenshot opens automatically
# Check: colors, layout, text, parameters
```

### Document All Plugin UIs

```bash
# Capture all for documentation
./target/release/audio-test-harness capture-ui --all

# HTML index generated at:
# /tmp/plugin-screenshots/YYYYMMDD_HHMMSS/index.html
```

### Compare UI Changes

```bash
# Before changes
./target/release/audio-test-harness capture-ui --plugin twang-machine --output /tmp/ui-before

# Make changes and rebuild
# ...

# After changes  
./target/release/audio-test-harness capture-ui --plugin twang-machine --output /tmp/ui-after

# Compare side-by-side
open /tmp/ui-before/twang-machine.png
open /tmp/ui-after/twang-machine.png
```

## Integration with Test Harness

The UI capture is fully integrated into the audio test harness:

```bash
# Complete testing workflow
./tools/test-plugins.sh                           # Generate test files
cargo xtask bundle <plugin> --release             # Build plugin
./target/release/audio-test-harness capture-ui --plugin <plugin>  # Capture UI
# Test in DAW...
./target/release/audio-test-harness analyze -i output.wav timing  # Analyze
```

## Tips

### Best Practices

1. **Clean background** - Use simple desktop, no clutter
2. **Consistent lighting** - Screenshots at same time of day
3. **Standard zoom** - 100% zoom in DAW
4. **Capture window only** - Use `-i` flag (already set)
5. **Meaningful states** - Set parameters to interesting values

### What to Check

When inspecting screenshots:

- ✅ **Readability** - All text clear at plugin size
- ✅ **Alignment** - Parameters aligned properly
- ✅ **Colors** - Match intended theme (check CSS)
- ✅ **Spacing** - Not cramped, not too sparse
- ✅ **Contrast** - Labels readable against background
- ✅ **Consistency** - Similar layout across plugins

### Common Issues to Catch

- ❌ Text too small
- ❌ Colors too similar (low contrast)
- ❌ Parameters cut off
- ❌ Labels misaligned
- ❌ Incorrect fonts
- ❌ Wrong dimensions

## macOS Screenshot Tools

These scripts use built-in macOS tools:

- **screencapture** - Capture windows/regions
  - `-i` = Interactive window selection
  - `-o` = Only window (no shadow)
  
- **sips** - Get image dimensions
  
- **open** - Preview screenshots

All tools pre-installed on macOS - no dependencies needed!

## Future Enhancements

### Automated UI Testing

```rust
// Future: Automated UI verification
#[test]
fn test_twang_machine_ui() {
    let screenshot = capture_plugin_ui("twang-machine");
    
    // Verify dimensions
    assert_eq!(screenshot.width, 600);
    assert_eq!(screenshot.height, 400);
    
    // Color analysis
    assert!(has_rust_orange(&screenshot));
    assert!(has_denim_blue(&screenshot));
    
    // Text detection
    assert!(contains_text(&screenshot, "Twang Machine"));
    assert!(contains_text(&screenshot, "Strum Speed"));
}
```

### Regression Testing

```bash
# Compare against reference screenshots
audio-test-harness ui-diff \
  --reference tests/ui-references/ \
  --current /tmp/plugin-screenshots/ \
  --tolerance 5%
```

### Accessibility Checks

- Color contrast ratios
- Font size verification
- Layout responsiveness

## Integration with CI/CD

```yaml
# .github/workflows/ui-test.yml
- name: Capture Plugin UIs
  run: |
    ./tools/plugin-ui-inspector/capture-all-plugins.sh
    
- name: Upload UI Screenshots
  uses: actions/upload-artifact@v2
  with:
    name: plugin-uis
    path: /tmp/plugin-screenshots/
```

## Development Loop

```
CODE UI → BUILD → CAPTURE → INSPECT → ITERATE
  ↑                                          ↓
  └──────────── VISUAL FEEDBACK ─────────────┘
```

Now I can **SEE** the plugins during development!

---

**Remember: The best way to verify UI is to look at it with your eyes!** 👁️
