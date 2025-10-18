# Velto CLI 🚀

**Velto CLI** is the official command-line tool for [Velto](https://github.com/pjdur/velto), a fast and minimal Rust web framework. It helps you scaffold, run, and manage Velto apps with ease.

---

## ✨ Features

- `velto new <name>` — Create a new Velto project instantly  
- `velto run` — Build and launch your app with clean output  
- `velto build` — Compile your app and optionally package it  
- `velto info` — Show your Velto environment and setup  
- LiveReload support in dev mode  
- Graceful shutdown on Ctrl+C  
- No noisy Cargo logs — just your app

---

## 📦 Installation

```bash
cargo install velto-cli
```

---

## 🚀 Usage

### Create a new project

```bash
velto new my-app
```

This generates:

- `src/main.rs` with a sample route  
- `templates/index.html` with dynamic content  
- `static/` for assets like CSS and JS  
- `Cargo.toml` with Velto dependencies

---

### Run your app

```bash
velto run --port 3000
```

Options:

- `--port <PORT>` — Set the port (default: 8080)  
- `-r`, `--release` — Run in release mode

---

### Build your app

```bash
velto build --release --output dist/ --copy-assets
```

Options:

- `-r`, `--release` — Build in release mode  
- `--target <TRIPLE>` — Cross-compile for a specific target  
- `--output <DIR>` — Copy the binary to a custom directory  
- `--quiet` — Suppress Cargo output  
- `--copy-assets` — Copy `templates/` and `static/` into the output directory

> Note: Running the binary from a custom output directory may require copying assets or setting environment paths.

---

### Show environment info

```bash
velto info
```

Outputs:

- Velto CLI version  
- Velto Framework version (from `Cargo.lock`)  
- Rust version  
- Target architecture

> Helpful for debugging, support, and verifying your setup.

---

## 🧪 CLI Tests

You can run these tests using `cargo test` inside the `velto-cli` repo.

### Example: `tests/cli.rs`

```rust
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
```

---

## 📄 License

MIT

---

## 🔗 Links

- [Velto Framework](https://github.com/pjdur/velto)
- [Velto CLI](https://github.com/pjdur/velto-cli)

---

Happy building with Velto! ⚡
