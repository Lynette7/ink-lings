use clap::{Parser, Subcommand};
use colored::*;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct ExerciseInfo {
    id: String,
    name: String,
    path: String,
    mode: String,
    hint: String,
}

#[derive(Debug, Deserialize)]
struct ExercisesConfig {
    exercises: Vec<ExerciseInfo>,
    #[serde(default)]
    categories: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ProgressData {
    completed: HashSet<String>,
    #[serde(default)]
    attempts: HashMap<String, u32>,
    #[serde(default)]
    last_worked_on: Option<String>,
}

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
    /// Show your progress
    Progress,
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
        Some(Commands::Progress) => show_progress(),
        Some(Commands::Reset) => reset_progress(),
        None => interactive_mode(),
    }
}

fn get_progress_file() -> PathBuf {
    let home = home::home_dir().expect("Could not find home directory");
    home.join(".inklings_progress.json")
}

fn load_progress() -> ProgressData {
    let path = get_progress_file();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(data) = serde_json::from_str(&content) {
                return data;
            }
        }
    }
    ProgressData::default()
}

fn save_progress(progress: &ProgressData) {
    let path = get_progress_file();
    if let Ok(json) = serde_json::to_string_pretty(progress) {
        let _ = fs::write(path, json);
    }
}

fn mark_exercise_completed(exercise_id: &str) {
    let mut progress = load_progress();
    progress.completed.insert(exercise_id.to_string());
    progress.last_worked_on = Some(exercise_id.to_string());
    save_progress(&progress);
}

fn increment_attempt(exercise_id: &str) {
    let mut progress = load_progress();
    let count = progress.attempts.entry(exercise_id.to_string()).or_insert(0);
    *count += 1;
    progress.last_worked_on = Some(exercise_id.to_string());
    save_progress(&progress);
}

fn load_exercises() -> Result<ExercisesConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("info/exercises.toml")?;
    let config: ExercisesConfig = toml::from_str(&content)?;
    Ok(config)
}

fn find_exercise_info(exercise_id: &str) -> Option<ExerciseInfo> {
    match load_exercises() {
        Ok(config) => config.exercises.into_iter().find(|e| e.id == exercise_id),
        Err(_) => None,
    }
}

fn verify_exercise(exercise: &str) {
    println!("{}", format!("🔍 Verifying {}...", exercise).cyan());
    
    let exercise_path = PathBuf::from("exercises").join(exercise);
    
    if !exercise_path.exists() {
        println!("{}", format!("Exercise '{}' not found!", exercise).red());
        return;
    }

    increment_attempt(exercise);

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
                        
                        // Mark as completed
                        mark_exercise_completed(exercise);
                        
                        println!("\n{}", "🎉 Exercise completed! Move to the next one.".bright_green().bold());
                        
                        // Show progress stats
                        show_completion_stats(exercise);
                        
                        // Show next exercise suggestion
                        if let Some(info) = find_exercise_info(exercise) {
                            suggest_next_exercise(&info.id);
                        }
                    } else {
                        println!("{}", "✗ Tests failed!".red());
                        println!("{}", String::from_utf8_lossy(&test_output.stderr));
                        println!();
                        println!("{}", "💡 Try running 'inklings hint' for help!".yellow());
                    }
                }
            } else {
                println!("{}", "✗ Compilation failed!".red());
                println!("{}", String::from_utf8_lossy(&output.stderr));
                println!();
                println!("{}", format!("💡 Try running 'inklings hint {}' for help!", exercise).yellow());
            }
        }
        Err(e) => {
            println!("{}", format!("Error running cargo: {}", e).red());
        }
    }
}

fn show_completion_stats(current_exercise: &str) {
    let progress = load_progress();
    
    if let Ok(config) = load_exercises() {
        let total = config.exercises.len();
        let completed = progress.completed.len();
        let percentage = (completed as f32 / total as f32 * 100.0) as u32;
        
        let attempts = progress.attempts.get(current_exercise).unwrap_or(&0);
        
        println!();
        println!("{}", "📊 Your Progress:".cyan().bold());
        println!("   Completed: {}/{} ({}%)", completed, total, percentage);
        println!("   This exercise took {} attempt(s)", attempts);
    }
}

