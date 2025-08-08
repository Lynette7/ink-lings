// use clap::{Parser, Subcommand};
// use colored::*;
// use std::process;

mod exercise;
mod run;
mod verify;
// mod watch;

// use exercise::Exercise;

// #[derive(Parser)]
// #[command(name = "inklings")]
// #[command(about = "Small exercises to get you used to reading and writing ink! smart contracts")]
// struct Cli {
//     #[command(subcommand)]
//     command: Option<Commands>,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// Verify a single exercise
//     Verify { name: String },
//     /// Watch exercises and rerun on file changes
//     Watch,
//     /// Run a single exercise
//     Run { name: String },
//     /// Show hint for an exercise
//     Hint { name: String },
//     /// List all exercises
//     List,
//     /// Reset an exercise to its original state
//     Reset { name: String },
// }

// fn main() {
//     let cli = Cli::parse();

//     // Check if cargo-contract is installed
//     if !check_cargo_contract() {
//         eprintln!("{}", "Error: cargo-contract is not installed!".red().bold());
//         eprintln!("Please install it with: {}", "cargo install cargo-contract".yellow());
//         process::exit(1);
//     }

//     let exercises = match Exercise::load_exercises() {
//         Ok(exercises) => exercises,
//         Err(e) => {
//             eprintln!("{} {}", "Error loading exercises:".red().bold(), e);
//             process::exit(1);
//         }
//     };

//     match cli.command {
//         Some(Commands::Verify { name }) => {
//             if let Some(exercise) = exercises.iter().find(|e| e.name == name) {
//                 match verify::verify_exercise(exercise) {
//                     Ok(_) => println!("{} Exercise '{}' verified successfully!", "✓".green().bold(), name),
//                     Err(e) => {
//                         eprintln!("{} Exercise '{}' failed: {}", "✗".red().bold(), name, e);
//                         process::exit(1);
//                     }
//                 }
//             } else {
//                 eprintln!("{} Exercise '{}' not found", "✗".red().bold(), name);
//                 process::exit(1);
//             }
//         }
//         Some(Commands::Watch) => {
//             if let Err(e) = watch::watch_exercises(exercises) {
//                 eprintln!("{} Watch failed: {}", "✗".red().bold(), e);
//                 process::exit(1);
//             }
//         }
//         Some(Commands::Run { name }) => {
//             if let Some(exercise) = exercises.iter().find(|e| e.name == name) {
//                 match run::run_exercise(exercise) {
//                     Ok(_) => println!("{} Exercise '{}' ran successfully!", "✓".green().bold(), name),
//                     Err(e) => {
//                         eprintln!("{} Exercise '{}' failed: {}", "✗".red().bold(), name, e);
//                         process::exit(1);
//                     }
//                 }
//             } else {
//                 eprintln!("{} Exercise '{}' not found", "✗".red().bold(), name);
//                 process::exit(1);
//             }
//         }
//         Some(Commands::Hint { name }) => {
//             if let Some(exercise) = exercises.iter().find(|e| e.name == name) {
//                 println!("{}", "Hint:".yellow().bold());
//                 println!("{}", exercise.hint);
//             } else {
//                 eprintln!("{} Exercise '{}' not found", "✗".red().bold(), name);
//                 process::exit(1);
//             }
//         }
//         Some(Commands::List) => {
//             println!("{}", "Available exercises:".blue().bold());
//             for exercise in &exercises {
//                 println!("  {} - {} ({})", exercise.name.cyan(), exercise.path, exercise.mode);
//             }
//         }
//         Some(Commands::Reset { name: _ }) => {
//             println!("{}", "Reset functionality not implemented yet".yellow());
//         }
//         None => {
//             println!("{}", "Welcome to Inklings!".green().bold());
//             println!("Run {} to get started", "inklings list".cyan());
//         }
//     }
// }

// fn check_cargo_contract() -> bool {
//     which::which("cargo-contract").is_ok()
// }

fn main() {
    
}