use std::fs;
use std::process::Command;

/// Display version information about Velto CLI, Velto Framework, and Rust toolchain.
/// # Errors
/// This function will return an error if it fails to read the Cargo.lock file
/// or if it fails to execute the `rustc` command.
pub fn show_info() -> std::io::Result<()> {
    let cli_version = env!("CARGO_PKG_VERSION");

    // Read Cargo.lock and search for the velto package entry in a single pass.
    let cargo_lock = match fs::read_to_string("Cargo.lock") {
        Ok(content) => content,
        Err(_) => {
            println!("Velto Framework: unknown (Cargo.lock not found — try running `velto build` or `velto run` first)");
            return Ok(());
        }
    };

    let mut lines = cargo_lock.lines();

    let mut found_velto = false;
    let mut framework_version = "unknown".to_string();

    while let Some(line) = lines.next() {
        if line.contains("name = \"velto\"") {
            found_velto = true;
        } else if found_velto && line.contains("version = ") {
            framework_version = line.split('"').nth(1).unwrap_or("unknown").to_string();
            break;
        }
    }

    let rustc_output = Command::new("rustc").arg("--version").output()?;
    let rust_version = String::from_utf8_lossy(&rustc_output.stdout)
        .trim()
        .to_string();

    let target_output = Command::new("rustc").arg("-vV").output()?;
    let target_line = String::from_utf8_lossy(&target_output.stdout)
        .lines()
        .find(|line| line.starts_with("host:"))
        .map(|s| s.to_string())
        .unwrap_or_else(|| "host: unknown".to_string());

    println!("Velto CLI: {}", cli_version);
    println!("Velto Framework: {}", framework_version);
    println!("Rust: {}", rust_version);
    println!("{}", target_line);

    Ok(())
}
