use clap::{Parser, Subcommand};
use colored::*;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "inklings")]
#[command(about = "Interactive ink! smart contract exercises", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify a single exercise
    Verify { exercise: String },
    /// Watch for changes and auto-verify
    Watch { exercise: Option<String> },
    /// Run a specific exercise
    Run { exercise: String },
    /// Show hint for an exercise
    Hint { exercise: String },
    /// List all exercises
    List,
    /// Reset progress
    Reset,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Verify { exercise }) => verify_exercise(&exercise),
        Some(Commands::Watch { exercise }) => watch_mode(exercise),
        Some(Commands::Run { exercise }) => run_exercise(&exercise),
        Some(Commands::Hint { exercise }) => show_hint(&exercise),
        Some(Commands::List) => list_exercises(),
        Some(Commands::Reset) => reset_progress(),
        None => interactive_mode(),
    }
}

fn verify_exercise(exercise: &str) {
    println!("{}", format!("Verifying {}...", exercise).cyan());

    let exercise_path = PathBuf::from("exercises").join(exercise);

    if !exercise_path.exists() {
        println!("{}", format!("Exercise '{}' not found!", exercise).red());
        return;
    }

    // Check if it compiles
    let output = Command::new("cargo")
        .args(&["build", "--manifest-path"])
        .arg(exercise_path.join("Cargo.toml"))
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("{}", "✓ Exercise compiled successfully!".green());

                // Run tests if they exist
                let test_output = Command::new("cargo")
                    .args(&["test", "--manifest-path"])
                    .arg(exercise_path.join("Cargo.toml"))
                    .output();

                if let Ok(test_output) = test_output {
                    if test_output.status.success() {
                        println!("{}", "✓ All tests passed!".green());
                        println!("\n{}", "🎉 Exercise completed! Move to the next one.".bright_green().bold());
                    } else {
                        println!("{}", "✗ Tests failed!".red());
                        println!("{}", String::from_utf8_lossy(&test_output.stderr));
                    }
                }
            } else {
                println!("{}", "✗ Compilation failed!".red());
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("{}", format!("Error running cargo: {}", e).red());
        }
    }
}

fn watch_mode(exercise: Option<String>) {
    println!("{}", "👀 Watching for changes...".cyan());
    println!("{}", "Press Ctrl+C to exit".dimmed());

    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    let watch_path = if let Some(ex) = &exercise {
        PathBuf::from("exercises").join(ex)
    } else {
        PathBuf::from("exercises")
    };

    watcher.watch(&watch_path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(_) => {
                if let Some(ref ex) = exercise {
                    println!("\n{}", "File changed, re-verifying...".yellow());
                    verify_exercise(ex);
                }
            }
            Err(_) => {}
        }
    }
}

fn run_exercise(exercise: &str) {
    println!("{}", format!("Running {}...", exercise).cyan());
    // Implementation for running contract on local node
    println!("{}", "Note: Contract execution requires a local node".yellow());
}

fn show_hint(exercise: &str) {
    println!("{}", format!("Hint for {}:", exercise).cyan());
    // Load hints from info/exercises.toml
    println!("{}", "Hints not yet implemented - check the exercise comments!".yellow());
}

fn list_exercises() {
    println!("{}", "Available exercises:".cyan().bold());
    println!("\n{}", "01_intro".yellow());
    println!("  intro1 - Your first ink! contract");
    println!("  intro2 - Adding storage");
}

fn reset_progress() {
    println!("{}", "Reset functionality not yet implemented".yellow());
}

fn interactive_mode() {
    println!("{}", r#"
  _       _    _ _                 
 (_)_ __ | | _| (_)_ __   __ _ ___ 
 | | '_ \| |/ / | | '_ \ / _` / __|
 | | | | |   <| | | | | | (_| \__ \
 |_|_| |_|_|\_\_|_|_| |_|\__, |___/
                         |___/     
"#.bright_cyan());

    println!("{}", "Interactive ink! smart contract exercises\n".cyan());
    println!("Run {} to get started!", "inklings list".green());
    println!("Run {} to verify your solution", "inklings verify <exercise>".green());
    println!("Run {} for auto-verification", "inklings watch <exercise>".green());
}
