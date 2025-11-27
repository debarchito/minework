//! Implementation of the `profile create` subcommand.

use crate::config::*;
use crate::utils::validators::inquire_validators;
use crate::utils::{self, validators};
use color_eyre::eyre::Result;
use crossterm::style::Stylize;
use inquire::{Select, Text};
use std::path::PathBuf;

/// Creates a new profile and saves it to current config.
/// If the current config has no profiles, the new one becomes the active profile.
pub async fn init(name: Option<&String>, mut config: Config, args: &crate::Args) -> Result<()> {
  let profiles: Vec<String> = config
    .profile
    .list
    .iter()
    .map(|p| p.name.to_owned())
    .collect();

  println!("{} Available Minecraft versions.", "[FETCHING]".green());
  let minecraft_versions = utils::get_minecraft_versions().await?;

  let (name, version, directory, loader) = if args.non_interactive {
    let mut inputs = utils::NonInteractiveInput {
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
      &profiles,
      &name,
      Some(false), // The default behaviour is inclusive. Turning it off enables exclusive validation.
      None,        // The default behaviour is to use auto-generated suggestions.
    )?;

    let version = inputs.remove(0);
    validators::against_minecraft_versions(&minecraft_versions, &version)?;

    let directory = utils::expand_path(inputs.remove(0))?;
    validators::is_valid_directory(&directory)?;

    let loader = utils::pascalize(inputs.remove(0));
    validators::against_enum(&super::SUPPORTED_MOD_LOADERS, &loader, None, None)?;

    (name, version, directory, loader)
  } else {
    let name = if let Some(provided_name) = name {
      validators::against_enum(&profiles, provided_name, Some(false), None)?;
      provided_name.clone()
    } else {
      Text::new("What should this profile be called?")
        .with_validators(&[
          inquire_validators::is_non_empty("Profile name"),
          inquire_validators::against_enum(&profiles, Some(false)),
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
        inquire_validators::is_non_empty("Profile name"),
        inquire_validators::is_valid_directory(),
      ])
      .prompt()?;
    let directory = shellexpand::full(&directory).map(|s| PathBuf::from(s.as_ref()))?;

    let loader = Select::new(
      "Which mod loader to use?",
      super::SUPPORTED_MOD_LOADERS.into(),
    )
    .prompt()?
    .to_owned();

    (name, version, directory, loader)
  };

  let options = ProfileOptions {
    name: name.clone(),
    game: GameDetails { version, directory },
    r#mod: ModConfig {
      loader,
      list: Vec::new(),
    },
  };

  config.profile.list.push(options);
  if config.profile.list.len() == 1 {
    config.profile.active = Some(0);
  }

  config.write_to(
    &args.config_file,
    None, // The parent directory already exists.
  )?;
  println!(
    "{} Profile {} created successfully!",
    "[SUCCESS]".green(),
    name.cyan()
  );

  Ok(())
}