fn suggest_next_exercise(current_id: &str) {
    if let Ok(config) = load_exercises() {
        let progress = load_progress();
        let current_pos = config.exercises.iter().position(|e| e.id == current_id);

        if let Some(pos) = current_pos {
            // Find next uncompleted exercise
            for i in (pos + 1)..config.exercises.len() {
                let next = &config.exercises[i];
                if !progress.completed.contains(&next.id) {
                    println!();
                    println!("{}", "📌 Next exercise:".cyan());
                    println!("   {} - {}", next.id.bright_blue(), next.name);
                    println!();
                    println!("   Run: {}", format!("inklings verify {}", next.id).green());
                    return;
                }
            }
            
            // All exercises completed!
            println!();
            println!("{}", "🏆 Congratulations! You've completed all exercises!".bright_green().bold());
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
    println!("{}", format!("Hint for {}:", exercise).cyan().bold());
    println!();

    let progress = load_progress();
    let attempts = progress.attempts.get(exercise).unwrap_or(&0);
    
    if *attempts > 0 {
        println!("{}", format!("(You've attempted this {} time(s))", attempts).dimmed());
        println!();
    }

    match find_exercise_info(exercise) {
        Some(info) => {
            println!("{}", info.name.bright_white().bold());
            println!();
            println!("{}", info.hint);
        }
        None => {
            println!("{}", format!("Exercise '{}' not found!", exercise).red());
            println!("{}", "Run 'inklings list' to see available exercises.".yellow());
        }
    }
}

fn list_exercises() {
    println!("{}", "Available exercises:".cyan().bold());
    println!();
    
    let progress = load_progress();
    
    match load_exercises() {
        Ok(config) => {
            if !config.categories.is_empty() {
                for (category, exercise_ids) in config.categories {
                    println!("{}", format!("  {}", category).yellow().bold());
                    for id in exercise_ids {
                        if let Some(info) = config.exercises.iter().find(|e| e.id == id) {
                            let status = if progress.completed.contains(&info.id) {
                                "✓".green().bold()
                            } else {
                                "○".dimmed()
                            };
                            println!("    {} {} - {}", status, info.id.bright_blue(), info.name);
                        }
                    }
                    println!();
                }
            } else {
                for exercise in &config.exercises {
                    let status = if progress.completed.contains(&exercise.id) {
                        "✓".green().bold()
                    } else {
                        "○".dimmed()
                    };
                    println!("  {} {} - {}", status, exercise.id.bright_blue(), exercise.name);
                }
            }
            
            let total = config.exercises.len();
            let completed = progress.completed.len();
            let percentage = if total > 0 {
                (completed as f32 / total as f32 * 100.0) as u32
            } else {
                0
            };
            
            println!("{}", format!("Progress: {}/{} ({}%)", completed, total, percentage).cyan());
            println!();
            println!("{}", "Run 'inklings verify <exercise>' to start!".green());
            println!("{}", "Run 'inklings hint <exercise>' for help.".green());
            println!("{}", "Run 'inklings progress' for detailed stats.".green());
        }
        Err(e) => {
            println!("{}", format!("Error loading exercises: {}", e).red());
        }
    }
}

fn show_progress() {
    let progress = load_progress();
    
    println!("{}", "📊 Your Inklings Progress".cyan().bold());
    println!();
    
    match load_exercises() {
        Ok(config) => {
            let total = config.exercises.len();
            let completed = progress.completed.len();
            let percentage = if total > 0 {
                (completed as f32 / total as f32 * 100.0) as u32
            } else {
                0
            };
            
            println!("{}", format!("Overall: {}/{} exercises completed ({}%)", completed, total, percentage).bright_white().bold());
            println!();
            
            // Progress bar
            let bar_width = 50;
            let filled = (bar_width as f32 * completed as f32 / total as f32) as usize;
            let bar: String = "█".repeat(filled) + &"░".repeat(bar_width - filled);
            println!("[{}] {}%", bar.green(), percentage);
            println!();
            
            // Category breakdown
            if !config.categories.is_empty() {
                println!("{}", "By Category:".yellow().bold());
                for (category, exercise_ids) in &config.categories {
                    let category_completed = exercise_ids.iter()
                        .filter(|id| progress.completed.contains(*id))
                        .count();
                    let category_total = exercise_ids.len();
                    let cat_percentage = if category_total > 0 {
                        (category_completed as f32 / category_total as f32 * 100.0) as u32
                    } else {
                        0
                    };
                    
                    println!("  {}: {}/{} ({}%)", 
                        category.bright_blue(), 
                        category_completed, 
                        category_total, 
                        cat_percentage
                    );
                }
                println!();
            }
            
            // Most attempted
            if !progress.attempts.is_empty() {
                println!("{}", "Most Challenging:".yellow().bold());
                let mut attempts_vec: Vec<_> = progress.attempts.iter().collect();
                attempts_vec.sort_by(|a, b| b.1.cmp(a.1));
                
                for (exercise_id, count) in attempts_vec.iter().take(3) {
                    if let Some(info) = config.exercises.iter().find(|e| &e.id == *exercise_id) {
                        println!("  {} - {} attempt(s)", info.name, count);
                    }
                }
                println!();
            }
            
            // Last worked on
            if let Some(last) = &progress.last_worked_on {
                if let Some(info) = config.exercises.iter().find(|e| &e.id == last) {
                    println!("{}", "Last Worked On:".yellow().bold());
                    println!("  {} - {}", info.id.bright_blue(), info.name);
                    
                    if !progress.completed.contains(last) {
                        println!();
                        println!("{}", "Continue with:".green());
                        println!("  {}", format!("inklings verify {}", last).green());
                    }
                }
            }
        }
        Err(e) => {
            println!("{}", format!("Error loading exercises: {}", e).red());
        }
    }
}

fn reset_progress() {
    println!("{}", "⚠️  Are you sure you want to reset all progress? (y/N)".yellow().bold());
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    if input.trim().to_lowercase() == "y" {
        let path = get_progress_file();
        if path.exists() {
            fs::remove_file(path).ok();
        }
        println!("{}", "✓ Progress reset successfully!".green());
    } else {
        println!("{}", "Cancelled.".dimmed());
    }
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

    let progress = load_progress();

    if let Some(last) = &progress.last_worked_on {
        println!("{}", format!("Welcome back! Last worked on: {}", last).yellow());
        println!();
    }

    println!("Commands:");
    println!("  {} - List all exercises with completion status", "inklings list".green());
    println!("  {} - View your progress statistics", "inklings progress".green());
    println!("  {} - Verify your solution", "inklings verify <exercise>".green());
    println!("  {} - Get a hint", "inklings hint <exercise>".green());
    println!("  {} - Watch mode (auto-verify)", "inklings watch <exercise>".green());
}
