use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Builds the Velto project using Cargo
/// # Arguments
/// * `release` - Whether to build in release mode
/// * `target` - Optional target triple for cross-compilation
/// * `output` - Optional output directory to copy the binary
/// * `quiet` - Whether to suppress Cargo output
/// * `copy_assets` - Whether to copy asset directories to the output directory
/// Returns `Ok(())` if the build succeeds, otherwise returns an `Err`
/// # Example
/// ```no_run
/// use velto_cli::commands::build::build_project;
/// build_project(true, Some("x86_64-unknown-linux-gnu".to_string()), Some(std::path::PathBuf::from("dist")), false, true).unwrap();
/// ```
pub fn build_project(
    release: bool,
    target: Option<String>,
    output: Option<PathBuf>,
    quiet: bool,
    copy_assets: bool,
) -> std::io::Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    if release {
        cmd.arg("--release");
    }

    if let Some(t) = &target {
        cmd.arg("--target").arg(t);
    }

    if quiet {
        cmd.arg("--quiet");
    }

    let status = cmd.status()?;
    if !status.success() {
        eprintln!("❌ Build failed");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Build failed",
        ));
    }

    println!("✅ Build succeeded");

    if let Some(out_dir) = output {
        let binary_name = std::env::current_dir()?
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let mut bin_path = PathBuf::from("target");

        if let Some(t) = &target {
            bin_path.push(t);
        }

        bin_path.push(if release { "release" } else { "debug" });
        bin_path.push(&binary_name);

        if !bin_path.exists() {
            eprintln!("❌ Could not find built binary at {}", bin_path.display());
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Binary not found",
            ));
        }

        fs::create_dir_all(&out_dir)?;
        fs::copy(&bin_path, out_dir.join(&binary_name))?;

        if copy_assets {
            let asset_dirs = ["templates", "static"];
            for dir in asset_dirs {
                let src = PathBuf::from(dir);
                if src.exists() {
                    let dest = out_dir.join(dir);
                    fs::create_dir_all(&dest)?;
                    for entry in fs::read_dir(&src)? {
                        let entry = entry?;
                        let path = entry.path();
                        if path.is_file() {
                            fs::copy(&path, dest.join(path.file_name().unwrap()))?;
                        }
                    }
                    println!("📁 Copied {} to {}", dir, dest.display());
                } else {
                    println!("⚠️  Asset directory '{}' not found", dir);
                }
            }
        }
        println!("📦 Binary copied to {}", out_dir.display());
    }
    Ok(())
}
