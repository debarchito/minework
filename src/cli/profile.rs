mod utils;
use crate::config::*;
use color_eyre::eyre::Result;
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
  let name = utils::prompt_profile_name(name, &config)?;
  let version = utils::prompt_minecraft_version().await?;
  let directory = utils::prompt_game_directory()?;
  let loader = utils::prompt_mod_loader()?;

  let options = ProfileOptions {
    name,
    game: GameConfig { version, directory },
    r#mod: ModConfig {
      loader,
      list: Vec::new(),
    },
  };

  config.profile.list.push(options);
  if config.profile.list.len() == 1 {
    config.profile.active = Some(0);
  }

  utils::write_config(&config_file, &config)?;

  Ok(())
}
