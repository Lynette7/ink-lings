use crate::exercise::{Exercise, Mode};
use anyhow::{Context, Result};
use std::process::Command;
use colored::*;

pub fn verify_exercise(exercise: &Exercise) -> Result<()> {
    if !exercise.exists() {
        return Err(anyhow::anyhow!("Exercise file not found: {}", exercise.path));
    }

    println!("{} Verifying exercise: {}", "🔍".blue(), exercise.name.cyan().bold());

    match exercise.mode {
        Mode::Compile => compile_exercise(exercise),
        Mode::Test => test_exercise(exercise),
    }
}

fn compile_exercise(exercise: &Exercise) -> Result<()> {
    println!("{} Compiling...", "⚙️".yellow());

    // Create a temporary Cargo.toml file for the exercise
    let exercise_dir = std::path::Path::new(&exercise.path)
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));

    let output = Command::new("cargo")
        .args(&["contract", "build", "--manifest-path"])
        .arg(format!("{}/Cargo.toml", exercise_dir.display()))
        .output()
        .context("Failed to run cargo contract build")?;

    if output.status.success() {
        println!("{} Compilation successful!", "✓".green().bold());
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        println!("{} Compilation failed:", "✗".red().bold());
        if !stderr.is_empty() {
            println!("{}", stderr);
        }
        if !stdout.is_empty() {
            println!("{}", stdout);
        }

        Err(anyhow::anyhow!("Compilation failed"))
    }
}

fn test_exercise(exercise: &Exercise) -> Result<()> {
    println!("{} Running tests...", "🧪".yellow());

    let exercise_dir = std::path::Path::new(&exercise.path)
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));

    let output = Command::new("cargo")
        .args(&["contract", "test", "--manifest-path"])
        .arg(format!("{}/Cargo.toml", exercise_dir.display()))
        .output()
        .context("Failed to run cargo contract test")?;

    if output.status.success() {
        println!("{} All tests passed!", "✓".green().bold());
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        println!("{} Tests failed:", "✗".red().bold());
        if !stderr.is_empty() {
            println!("{}", stderr);
        }
        if !stdout.is_empty() {
            println!("{}", stdout);
        }

        Err(anyhow::anyhow!("Tests failed"))
    }
}
