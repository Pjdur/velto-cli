use std::process::Command;

#[test]
fn test_new_project() {
    let output = Command::new("cargo")
        .args(["run", "--", "new", "test-app"])
        .output()
        .expect("Failed to run velto new");

    assert!(output.status.success());
    assert!(std::path::Path::new("test-app/src/main.rs").exists());
}

#[test]
fn test_run_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "run", "--help"])
        .output()
        .expect("Failed to run velto run --help");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage"));
    assert!(output.status.success());
}
