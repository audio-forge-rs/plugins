use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("bundle") => bundle_all()?,
        Some("bundle-plugin") => {
            let plugin = env::args()
                .nth(2)
                .ok_or("Please specify a plugin name")?;
            bundle_plugin(&plugin)?;
        }
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:
  bundle              Build and bundle all plugins
  bundle-plugin NAME  Build and bundle a specific plugin
"
    );
}

fn bundle_all() -> Result<(), Box<dyn Error>> {
    let plugins_dir = project_root().join("plugins");
    
    println!("Building all plugins in release mode...");
    
    for entry in fs::read_dir(plugins_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            if let Some(plugin_name) = path.file_name().and_then(|n| n.to_str()) {
                println!("\n=== Building {} ===", plugin_name);
                bundle_plugin(plugin_name)?;
            }
        }
    }
    
    println!("\n✓ All plugins built successfully!");
    println!("Bundled plugins are in: target/bundled/");
    
    Ok(())
}

fn bundle_plugin(name: &str) -> Result<(), Box<dyn Error>> {
    let root = project_root();
    
    // Build the plugin
    let status = Command::new("cargo")
        .current_dir(&root)
        .args(&["build", "--release", "-p", &format!("audio-forge-{}", name)])
        .status()?;
    
    if !status.success() {
        return Err(format!("Failed to build plugin: {}", name).into());
    }
    
    // nih-plug automatically bundles plugins to target/bundled/ during the build
    println!("✓ {} built and bundled", name);
    
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
