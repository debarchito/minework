use color_eyre::eyre;
use color_eyre::eyre::{Context, Result};
use console::style;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
  pub(crate) version: String,
  pub(crate) profile: ProfileConfig,
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
pub(crate) struct ProfileConfig {
  pub(crate) active: Option<usize>,
  pub(crate) list: Vec<ProfileOptions>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ProfileOptions {
  pub(crate) name: String,
  pub(crate) game: GameConfig,
  pub(crate) r#mod: ModConfig,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GameConfig {
  pub(crate) version: String,
  pub(crate) directory: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ModConfig {
  pub(crate) loader: String,
  pub(crate) list: Vec<ModOptions>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ModOptions {
  pub(crate) name: String,
  pub(crate) identifier: String,
}

/// Initializes the config file and returns a `Config`.
///
/// If the config file doesn't exist, it will be created with default values.
pub(crate) fn init(config_file: &Path) -> Result<Config> {
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
    .context(format!("Failed to read config from {config_file:?}"))?;
  let config = serde_json::from_str(&content)
    .context("Malformed config. Does it conform to the required JSON schema?")?;

  println!(
    "{} config from {:?}",
    style("Using").green().bold(),
    style(&config_file).cyan()
  );

  Ok(config)
}

/// Writes a serialized `Config` to a JSON file, creating the parent directories if needed.
fn write_config(config: &Config, config_file: &Path) -> Result<()> {
  let parent = config_file
    .parent()
    .ok_or_else(|| eyre::eyre!("Cannot determine parent directory for {config_file:?}"))?;
  fs::create_dir_all(parent).context(format!(
    "Failed to create parent directory(ies) for {config_file:?}"
  ))?;

  let mut writer = fs::File::create(config_file)
    .context(format!("Failed to create config file {config_file:?}"))?;
  let json = serde_json::to_string_pretty(&config)
    .context("Malformed default config. This is a bug, please report it")?;

  writer
    .write_all(json.as_bytes())
    .context(format!("Failed to write default config to {config_file:?}"))?;

  println!(
    "{} default config to {:?}",
    style("Wrote").green().bold(),
    style(&config_file).cyan()
  );

  Ok(())
}
