//! Everything related to config goes here.

use color_eyre::eyre::{self, ContextCompat};
use color_eyre::eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents the overall config.
///
/// This includes the config version and all user-defined profiles.
#[derive(Serialize, Deserialize)]
pub struct Config {
  /// The version of the config format.
  pub version: u8,
  /// The profile config containing all user-defined profiles.
  pub profile: ProfileConfig,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      version: 1u8,
      profile: ProfileConfig {
        active: None,
        list: Vec::new(),
      },
    }
  }
}

impl Config {
  /// Serialize the current config and write it to a config file as pretty JSON.
  ///
  /// # Arguments
  ///
  /// * `config_file` - The path to the config file to write to.
  /// * `create_parent` - Create the parent directory if it doesn't exist.
  ///
  /// # Errors
  ///
  /// Returns an [`eyre::Error`] if serialization fails (which indicates a bug, please report it!),
  /// creating the parent directory fails or if writing to the file system fails.
  pub fn write_to(
    &self,
    config_file: impl AsRef<Path> + Copy + Debug,
    create_parent: Option<bool>,
  ) -> Result<()> {
    if let Some(true) = create_parent {
      let parent = config_file
        .as_ref()
        .parent()
        .wrap_err("Cannot determine parent directory for {config_file:?}")?;
      fs::create_dir_all(parent).wrap_err(format!(
        "Failed to create parent directory(ies) for {config_file:?}"
      ))?;
    }

    fs::write(
      config_file,
      serde_json::to_string_pretty(self)
        .wrap_err("Malformed config. This is a bug, please report it")?
        .as_bytes(),
    )
    .wrap_err(format!("Failed to write config to {config_file:?}"))?;

    Ok(())
  }

  /// Deserialize the content of a config file into a [`Config`] instance.
  ///
  /// # Arguments
  ///
  /// * `config_file` — The path to the config file to read from.
  ///
  /// # Errors
  ///
  /// Returns an [`eyre::Error`] if reading from the file system fails,
  /// or if the file contains malformed JSON that does not match the expected schema.
  pub fn read_from(config_file: impl AsRef<Path> + Copy + Debug) -> Result<Self> {
    let content = fs::read_to_string(config_file)
      .wrap_err(format!("Failed to read config from {config_file:?}"))?;
    let config = serde_json::from_str(&content)
      .wrap_err("Malformed config. Does it conform to the required JSON schema?")?;

    Ok(config)
  }

  /// Initialize a config file and return a [`Config`] instance.
  ///
  /// If the specified `config_file` does not exist, it will be created and
  /// populated with the default config.
  ///
  /// # Arguments
  ///
  /// * `config_file` — The path to the config file to initialize.
  ///
  /// # Errors
  ///
  /// Returns an [`eyre::Error`] if reading from/writing to the file system fails,
  /// creating the parent directory fails if the file contains malformed JSON
  /// that does not match the expected schema.
  pub fn init(config_file: impl AsRef<Path> + Copy + Debug) -> Result<Config> {
    let config_file = config_file.as_ref();
    if config_file.exists() && !config_file.is_file() {
      eyre::bail!("{config_file:?} exists but is not a file");
    }
    if !config_file.exists() {
      let config = Config::default();
      config.write_to(config_file, Some(true))?;

      return Ok(config);
    }

    Config::read_from(config_file)
  }
}

/// Config for all user-defined profiles.
///
/// A profile represents a complete set of game details, mod loader, and mod list
/// settings that can be activated and switched between by the user.
#[derive(Serialize, Deserialize)]
pub struct ProfileConfig {
  /// The index of the currently active profile, if any.
  pub active: Option<usize>,
  /// The list of all configured profiles.
  pub list: Vec<ProfileOptions>,
}

/// A single user profile, containing game details and mod configs.
///
/// Each profile represents a standalone environment config:
/// * game version
/// * installation directory
/// * mod loader settings
/// * list of installed or enabled mods
///
/// Profiles allow switching between different Minecraft setups.
#[derive(Serialize, Deserialize)]
pub struct ProfileOptions {
  /// The user-defined name of the profile.
  pub name: String,
  /// Game details associated with this profile.
  pub game: GameDetails,
  /// Mod loader and mod list config for this profile.
  pub r#mod: ModConfig,
}

/// Game-specific config for a profile.
///
/// Defines the target game version and the directory where the game
/// installation for this profile is located.
#[derive(Serialize, Deserialize)]
pub struct GameDetails {
  /// The game version to target.
  pub version: String,
  /// Path to the game directory used by this profile.
  pub directory: PathBuf,
}

/// config for the mod loader and the list of mods applied to a profile.
///
/// This describes which mod loader is in use
/// and which mods are installed or enabled for this profile.
#[derive(Serialize, Deserialize)]
pub struct ModConfig {
  /// The mod loader to use.
  pub loader: String,
  /// The list of mod options belonging to this profile.
  pub list: Vec<ModOptions>,
}

/// A single mod entry within a profile.
///
/// Contains user-friendly identifying information about a mod, such as its
/// display name and its internal identifier (e.g., mod ID or file name).
#[derive(Serialize, Deserialize)]
pub struct ModOptions {
  /// The display name of the mod.
  pub name: String,
  /// Where is the mod from?
  pub from: String,
  /// The unique identifier of the mod.
  pub id: String,
}

impl std::fmt::Display for ModOptions {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{} ({})", self.name, self.id)
  }
}
