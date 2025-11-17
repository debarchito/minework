use crate::config::*;
use color_eyre::eyre;
use color_eyre::eyre::{Context, Result};
use ferinth::Ferinth;
use inquire::validator::{ErrorMessage, Validation};
use inquire::{Select, Text};
use std::path::PathBuf;

/// Creates a new profile and saves it to the config file.
///
/// If `name` is not provided, the function interactively asks the user for:
///   - profile name (validated to be unique)
///   - Minecraft version (fetched from Modrinth)
///   - game directory (validated to exist and be a directory)
///   - mod loader (vanilla is supported through the "none" option)
///
/// The completed profile is appended to the config. If the config has no
/// profiles yet, the new one becomes the active profile.
pub(crate) async fn create(
  name: Option<String>,
  mut config: Config,
  config_file: PathBuf,
) -> Result<()> {
  let name = prompt_profile_name(name, &config)?;
  let version = prompt_minecraft_version().await?;
  let directory = prompt_game_directory()?;
  let loader = prompt_mod_loader()?;

  let options = ProfileOptions {
    name,
    game: GameConfig { version, directory },
    r#mod: ModConfig {
      loader,
      list: Vec::new(),
    },
  };

  update_config(&mut config, options);
  write_config(&config_file, &config)?;

  Ok(())
}

/// Get the profile name.  
///
/// If a name was passed in, it is returned. If not, the user is prompted,
/// and the input is validated to ensure the config does not already
/// contain a profile with the same case-insensitive name.
fn prompt_profile_name(name: Option<String>, config: &Config) -> Result<String> {
  if let Some(n) = name {
    if config
      .profile
      .list
      .iter()
      .any(|p| p.name.eq_ignore_ascii_case(&n))
    {
      eyre::bail!("A profile with the name \"{n}\" already exists");
    }
    return Ok(n);
  }

  Text::new("What should this profile be called?")
    .with_validator(|s: &str| {
      if s.trim().is_empty() {
        Ok(Validation::Invalid(ErrorMessage::Custom(
          "Profile name cannot be empty".into(),
        )))
      } else if config
        .profile
        .list
        .iter()
        .any(|p| p.name.eq_ignore_ascii_case(s))
      {
        Ok(Validation::Invalid(ErrorMessage::Custom(format!(
          "A profile with the name \"{s}\" already exists"
        ))))
      } else {
        Ok(Validation::Valid)
      }
    })
    .prompt()
    .map_err(Into::into)
}

/// Fetch the list of Minecraft versions from Modrinth and prompt the user
/// to pick one.
async fn prompt_minecraft_version() -> Result<String> {
  let versions: Vec<String> = Ferinth::default()
    .tag_list_game_versions()
    .await?
    .into_iter()
    .map(|v| v.version)
    .collect();

  Select::new(
    "Which version of Minecraft should this profile target?",
    versions,
  )
  .prompt()
  .map_err(Into::into)
}

/// Prompt for the game directory and ensure it is valid.
///
/// The user may enter `~` or other shell-expanded paths.  
/// The validator checks:
///   - the path exists
///   - it is a directory  
fn prompt_game_directory() -> Result<PathBuf> {
  let dir = Text::new("Enter the location of the game:")
    .with_validator(|s: &str| match shellexpand::full(s) {
      Ok(expanded) => {
        let path = PathBuf::from(expanded.as_ref());
        if !path.exists() {
          Ok(Validation::Invalid(ErrorMessage::Custom(
            "Path does not exist".to_owned(),
          )))
        } else if !path.is_dir() {
          Ok(Validation::Invalid(ErrorMessage::Custom(
            "Path is not a directory".to_owned(),
          )))
        } else {
          Ok(Validation::Valid)
        }
      }
      Err(e) => Ok(Validation::Invalid(ErrorMessage::Custom(format!(
        "Invalid path: {}",
        e
      )))),
    })
    .prompt()?;

  Ok(
    shellexpand::full(&dir)
      .map(|s| PathBuf::from(s.as_ref()))
      .unwrap_or_else(|_| PathBuf::from(dir)),
  )
}

/// Prompt the user to choose a mod loader.  
///
/// `none` can be chosen to indicate vanilla.
fn prompt_mod_loader() -> Result<String> {
  Select::new("Which mod loader to use?", vec!["none", "fabric"])
    .prompt()
    .map(|s| s.to_string())
    .map_err(Into::into)
}

/// Insert the new profile into the config.
///
/// If this is the first profile being added, mark it as active.
fn update_config(config: &mut Config, options: ProfileOptions) {
  config.profile.list.push(options);
  if config.profile.list.len() == 1 {
    config.profile.active = Some(0);
  }
}

/// Write the updated configuration to the config file in pretty JSON format.
fn write_config(config_file: &PathBuf, config: &Config) -> Result<()> {
  std::fs::write(
    config_file,
    serde_json::to_string_pretty(config)
      .context("Malformed config. This is a bug, please report it")?
      .as_bytes(),
  )
  .context(format!("Failed to write default config to {config_file:?}"))?;

  Ok(())
}
