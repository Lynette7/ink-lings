use crate::exercise::Exercise;
use anyhow::Result;
use colored::*;

pub fn run_exercise(exercise: &Exercise) -> Result<()> {
    if !exercise.exists() {
        return Err(anyhow::anyhow!("Exercise file not found: {}", exercise.path));
    }

    println!("{} Running exercise: {}", "🚀".blue(), exercise.name.cyan().bold());

    let content = exercise.get_content()?;

    println!("{}", "Exercise content:".yellow().bold());
    println!("{}", content);

    println!("\n{}", "Hint:".yellow().bold());
    println!("{}", exercise.hint);

    Ok(())
}
