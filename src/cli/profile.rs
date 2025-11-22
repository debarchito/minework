//! Implements all the profile subcommands

mod utils;

use crate::config::*;
use crate::utils::*;
use color_eyre::eyre::Result;

const SUPPORTED_MOD_LOADERS: [&'static str; 2] = ["none", "fabric"];

/// Creates a new profile and saves it to the config file.
///
/// The completed profile is appended to the config. If the config has no
/// profiles yet, the new one becomes the active profile.
pub(crate) async fn create(
  name: &Option<String>,
  mut config: Config,
  args: &crate::Args,
) -> Result<()> {
  let (name, version, directory, loader) = if args.non_interactive {
    let mut inputs = NonInteractiveInput {
      fields: 4,
      descriptions: &[
        "A unique profile name",
        "The Minecraft version to target",
        "Path to directory where Minecraft instance is installed",
        "The mod loader to use (none, fabric)",
      ],
      examples: &[
        "MINEWORK_ENVIN=\"MineWorld <> 1.21.5 <> ~/.minecraft <> fabric\" minework --non-interactive profile create",
        "echo \"MineWorld <> 1.21.5 <> $XDG_DATA_HOME/PrismLauncher/instances/MineWorld/minecraft <> Fabric\" | minework -I pr c",
      ],
    }.parse()?;

    let name = inputs.remove(0);
    let profile_names: Vec<&str> = config
      .profile
      .list
      .iter()
      .map(|p| p.name.as_str())
      .collect();
    validate_against_enum(&profile_names, &name, Some(false))?;

    let version = inputs.remove(0);
    let minecraft_versions = get_minecraft_versions().await?;
    validate_against_enum(&minecraft_versions, &version, None)?;

    let directory = expand_path(inputs.remove(0))?;
    validate_directory(&directory)?;

    let loader = inputs.remove(0).to_lowercase();
    validate_against_enum(&SUPPORTED_MOD_LOADERS, &loader, None)?;

    (name, version, directory, loader)
  } else {
    let name = utils::prompt_profile_name(name, &config)?;
    let version = utils::prompt_minecraft_version().await?;
    let directory = utils::prompt_game_directory()?;
    let loader = utils::prompt_mod_loader()?;

    (name, version, directory, loader)
  };

  let options = ProfileOptions {
    name: name.clone(),
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

  utils::write_config(&args.config_file, &config)?;
  println!("✓ Profile '{}' created successfully", name);
  Ok(())
}
