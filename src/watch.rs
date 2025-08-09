use crate::exercise::Exercise;
use crate::verify;
use anyhow::Result;
use colored::*;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{path::Path, sync::mpsc, time::Duration};

pub fn watch_exercises(exercises: Vec<Exercise>) -> Result<()> {
    println!("{} Starting watch mode...", "👀".blue());
    println!("Watching for changes in exercise files. Press Ctrl+C to exit.");

    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Watch the exercises directory
    watcher.watch(Path::new("exercises"), RecursiveMode::Recursive)?;

    // Keep track of the current exercise
    let mut current_exercise_index = 0;

    // Initial run
    if !exercises.is_empty() {
        run_current_exercise(&exercises[current_exercise_index]);
    }

    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(event)) => {
                // Check if any exercise file was modified
                if let Some(paths) = event.paths.get(0) {
                    if let Some(exercise) = exercises.iter().find(|e| {
                        Path::new(&e.path) == paths
                    }) {
                        println!("\n{} File changed: {}", "📝".yellow(), exercise.path);
                        run_current_exercise(exercise);

                        // update current exercise index
                        if let Some(index) = exercises.iter().position(|e| e.name == exercise.name) {
                            current_exercise_index = index;
                        }
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watch error: {:?}", e);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                println!("Watch timeout");
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }

    Ok(())
}

fn run_current_exercise(exercise: &Exercise) {
    println!("\n{}", "=".repeat(50).blue());
    println!("{} Current exercise: {}", "📚".blue(), exercise.name.cyan().bold());

    match verify::verify_exercise(exercise) {
        Ok(_) => {
            println!("{} Great job! Exercise completed successfully!", "🎉".green().bold());
            println!("Move on to the next exercise or modify this one to explore more.");
        }
        Err(_) => {
            println!("\n{} Need help? Run: {}", "💡".yellow(), format!("inklings hint{}", exercise.name).cyan());
        }
    }

    println!("{}", "=".repeat(50).blue());
}
