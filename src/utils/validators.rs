//! Bunch of helpful validation functions.

pub mod inquire_validators;

use color_eyre::Section;
use color_eyre::eyre::{self, Result};
use crossterm::style::Stylize;
use std::path::Path;

/// Validate the existence of the provided path and it being a directory.
///
/// # Arguments
///
/// * `path` - The target to validate.
pub fn is_valid_directory(path: impl AsRef<Path>) -> Result<()> {
  let path = path.as_ref();

  if !path.exists() {
    eyre::bail!("Directory {:?} does not exist", path);
  }

  if !path.is_dir() {
    eyre::bail!("While {:?} exists, it is not a directory", path);
  }

  Ok(())
}

/// Validates the provided string against a list of enumerations.
///
/// # Arguments
///
/// * `variants` - An iterable collection of string-like values representing valid options.
/// * `string` - The string to validate.
/// * `inclusive` - Controls validation behavior:
///   - `None` or `Some(true)`: Target MUST exist in variants (inclusive check). This is the default behavior.
///   - `Some(false)`: Target MUST NOT exist in variants (exclusive check, for uniqueness).
/// * `suggestion` - Custom content for the suggestion section. If None, lists all variants.
pub fn against_enum(
  variants: &[impl AsRef<str>],
  string: &str,
  inclusive: Option<bool>,
  suggestion: Option<String>,
) -> Result<()> {
  let exists = variants.iter().any(|v| v.as_ref() == string);

  match (inclusive, exists) {
    (Some(false), true) => Err(eyre::eyre!("{string:?} is a duplicate entry")),
    (None | Some(true), false) => {
      let base_error = eyre::eyre!("Invalid entry {string:?}");

      if let Some(context) = suggestion {
        Err(base_error.suggestion(context))
      } else {
        use crossterm::style::Stylize;
        let options = variants
          .iter()
          .map(|v| format!("     • {}", v.as_ref().green()))
          .collect::<Vec<_>>()
          .join("\n");

        Err(base_error.suggestion(format!("\n   Valid options include:\n{options}")))
      }
    }
    _ => Ok(()),
  }
}

/// Validates a Minecraft version against available versions.
///
/// Provides specific context about Minecraft versions including a helpful
/// command to search all available versions.
///
/// # Arguments
///
/// * `minecraft_versions` - Slice of valid Minecraft version strings.
/// * `version` - The version string to validate.
pub fn against_minecraft_versions(
  minecraft_versions: &[impl AsRef<str>],
  version: &str,
) -> Result<()> {
  let options = minecraft_versions
    .iter()
    .take(10)
    .map(|v| format!("     • {}", v.as_ref().green()))
    .collect::<Vec<_>>()
    .join("\n");

  let total = minecraft_versions.len();
  let query = format!(
    "{} {} {} {} {}",
    "curl".cyan(),
    "-s https://api.modrinth.com/v2/tag/game_version |".yellow(),
    "jq".cyan(),
    "-r '.[].version' |".yellow(),
    "fzf".cyan(),
  );

  let more_msg = if total > 10 {
    format!(
      "\n     ... and {} more.\n\n   Search all versions using:\n     {}",
      total - 10,
      query
    )
  } else {
    format!("\n\n   Search all versions using:\n     {}", query)
  };

  let context = format!("\n   Supported Minecraft versions include:\n{options}{more_msg}");

  against_enum(minecraft_versions, version, None, Some(context))
}
