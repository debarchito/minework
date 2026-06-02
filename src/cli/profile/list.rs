//! Implementation of the `profile list` subcommand

use crate::config::*;
use color_eyre::eyre::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

/// List all available profiles.
pub fn init(config: Config) -> Result<()> {
  let profiles = &config.profile.list;

  for (index, profile) in profiles.iter().enumerate() {
    let is_active = config.profile.active == Some(index);

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
        Cell::new("Profile #").fg(Color::Blue),
        Cell::new((index + 1).to_string()).fg(Color::Cyan),
      ])
      .add_row(vec![
        Cell::new("Profile name").fg(Color::Blue),
        if is_active {
          Cell::new(format!("{} (active ✓)", profile.name))
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
        Cell::new(profile.r#mod.list.len().to_string()).fg(Color::Magenta),
      ]);

    if let Some(col) = table.column_mut(1) {
      col.set_cell_alignment(CellAlignment::Left);
    }

    println!("{table}");
  }

  Ok(())
}
