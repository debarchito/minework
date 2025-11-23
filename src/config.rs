//! Everything releated to the config file.

use color_eyre::eyre;
use color_eyre::eyre::{Result, WrapErr};
use crossterm::ExecutableCommand;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub version: String,
  pub profile: ProfileConfig,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      version: "1".into(),
      profile: ProfileConfig {
        active: None,
        list: Vec::new(),
      },
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct ProfileConfig {
  pub active: Option<usize>,
  pub list: Vec<ProfileOptions>,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileOptions {
  pub name: String,
  pub game: GameConfig,
  pub r#mod: ModConfig,
}

#[derive(Serialize, Deserialize)]
pub struct GameConfig {
  pub version: String,
  pub directory: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct ModConfig {
  pub loader: String,
  pub list: Vec<ModOptions>,
}

#[derive(Serialize, Deserialize)]
pub struct ModOptions {
  pub name: String,
  pub identifier: String,
}

/// Initializes the config file and returns a `Config`.
///
/// If the config file doesn't exist, it will be created with default values.
pub fn init(config_file: &Path) -> Result<Config> {
  if config_file.exists() && !config_file.is_file() {
    eyre::bail!("{config_file:?} exists but is not a file");
  }

  if !config_file.exists() {
    let config = Config::default();
    write_config(&config, config_file)?;
    return Ok(config);
  }

  read_config(config_file)
}

/// Reads a JSON config file from disk and deserializes it into a `Config`.
fn read_config(config_file: &Path) -> Result<Config> {
  let content = fs::read_to_string(config_file)
    .wrap_err(format!("Failed to read config from {config_file:?}"))?;
  let config = serde_json::from_str(&content)
    .wrap_err("Malformed config. Does it conform to the required JSON schema?")?;

  io::stdout()
    .execute(SetForegroundColor(Color::Green))?
    .execute(Print("Using"))?
    .execute(ResetColor)?
    .execute(Print(" config from "))?
    .execute(SetForegroundColor(Color::Cyan))?
    .execute(Print(format!("{:?}", &config_file)))?
    .execute(ResetColor)?
    .execute(Print("\n"))?;

  Ok(config)
}

/// Writes a serialized `Config` to a JSON file, creating the parent directories if needed.
fn write_config(config: &Config, config_file: &Path) -> Result<()> {
  let parent = config_file
    .parent()
    .ok_or_else(|| eyre::eyre!("Cannot determine parent directory for {config_file:?}"))?;
  fs::create_dir_all(parent).wrap_err(format!(
    "Failed to create parent directory(ies) for {config_file:?}"
  ))?;

  let mut writer = fs::File::create(config_file)
    .wrap_err(format!("Failed to create config file {config_file:?}"))?;
  let json = serde_json::to_string_pretty(&config)
    .wrap_err("Malformed default config. This is a bug, please report it")?;

  writer
    .write_all(json.as_bytes())
    .wrap_err(format!("Failed to write default config to {config_file:?}"))?;

  io::stdout()
    .execute(SetForegroundColor(Color::Green))?
    .execute(Print("Wrote"))?
    .execute(ResetColor)?
    .execute(Print(" default config to "))?
    .execute(SetForegroundColor(Color::Cyan))?
    .execute(Print(format!("{:?}", &config_file)))?
    .execute(ResetColor)?
    .execute(Print("\n"))?;

  Ok(())
}
