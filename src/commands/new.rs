use std::path::Path;
use std::{fs, io};

/// Creates a new Velto project with the given name.
/// This sets up a basic directory structure and a sample `main.rs` file along with a `templates` and `static` directory.
/// # Arguments
/// * `name` - The name of the new project.
/// # Returns
/// * `io::Result<()>` - Result indicating success or failure.
/// # Example
/// ```
/// create_project("my_velto_app")?;
/// ```
pub fn create_project(name: &str) -> io::Result<()> {
    let path = Path::new(name);
    fs::create_dir_all(path.join("src"))?;
    fs::create_dir_all(path.join("templates"))?;
    fs::create_dir_all(path.join("static"))?;

    fs::write(
        path.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
velto = "1.6.0"
tokio = {{ version = "1", features = ["full"] }}
"#,
            name
        ),
    )?;

    fs::write(
        path.join("src").join("main.rs"),
        r#"use velto::prelude::*;

fn homepage(_req: &Request) -> Response {
    render!("index.html", {
        "title" => "My Velto App",
        "message" => "Hello, Velto!"
    })
}

#[tokio::main]
async fn main() {
    let mut app = App::new();
    app.enable_dev_mode();
    route!(app, "/" => homepage);
    app.serve_static("static");

    if std::env::var("VELTO_DEV").is_ok() {
        app.enable_dev_mode();
    }

    let port = std::env::var("VELTO_PORT").unwrap_or_else(|_| "8080".into());
    if let Err(e) = app.run(&format!("127.0.0.1:{}", port)).await {
        eprintln!("Server failed to start: {}", e);
    }
}
"#,
    )?;

    fs::write(
        path.join("templates").join("index.html"),
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }}</title>
    <link href="/app.css" rel="stylesheet" />
</head>
<body>
    <h1>{{ message }}</h1>
    <p>Welcome to your Velto app. Start building something amazing!</p>
    <p>
        <a href="https://github.com/pjdur/velto" target="_blank">Velto Framework</a> |
        <a href="https://crates.io/crates/velto-cli" target="_blank">Velto CLI</a>
    </p>
</body>
</html>
"#,
    )?;

    fs::write(
        path.join("static").join("app.css"),
        r#"body {
    margin: 0;
    padding: 0;
    font-family: system-ui, sans-serif;
    background: #f5f7fa;
    color: #333;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
}

h1 {
    font-size: 2.5rem;
    color: #4f46e5;
    margin-bottom: 1rem;
}

a {
    color: #4f46e5;
    text-decoration: none;
    font-weight: bold;
}

a:hover {
    text-decoration: underline;
}
"#,
    )?;

    println!("✅ Project '{}' created!", name);
    println!("You can run `cd {}`", name);
    println!("Use `velto run` to run your Velto app");
    Ok(())
}
