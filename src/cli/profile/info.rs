//! Implementation of the `profile info` subcommand.

use crate::config::*;
use color_eyre::eyre::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub fn init(name: Option<&String>, config: Config, _args: &crate::Args) -> Result<()> {
  let profile_index = if let Some(profile_name) = name {
    config
      .profile
      .list
      .iter()
      .position(|p| p.name == *profile_name)
      .ok_or_else(|| color_eyre::eyre::eyre!("Profile {:?} not found", profile_name))?
  } else {
    config
      .profile
      .active
      .ok_or_else(|| color_eyre::eyre::eyre!("No active profile set"))?
  };
  let profile = &config.profile.list[profile_index];

  let mut table = Table::new();
  table
    .load_preset(UTF8_FULL)
    .apply_modifier(UTF8_ROUND_CORNERS)
    .set_content_arrangement(ContentArrangement::Dynamic)
    .set_header(vec![
      Cell::new("Name")
        .fg(Color::Green)
        .add_attribute(Attribute::Bold),
      Cell::new(&profile.name)
        .fg(Color::Green)
        .add_attribute(Attribute::Bold),
    ])
    .add_row(vec![
      Cell::new("Minecraft Version").fg(Color::Blue),
      Cell::new(&profile.game.version),
    ])
    .add_row(vec![
      Cell::new("Minecraft Directory").fg(Color::Blue),
      Cell::new(profile.game.directory.display().to_string()),
    ])
    .add_row(vec![
      Cell::new("Mod Loader").fg(Color::Blue),
      Cell::new(&profile.r#mod.loader),
    ])
    .add_row(vec![
      Cell::new("Mods Installed").fg(Color::Blue),
      Cell::new(profile.r#mod.list.len().to_string()).fg(Color::Magenta),
    ]);

  if let Some(col) = table.column_mut(1) {
    col.set_cell_alignment(CellAlignment::Left);
  }

  println!("{table}");

  if !profile.r#mod.list.is_empty() {
    let mut mods_table = Table::new();
    mods_table
      .load_preset(UTF8_FULL)
      .apply_modifier(UTF8_ROUND_CORNERS)
      .set_content_arrangement(ContentArrangement::Dynamic)
      .set_header(vec![
        Cell::new("#")
          .fg(Color::Green)
          .add_attribute(Attribute::Bold),
        Cell::new("Mod(s)")
          .fg(Color::Green)
          .add_attribute(Attribute::Bold),
      ]);

    for (i, mod_entry) in profile.r#mod.list.iter().enumerate() {
      mods_table.add_row(vec![
        Cell::new((i + 1).to_string()).fg(Color::DarkGrey),
        Cell::new(mod_entry),
      ]);
    }

    if let Some(col) = mods_table.column_mut(0) {
      col.set_cell_alignment(CellAlignment::Center);
    }

    println!("{mods_table}");
  }

  Ok(())
}
