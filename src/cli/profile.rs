//! Implements all the profile subcommands.

use crate::config::*;
use crate::utils::{validators, *};
use color_eyre::eyre::Result;
use inquire::{Select, Text};
use std::path::PathBuf;

const SUPPORTED_MOD_LOADERS: [&str; 2] = ["none", "fabric"];

/// Creates a new profile and saves it to the config file.
///
/// The completed profile is appended to the config. If the config has no
/// profiles yet, the new one becomes the active profile.
pub async fn create(name: &Option<String>, mut config: Config, args: &crate::Args) -> Result<()> {
  let profile_names: Vec<String> = config
    .profile
    .list
    .iter()
    .map(|p| p.name.to_owned())
    .collect();
  let minecraft_versions = get_minecraft_versions().await?;
  let (name, version, directory, loader) = if args.non_interactive {
    let mut inputs = NonInteractiveInput {
      fields: 4,
      descriptions: &[
        "A unique profile name.",
        "The Minecraft version to target.",
        "Path to directory where Minecraft instance is installed.",
        "The mod loader to use (none, fabric).",
      ],
      examples: &[
        "MINEWORK_ENVIN=\"MineWorld <> 1.21.5 <> ~/.minecraft <> fabric\" minework --non-interactive profile create",
        "echo \"MineWorld <> 1.21.5 <> $XDG_DATA_HOME/PrismLauncher/instances/MineWorld/minecraft <> Fabric\" | minework -I pr c",
      ],
    }.parse()?;

    let name = inputs.remove(0);
    validators::against_enum(
      &profile_names,
      &name,
      /* inclusive (default behaviour) */ Some(false),
      /* custom suggestion (auto-lists variants otherwise) */ None,
    )?;

    let version = inputs.remove(0);
    validators::against_minecraft_versions(&minecraft_versions, &version)?;

    let directory = expand_path(inputs.remove(0))?;
    validators::is_valid_directory(&directory)?;

    let loader = inputs.remove(0).to_lowercase();
    validators::against_enum(&SUPPORTED_MOD_LOADERS, &loader, None, None)?;

    (name, version, directory, loader)
  } else {
    let name = if let Some(provided_name) = name {
      validators::against_enum(&profile_names, provided_name, Some(false), None)?;
      provided_name.clone()
    } else {
      Text::new("What should this profile be called?")
        .with_validators(&[
          validators::inquire::is_non_empty("Profile name"),
          validators::inquire::against_enum(&profile_names, Some(false)),
        ])
        .prompt()?
    };
    let version = Select::new(
      "Which version of Minecraft should this profile target?",
      minecraft_versions,
    )
    .prompt()?;
    let directory = Text::new("Enter the location of the game:")
      .with_validators(&[
        validators::inquire::is_non_empty("Profile name"),
        validators::inquire::is_valid_directory(),
      ])
      .prompt()?;
    let directory = shellexpand::full(&directory).map(|s| PathBuf::from(s.as_ref()))?;
    let loader = Select::new("Which mod loader to use?", SUPPORTED_MOD_LOADERS.into())
      .prompt()?
      .to_owned();

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

  write_config(&args.config_file, &config)?;
  println!("✓ Profile '{}' created successfully", name);
  Ok(())
}
