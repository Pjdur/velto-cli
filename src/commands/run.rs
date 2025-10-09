use std::env;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use console::style;
use ctrlc;
use indicatif::{ProgressBar, ProgressStyle};

/// Runs a Velto app
///
/// # Arguments
/// * `port` - The port number on which the app will run, e.g. 3000
/// * `release` - Whether the app should run in release mode or not
///
/// # Returns
/// * `io::Result<()>` - Result indicating success or failure of the operation
pub fn run(port: u16, release: bool) -> io::Result<()> {
    if !std::path::Path::new("Cargo.toml").exists() {
        eprintln!("❌ You must run this command from inside your Velto app folder.");
        std::process::exit(1);
    }

    println!("{}", style("Starting Velto app... 🚀").bold());

    // Spinner setup
    let pb = ProgressBar::new_spinner();
    pb.set_message("Building Velto app...");
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["🌱", "🌿", "🌳", "🌲", "🌴"])
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    // Build the Velto app
    let start = Instant::now();
    let mut build_cmd = Command::new("cargo");
    build_cmd.arg("build");
    if release {
        build_cmd.arg("--release");
    }
    build_cmd
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    let duration = start.elapsed();

    // Finish spinner
    if duration > Duration::from_millis(500) {
        pb.finish_with_message("Build complete ✅");
    } else {
        pb.finish_and_clear();
        println!("{}", style("Build completed instantly ✅").green());
    }

    // Final messages
    println!();
    println!(
        "{}",
        style(format!("Velto app is running at http://localhost:{port}"))
            .cyan()
            .bold()
    );
    println!(
        "{}",
        style("Watching for changes in `static/` and `templates/`").dim()
    );
    println!("{}", style("Press Ctrl+C to stop the app.").dim());
    println!();

    // Get binary path
    let binary_path = get_binary_path(release);
    if !binary_path.exists() {
        eprintln!("❌ Compiled binary not found at {:?}", binary_path);
        std::process::exit(1);
    }

    // Run the Velto app directly
    let mut run_cmd = Command::new(binary_path);
    run_cmd
        .env("VELTO_DEV", "1")
        .env("VELTO_PORT", port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let child = run_cmd.spawn()?;
    let child_arc = Arc::new(Mutex::new(child));
    let child_for_signal = Arc::clone(&child_arc);

    // Handle Ctrl+C
    ctrlc::set_handler(move || {
        println!("\n🛑 Ctrl+C received. Shutting down Velto app...");
        if let Ok(mut child) = child_for_signal.lock() {
            let _ = child.kill();
        }
        std::process::exit(0);
    })
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Stream stdout
    if let Some(stdout) = child_arc.lock().unwrap().stdout.take() {
        let reader = BufReader::new(stdout);
        std::thread::spawn(move || {
            for line in reader.lines().flatten() {
                println!("{}", line);
            }
        });
    }

    // Stream stderr
    if let Some(stderr) = child_arc.lock().unwrap().stderr.take() {
        let reader = BufReader::new(stderr);
        std::thread::spawn(move || {
            for line in reader.lines().flatten() {
                eprintln!("⚠️ {}", line);
            }
        });
    }

    // Wait for the app to exit
    let status = child_arc.lock().unwrap().wait()?;
    let code = status.code().unwrap_or(1);

    // Gracefully handle Ctrl+C exit codes
    const CTRL_C_EXIT_WINDOWS: i32 = 0xC000013A_u32 as i32;
    const CTRL_C_EXIT_UNIX: i32 = 130;

    if (cfg!(windows) && code == CTRL_C_EXIT_WINDOWS) || (cfg!(unix) && code == CTRL_C_EXIT_UNIX) {
        println!("🛑 Velto app terminated by Ctrl+C.");
        return Ok(());
    }

    if !status.success() {
        eprintln!("Velto app exited with an error.");
        std::process::exit(code);
    }

    Ok(())
}

/// Determines the path to the compiled Velto binary based on platform and build mode.
///
/// Uses the current folder name as the binary name.
/// Appends `.exe` on Windows.
fn get_binary_path(release: bool) -> PathBuf {
    let binary_name = env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "velto-app".into());

    let binary_name = if cfg!(windows) {
        format!("{binary_name}.exe")
    } else {
        binary_name
    };

    let target_dir = if release { "release" } else { "debug" };
    PathBuf::from("target").join(target_dir).join(binary_name)
}
