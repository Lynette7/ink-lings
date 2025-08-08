use serde::Deserialize;
use std::fs;
use anyhow::{Context, Result};

#[derive(Debug, Clone, Deserialize)]
pub struct Exercise {
    pub name: String,
    pub path: String,
    pub mode: Mode,
    pub hint: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Compile,
    Test,
}

impl Exercise {
    pub fn load_exercises() -> Result<Vec<Exercise>> {
        let info_content = fs::read_to_string("info.toml")
            .context("Failed to read info.toml. Make sure you're in the inklings directory.")?;

        let info: Info = toml::from_str(&info_content)
            .context("Failed to parse info.toml")?;

        Ok(info.exercises)
    }

    pub fn exists(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }

    pub fn get_content(&self) -> Result<String> {
        fs::read_to_string(&self.path)
            .with_context(|| format!("Failed to read exercise file: {}", self.path))
    }
}

#[derive(Deserialize)]
struct Info {
    exercises: Vec<Exercise>,
}
