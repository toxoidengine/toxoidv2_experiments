use clap::{Parser, Subcommand};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, Event};
use interprocess::local_socket::{Name, Stream};
use interprocess::local_socket::traits::Stream as StreamTrait;
use interprocess::local_socket::{prelude::*, GenericFilePath, GenericNamespaced};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::process::Command;
use std::io::prelude::*;
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[command(name = "toxoid_cli")]
#[command(about = "A CLI tool for Toxoid Engine, watching file changes and building WASM scripts, reloading them from the host", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Watch for file changes and build WASM
    Watch {
        /// Path to watch for changes
        #[arg(short, long, default_value = "app/guest")]
        path: String,

        /// Output path for the WASM file
        #[arg(short, long, default_value = "target/wasm32-wasip1/debug/guest.wasm")]
        output: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Watch { path, output } => {
            let (tx, rx) = channel();
            let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
            watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

            println!("Watching directory: {}", path);

            let debounce_duration = Duration::from_secs(2);
            let mut last_event_time = Instant::now();

            // Pick a name.
            let name = if GenericNamespaced::is_supported() {
                "toxoid.sock".to_ns_name::<GenericNamespaced>()?
            } else {
                "/tmp/toxoid.sock".to_fs_name::<GenericFilePath>()?
            };

            Command::new("cargo")
                .args(&["run", "--package", "host"])
                .status()
                .expect("Failed to run host");

            for res in rx {
                match res {
                    Ok(Event { paths, .. }) if !paths.is_empty() => {
                        let now = Instant::now();
                        if now.duration_since(last_event_time) >= debounce_duration {
                            last_event_time = now;
                            println!("File change detected, building WASM...");

                            Command::new("cargo")
                                .args(&["component", "build"])
                                .current_dir(path)
                                .status()
                                .expect("Failed to build WASM");

                            println!("Copying WASM file to {:?}", output);
                            std::fs::copy("target/wasm32-wasip1/debug/guest.wasm", output)
                                .expect("Failed to copy WASM file");

                            let mut conn = Stream::connect(name.clone())?;
                            conn.write_all(b"Reload WASM script\n")?;
                            println!("Sent reload message to host");
                        }
                    }
                    Err(e) => println!("Watch error: {:?}", e),
                    _ => (),
                }
            }
        }
    }

    Ok(())
}