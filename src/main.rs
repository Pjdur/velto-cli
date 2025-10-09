use clap::{Parser, Subcommand};
use velto_cli::commands::{create_project, run};

#[derive(Parser)]
#[command(
    name = "Velto",
    version = "0.1.0",
    about = "Velto CLI – Build blazing-fast Rust web apps",
    long_about = "Velto CLI helps you scaffold, run, and manage your Velto web apps with ease.\n\nUse `velto new <name>` to create a new project.\nUse `velto run` to launch your app.\n\nFor more commands and options, use `velto --help`.",
    after_help = "GitHub: https://github.com/pjdur/velto-cli\nFramework: https://github.com/pjdur/velto\n\nHappy coding! 🚀",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Velto project
    New {
        /// Name of the project
        name: String,
    },

    /// Run the Velto app
    Run {
        /// Port to run the app on (default: 8080)
        #[arg(long, default_value = "8080")]
        port: u16,

        /// Run in release mode
        #[arg(short, long)]
        release: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            if let Err(e) = create_project(&name) {
                println!("Creating Velto project '{}'...", name);
                eprintln!("Error creating project: {}", e);
            }
        }
        Commands::Run { port, release } => {
            println!("Starting Velto app on port {}...", port);
            if let Err(e) = run(port, release) {
                eprintln!("Error running Velto app: {}", e);
            }
        }
    }
}
