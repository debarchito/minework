//! Implementation of the `profile info` subcommand.

use crate::config::*;
use color_eyre::eyre;
use color_eyre::eyre::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use crossterm::style::Stylize;
use inquire::Select;

/// Lists info about a profile.
/// If no profile name is provided, tries to fallback to the default profile.
pub fn init(name: Option<&String>, picker: bool, config: Config, args: &crate::Args) -> Result<()> {
  let picker = if picker && args.non_interactive {
    println!(
      "{} Picker is not supported in non-interactive mode. Trying to fallback to the default profile.",
      "[WARNING]".yellow()
    );
    false
  } else {
    picker
  };

  let profile_index = if let Some(profile_name) = name {
    config
      .profile
      .list
      .iter()
      .position(|p| p.name == *profile_name)
      .ok_or_else(|| color_eyre::eyre::eyre!("Profile {:?} not found", profile_name))?
  } else if picker {
    let profile_names: Vec<String> = config.profile.list.iter().map(|p| p.name.clone()).collect();

    if profile_names.is_empty() {
      return Err(color_eyre::eyre::eyre!("No profiles found"));
    }

    let selected_name =
      Select::new("Select a profile to view info about:", profile_names).prompt()?;

    config
      .profile
      .list
      .iter()
      .position(|p| p.name == selected_name)
      .ok_or_else(|| eyre::eyre!("Selected profile should exist"))?
  } else {
    config
      .profile
      .active
      .ok_or_else(|| color_eyre::eyre::eyre!("No active profile set"))?
  };

  let profile = &config.profile.list[profile_index];
  let is_active = config.profile.active == Some(profile_index);

  let mut table = Table::new();
  table
    .load_preset(UTF8_FULL)
    .apply_modifier(UTF8_ROUND_CORNERS)
    .set_content_arrangement(ContentArrangement::Dynamic)
    .set_header(vec![
      Cell::new("Property")
        .fg(Color::Green)
        .add_attribute(Attribute::Bold),
      Cell::new("Value")
        .fg(Color::Green)
        .add_attribute(Attribute::Bold),
    ])
    .add_row(vec![
      Cell::new("Profile name").fg(Color::Blue),
      if is_active {
        Cell::new(format!("{} (active ✓)", &profile.name))
          .fg(Color::Green)
          .add_attribute(Attribute::Italic)
      } else {
        Cell::new(&profile.name)
      },
    ])
    .add_row(vec![
      Cell::new("Minecraft version").fg(Color::Blue),
      Cell::new(&profile.game.version),
    ])
    .add_row(vec![
      Cell::new("Minecraft directory").fg(Color::Blue),
      Cell::new(profile.game.directory.display().to_string()),
    ])
    .add_row(vec![
      Cell::new("Mod loader").fg(Color::Blue),
      Cell::new(&profile.r#mod.loader),
    ])
    .add_row(vec![
      Cell::new("# of mods installed").fg(Color::Blue),
      Cell::new(profile.r#mod.list.len().to_string()).fg(Color::Blue),
    ]);

  if let Some(col) = table.column_mut(1) {
    col.set_cell_alignment(CellAlignment::Left);
  }

  println!("{table}");

  Ok(())
}
