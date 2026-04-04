//! Holds a bunch of useful functions used through out the crate in one place.

pub mod validators;

use color_eyre::Section;
use color_eyre::config::HookBuilder;
use color_eyre::eyre::{self, Result, WrapErr};
use std::env;
use std::io::{self, IsTerminal, Read};
use std::path::{Path, PathBuf};

/// Format the input string as PascalCase.
///
/// # Arguments
///
/// * `string` - The target to pascalize.
pub fn pascalize(string: impl AsRef<str>) -> String {
  string
    .as_ref()
    .split(|c: char| !c.is_alphanumeric())
    .filter(|&word| !word.is_empty())
    .map(|word| word[0..1].to_uppercase() + &word[1..])
    .collect()
}

/// Expand any combination of a relative path, tilde, and environment variables into an absolute path.
///
/// # Arguments
///
/// * `path` - The target to expand.
pub fn expand_path(path: impl AsRef<Path>) -> Result<PathBuf> {
  let path = path.as_ref();
  let path_string = path.display().to_string();
  let path_expanded =
    shellexpand::full(&path_string).wrap_err(format!("Failed to expand: {:?}", path))?;

  Ok(PathBuf::from(path_expanded.into_owned()))
}

/// Setup `color_eyre` and `crossterm` depending on flags passed to disable features like _color_, _location section_ and _backtrace section_.
///
/// # Arguments
///
/// * `args` - The Args struct constructed by clap.
pub fn setup_hook(args: &crate::cli::Args) -> Result<()> {
  let mut builder = HookBuilder::new();

  // https://no-color.org
  let no_color = args.no_color
    || env::var("NO_COLOR")
      .ok()
      .map(|v| !v.is_empty())
      .unwrap_or(false);

  if no_color {
    builder = builder.theme(color_eyre::config::Theme::new());
    crossterm::style::force_color_output(false);
  }

  if args.no_location_section {
    builder = builder.display_location_section(false);
  }

  if args.no_backtrace_section {
    builder = builder.display_env_section(false);
  }

  builder
    .install()
    .wrap_err("Failed to install the constructed color_eyre hook")
}

/// Splits the input string at commas, trims whitespace from individual entries, and returns them as a vector of non-empty strings.
///
/// # Arguments
///
/// * `string` - The target to split.
pub fn extract_values(string: impl AsRef<str>) -> Vec<String> {
  let mut result = Vec::new();
  let mut current = String::new();
  let mut escaped = false;

  for c in string.as_ref().chars() {
    match c {
      '\\' if !escaped => escaped = true,
      ',' if !escaped => {
        let trimmed = current.trim();
        if !trimmed.is_empty() {
          result.push(trimmed.to_owned());
        }
        current.clear();
      }
      _ => {
        current.push(c);
        escaped = false;
      }
    }
  }

  let final_trimmed = current.trim();
  if !final_trimmed.is_empty() {
    result.push(final_trimmed.to_owned());
  }

  result
}

/// Gets input either via the `$MINEWORK_ENVIN` environment variable or `stdin`, while extracting useful values using the [`extract_values`] function.
///
/// # Arguments
///
/// * `suggestion` - The message to print in the `Suggestion` section when no input is provided.
pub fn get_non_interactive_input(suggestion: impl Into<String>) -> Result<Vec<String>> {
  let bail_message = "Non-interactive mode expects input either via the $MINEWORK_ENVIN environment variable or stdin with the former getting higher priority.";

  if let Ok(env) = env::var("MINEWORK_ENVIN") {
    let values = extract_values(&env);
    if !values.is_empty() {
      return Ok(values);
    }
  }

  if io::stdin().is_terminal() {
    return Err(eyre::eyre!(bail_message).suggestion(suggestion.into()));
  }

  let mut buffer = String::new();
  io::stdin()
    .read_to_string(&mut buffer)
    .map_err(|e| eyre::eyre!("Failed to read from stdin: {e:?}"))?;

  let values = extract_values(&buffer);
  if !values.is_empty() {
    return Ok(values);
  }

  Err(eyre::eyre!(bail_message).suggestion(suggestion.into()))
}

/// Query Modrinth to get the latest list of all existing Minecraft versions.
pub async fn get_minecraft_versions() -> Result<Vec<String>> {
  Ok(
    ferinth::Ferinth::default()
      .tag_list_game_versions()
      .await?
      .into_iter()
      .map(|v| v.version)
      .collect(),
  )
}

/// Configuration for non-interactive input parsing.
pub struct NonInteractiveInput<'a> {
  /// Number of expected input fields.
  pub fields: usize,
  /// Description of each field for the help message.
  pub descriptions: &'a [&'a str],
  /// Example commands to show in the help message.
  pub examples: &'a [&'a str],
}

impl<'a> NonInteractiveInput<'a> {
  /// Build the complete suggestion message.
  fn build_suggestion_message(&self) -> String {
    let field_list = self
      .descriptions
      .iter()
      .enumerate()
      .map(|(i, desc)| format!("      {}. {}", i + 1, desc))
      .collect::<Vec<_>>()
      .join("\n");

    let examples = self
      .examples
      .iter()
      .map(|ex| format!("      $ {}", ex))
      .collect::<Vec<_>>()
      .join("\n");

    format!(
      "\n   Input is expected to have {} field(s) concatenated using commas with each position mapping to:\n{}\n\n   Here are some examples for your reference:\n{}",
      self.fields, field_list, examples
    )
  }

  /// Parse and validate input count.
  pub fn parse(&self) -> Result<Vec<String>> {
    let suggestion = self.build_suggestion_message();
    let mut lines = get_non_interactive_input(&suggestion)?;

    if lines.len() < self.fields {
      return Err(
        eyre::eyre!(
          "Got {} field(s) while expecting {}.",
          lines.len(),
          self.fields
        )
        .suggestion(suggestion),
      );
    }

    Ok(lines.drain(..self.fields).collect())
  }
}
