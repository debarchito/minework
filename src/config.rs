use console::style;
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
  version: String,
  profile: ProfileConfig,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ProfileConfig {
  active: Option<usize>,
  list: Vec<ProfileOptions>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ProfileOptions {
  name: String,
  game: GameConfig,
  r#mod: ModConfig,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GameConfig {
  version: String,
  directory: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ModConfig {
  loader: String,
  list: Vec<ModOptions>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ModOptions {
  name: String,
  identifier: String,
}

pub(crate) fn init(config_file: &str) -> Result<Config> {
  let expanded = tilde(config_file);
  let path = PathBuf::from(expanded.into_owned());
  println!(
    "   {} to use configuration from {:?}",
    style("Trying").green().bold(),
    style(&path).cyan()
  );

  if path.exists() && !path.is_file() {
    eyre::bail!("{path:?} exists but is not a file");
  }

  if !path.exists() {
    let parent = path
      .parent()
      .ok_or_else(|| eyre::eyre!("Cannot determine parent directory for {path:?}"))?;
    fs::create_dir_all(parent)
      .context(format!("Failed to create parent directory for {path:?}. Do you have the permission to create this directory?"))?;

    let mut writer =
      fs::File::create(&path).context(format!("Failed to create configuration file {path:?}. Do you have the permission to create this file?"))?;
    let config = Config {
      version: "1".into(),
      profile: ProfileConfig {
        active: None,
        list: Vec::new(),
      },
    };

    let content = serde_json::to_string_pretty(&config).context(
      "Failed to serialize default configuration to JSON. This is a bug, please report it",
    )?;
    writer
      .write_all(content.as_bytes())
      .context(format!(
        "Failed to write default configuration to {path:?}. Do you have the permission to write to this file?"
      ))?;
    println!(
      "   {} default configuration to {:?}",
      style("Wrote").green().bold(),
      style(&path).cyan()
    );
  }

  let content = fs::read_to_string(&path).context(format!(
    "Failed to read configuration from {path:?}. Do you have the permission to read this file?"
  ))?;
  let content_json: Config = serde_json::from_str(&content)
    .context("Malformed configuration. Does it conform to the JSON schema?")?;

  Ok(content_json)
}
