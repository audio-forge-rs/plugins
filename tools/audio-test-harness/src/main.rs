use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use hound::{WavReader, WavWriter, WavSpec};
use std::path::PathBuf;

mod generators;
mod analyzers;
mod midi;

#[derive(Parser)]
#[command(name = "audio-test-harness")]
#[command(about = "Test Audio Forge plugins with CLI audio generation and analysis")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate test audio files
    Generate {
        #[command(subcommand)]
        generator: GeneratorType,
    },
    
    /// Analyze audio files
    Analyze {
        /// Input audio file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Analysis type
        #[command(subcommand)]
        analysis: AnalysisType,
    },
    
    /// Test MIDI processor plugins
    TestMidi {
        /// Plugin CLAP file
        #[arg(short, long)]
        plugin: PathBuf,
        
        /// MIDI input file
        #[arg(short, long)]
        midi: PathBuf,
        
        /// Virtual instrument for rendering (fluidsynth soundfont)
        #[arg(short, long)]
        soundfont: Option<PathBuf>,
        
        /// Output WAV file
        #[arg(short, long)]
        output: PathBuf,
        
        /// Tempo (BPM)
        #[arg(short, long, default_value = "100")]
        tempo: u32,
    },
    
    /// Test audio effect plugins
    TestEffect {
        /// Plugin CLAP file
        #[arg(short, long)]
        plugin: PathBuf,
        
        /// Input audio file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output WAV file
        #[arg(short, long)]
        output: PathBuf,
        
        /// Show analysis after processing
        #[arg(short, long)]
        analyze: bool,
    },
    
    /// Run comprehensive test suite
    TestAll {
        /// Directory containing CLAP plugins
        #[arg(short, long)]
        plugins_dir: PathBuf,
        
        /// Output directory for test results
        #[arg(short, long)]
        output_dir: PathBuf,
    },
    
    /// Capture plugin UI screenshots
    CaptureUi {
        /// Plugin name (e.g., "twang-machine", "low-rider")
        #[arg(short, long)]
        plugin: Option<String>,
        
        /// Output directory for screenshots
        #[arg(short, long, default_value = "/tmp/plugin-screenshots")]
        output: PathBuf,
        
        /// Capture all plugins
        #[arg(short, long)]
        all: bool,
    },
}

#[derive(Subcommand)]
enum GeneratorType {
    /// Generate sine wave test tone
    Sine {
        /// Frequency in Hz
        #[arg(short, long, default_value = "440")]
        freq: f32,
        
        /// Duration in seconds
        #[arg(short, long, default_value = "2.0")]
        duration: f32,
        
        /// Output file
        #[arg(short, long)]
        output: PathBuf,
    },
    
    /// Generate MIDI test file
    Midi {
        /// Chord progression (e.g., "C,F,G,C")
        #[arg(short, long)]
        chords: String,
        
        /// Tempo (BPM)
        #[arg(short, long, default_value = "100")]
        tempo: u32,
        
        /// Beats per chord
        #[arg(short, long, default_value = "4")]
        beats: u32,
        
        /// Output MIDI file
        #[arg(short, long)]
        output: PathBuf,
    },
    
    /// Generate white noise
    Noise {
        /// Duration in seconds
        #[arg(short, long, default_value = "2.0")]
        duration: f32,
        
        /// Output file
        #[arg(short, long)]
        output: PathBuf,
    },
    
    /// Generate impulse (for measuring plugin response)
    Impulse {
        /// Output file
        #[arg(short, long)]
        output: PathBuf,
    },
}

#[derive(Subcommand)]
enum AnalysisType {
    /// Show waveform statistics
    Stats,
    
    /// Analyze frequency spectrum (FFT)
    Spectrum,
    
    /// Measure timing/rhythm (for MIDI processor output)
    Timing,
    
    /// Detect peaks and dynamics
    Dynamics,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Generate { generator } => {
            handle_generate(generator)?;
        }
        
        Commands::Analyze { input, analysis } => {
            handle_analyze(input, analysis)?;
        }
        
        Commands::TestMidi { plugin, midi, soundfont, output, tempo } => {
            handle_test_midi(plugin, midi, soundfont, output, tempo)?;
        }
        
        Commands::TestEffect { plugin, input, output, analyze } => {
            handle_test_effect(plugin, input, output, analyze)?;
        }
        
        Commands::TestAll { plugins_dir, output_dir } => {
            handle_test_all(plugins_dir, output_dir)?;
        }
        
        Commands::CaptureUi { plugin, output, all } => {
            handle_capture_ui(plugin, output, all)?;
        }
    }
    
    Ok(())
}

fn handle_generate(generator: GeneratorType) -> Result<()> {
    match generator {
        GeneratorType::Sine { freq, duration, output } => {
            generators::generate_sine(freq, duration, &output)?;
            println!("{} Generated sine wave: {} Hz, {} seconds", 
                "✓".green().bold(), freq, duration);
        }
        
        GeneratorType::Midi { chords, tempo, beats, output } => {
            generators::generate_midi(&chords, tempo, beats, &output)?;
            println!("{} Generated MIDI file: {} @ {} BPM", 
                "✓".green().bold(), chords, tempo);
        }
        
        GeneratorType::Noise { duration, output } => {
            generators::generate_noise(duration, &output)?;
            println!("{} Generated white noise: {} seconds", 
                "✓".green().bold(), duration);
        }
        
        GeneratorType::Impulse { output } => {
            generators::generate_impulse(&output)?;
            println!("{} Generated impulse response test signal", 
                "✓".green().bold());
        }
    }
    
    Ok(())
}

