//! Implementation of the `profile delete` subcommand.

use crate::config::*;
use crate::utils;
use color_eyre::eyre::Result;
use crossterm::style::Stylize;
use inquire::{Confirm, MultiSelect};

/// Delete a profile.
pub fn init(name: Option<&String>, config: Config, args: &crate::Args) -> Result<()> {
  if config.profile.list.is_empty() {
    return Err(color_eyre::eyre::eyre!("No profiles found to delete"));
  }

  let profile_names = if args.non_interactive {
    let mut inputs = utils::NonInteractiveInput {
      fields: 1,
      descriptions: &[
        "The profile name(s) to delete, comma-separated (use \"(all)\" to delete all profiles).",
      ],
      examples: &[
        "MINEWORK_ENVIN=\"MineWorld,TestWorld\" minework --non-interactive profile delete",
        "echo \"(all)\" | minework -I pr d",
      ],
    }
    .parse()?;
    let input = inputs.remove(0);
    if input == "(all)" {
      vec!["(all)".to_string()]
    } else {
      input.split(',').map(|s| s.trim().to_string()).collect()
    }
  } else if let Some(provided_name) = name {
    vec![provided_name.clone()]
  } else {
    let mut options: Vec<String> = config.profile.list.iter().map(|p| p.name.clone()).collect();
    options.insert(0, "(all)".to_string());

    let selected = MultiSelect::new("Select profile(s) to delete:", options).prompt()?;

    if selected.contains(&"(all)".to_string()) {
      vec!["(all)".to_string()]
    } else {
      selected
    }
  };

  if profile_names.len() == 1 && profile_names[0] == "(all)" {
    return delete_all_profiles(config, args);
  }

  delete_profiles(config, &profile_names, args)
}

fn delete_profiles(mut config: Config, profile_names: &[String], args: &crate::Args) -> Result<()> {
  let mut indices_to_delete: Vec<usize> = Vec::new();

  for name in profile_names {
    let index = config
      .profile
      .list
      .iter()
      .position(|p| &p.name == name)
      .ok_or_else(|| color_eyre::eyre::eyre!("Profile {:?} not found", name))?;
    indices_to_delete.push(index);
  }

  indices_to_delete.sort_unstable();
  indices_to_delete.dedup();

  let confirmation_message = if indices_to_delete.len() == 1 {
    let profile = &config.profile.list[indices_to_delete[0]];
    let is_active = config.profile.active == Some(indices_to_delete[0]);
    if is_active {
      format!(
        "Are you sure you want to delete the active profile {}? This action cannot be undone.",
        &profile.name.clone().cyan()
      )
    } else {
      format!(
        "Are you sure you want to delete profile {}? This action cannot be undone.",
        &profile.name.clone().cyan()
      )
    }
  } else {
    format!(
      "Are you sure you want to delete {} profile(s)? This action cannot be undone.",
      indices_to_delete.len()
    )
  };

  let confirmed = if args.non_interactive {
    println!(
      "{} Non-interactive mode: skipping confirmation for profile deletion.",
      "[WARNING]".yellow()
    );
    true
  } else {
    Confirm::new(&confirmation_message)
      .with_default(false)
      .prompt()?
  };

  if !confirmed {
    println!("{} Profile deletion cancelled.", "[INFO]".cyan());
    return Ok(());
  }

  for &index in indices_to_delete.iter().rev() {
    let profile_name = &config.profile.list[index].name.clone();
    let is_active = config.profile.active == Some(index);

    config.profile.list.remove(index);

    if is_active {
      config.profile.active = None;
      println!(
        "{} Deleted active profile {}. No active profile is now set.",
        "[SUCCESS]".green(),
        profile_name.clone().cyan()
      );
    } else {
      println!(
        "{} Profile {} deleted successfully.",
        "[SUCCESS]".green(),
        profile_name.clone().cyan()
      );
    }

    if let Some(active_idx) = config.profile.active {
      if active_idx > index {
        config.profile.active = Some(active_idx - 1);
      }
    }
  }

  config.write_to(&args.config_file, None)?;

  Ok(())
}

fn delete_all_profiles(mut config: Config, args: &crate::Args) -> Result<()> {
  let profile_count = config.profile.list.len();

  let confirmation_message = format!(
    "Are you sure you want to delete ALL {} profile(s)? This action cannot be undone.",
    profile_count
  );

  let confirmed = if args.non_interactive {
    println!(
      "{} Non-interactive mode: skipping confirmation for deleting all profiles.",
      "[WARNING]".yellow()
    );
    true
  } else {
    Confirm::new(&confirmation_message)
      .with_default(false)
      .prompt()?
  };

  if !confirmed {
    println!("{} Profile deletion cancelled.", "[INFO]".cyan());
    return Ok(());
  }

  config.profile.list.clear();
  config.profile.active = None;

  println!(
    "{} Deleted all {} profile(s) successfully.",
    "[SUCCESS]".green(),
    profile_count
  );

  config.write_to(&args.config_file, None)?;

  Ok(())
}