fn handle_analyze(input: PathBuf, analysis: AnalysisType) -> Result<()> {
    let reader = WavReader::open(&input)
        .context("Failed to open audio file")?;
    
    match analysis {
        AnalysisType::Stats => {
            analyzers::analyze_stats(&input)?;
        }
        
        AnalysisType::Spectrum => {
            analyzers::analyze_spectrum(&input)?;
        }
        
        AnalysisType::Timing => {
            analyzers::analyze_timing(&input)?;
        }
        
        AnalysisType::Dynamics => {
            analyzers::analyze_dynamics(&input)?;
        }
    }
    
    Ok(())
}

fn handle_test_midi(
    plugin: PathBuf,
    midi: PathBuf,
    soundfont: Option<PathBuf>,
    output: PathBuf,
    tempo: u32,
) -> Result<()> {
    println!("{} Testing MIDI processor plugin...", "→".cyan().bold());
    println!("  Plugin: {}", plugin.display());
    println!("  MIDI: {}", midi.display());
    println!("  Tempo: {} BPM", tempo);
    
    // For now, provide instructions for manual testing
    // TODO: Implement CLAP plugin hosting and MIDI processing
    
    println!("\n{} {}", "⚠".yellow().bold(), "Manual testing required:".yellow());
    println!("  1. Load {} in your DAW", plugin.display());
    println!("  2. Route MIDI file {} through plugin", midi.display());
    println!("  3. Send output to FluidSynth or virtual instrument");
    println!("  4. Render to {}", output.display());
    
    println!("\n{} {}", "→".cyan(), "FluidSynth command:");
    if let Some(sf) = soundfont {
        println!("  fluidsynth -ni {} {} -F {} -r 48000", 
            sf.display(), midi.display(), output.display());
    }
    
    Ok(())
}

fn handle_test_effect(
    plugin: PathBuf,
    input: PathBuf,
    output: PathBuf,
    analyze: bool,
) -> Result<()> {
    println!("{} Testing audio effect plugin...", "→".cyan().bold());
    println!("  Plugin: {}", plugin.display());
    println!("  Input: {}", input.display());
    println!("  Output: {}", output.display());
    
    // TODO: Implement CLAP plugin hosting for effects
    
    println!("\n{} {}", "⚠".yellow().bold(), "Manual testing required:".yellow());
    println!("  1. Load {} in your DAW", plugin.display());
    println!("  2. Process {} through plugin", input.display());
    println!("  3. Render to {}", output.display());
    
    if analyze {
        println!("\n{} Run analysis after rendering:", "→".cyan());
        println!("  cargo run --bin audio-test-harness -- analyze -i {} stats", 
            output.display());
    }
    
    Ok(())
}

fn handle_test_all(plugins_dir: PathBuf, output_dir: PathBuf) -> Result<()> {
    println!("{} Running comprehensive plugin test suite...", "→".cyan().bold());
    println!("  Plugins: {}", plugins_dir.display());
    println!("  Output: {}", output_dir.display());
    
    // Find all CLAP plugins
    let plugins = std::fs::read_dir(&plugins_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "clap")
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    
    println!("\n{} Found {} plugins", "✓".green().bold(), plugins.len());
    
    for plugin in plugins {
        let name = plugin.file_name();
        println!("\n  {} Testing: {}", "→".cyan(), name.to_string_lossy());
        
        // Determine plugin type and run appropriate tests
        // TODO: Implement automatic test detection
    }
    
    Ok(())
}

fn handle_capture_ui(plugin: Option<String>, output: PathBuf, all: bool) -> Result<()> {
    use std::process::Command;
    
    println!("{} Plugin UI Screenshot Capture", "📸".bold());
    println!();
    
    std::fs::create_dir_all(&output)?;
    
    if all {
        // Run the capture-all script
        println!("{} Launching capture-all script...", "→".cyan());
        println!("  This will guide you through capturing all plugin UIs");
        println!();
        
        let script_path = "tools/plugin-ui-inspector/capture-all-plugins.sh";
        
        Command::new("bash")
            .arg(script_path)
            .status()
            .context("Failed to run capture-all script")?;
            
    } else if let Some(name) = plugin {
        // Capture single plugin
        println!("{} Capturing: {}", "→".cyan(), name.green());
        println!();
        println!("{}  Instructions:", "📋".bold());
        println!("  1. Open your DAW");
        println!("  2. Load the {} plugin", name);
        println!("  3. Position the plugin window clearly");
        println!("  4. When ready, you'll select the window to capture");
        println!();
        
        println!("Press RETURN when ready...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        println!("{} Capturing in 3 seconds...", "→".cyan());
        std::thread::sleep(std::time::Duration::from_secs(3));
        
        let filename = output.join(format!("{}.png", name));
        
        Command::new("screencapture")
            .arg("-i")  // Interactive mode
            .arg("-o")  // Only capture window, not shadow
            .arg(&filename)
            .status()
            .context("Failed to run screencapture")?;
        
        if filename.exists() {
            println!("{} Screenshot saved: {}", "✓".green().bold(), filename.display());
            
            // Get dimensions
            if let Ok(output) = Command::new("sips")
                .arg("-g")
                .arg("pixelWidth")
                .arg("-g")
                .arg("pixelHeight")
                .arg(&filename)
                .output()
            {
                let info = String::from_utf8_lossy(&output.stdout);
                println!("{}", info);
            }
            
            // Open preview
            Command::new("open").arg(&filename).status().ok();
            
        } else {
            println!("{} Screenshot cancelled or failed", "⚠".yellow());
        }
        
    } else {
        println!("{} Please specify --plugin NAME or --all", "⚠".yellow());
        println!();
        println!("Examples:");
        println!("  audio-test-harness capture-ui --plugin twang-machine");
        println!("  audio-test-harness capture-ui --all");
    }
    
    Ok(())
}
