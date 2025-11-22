//! Holds a bunch of useful functions used through out the crate in one place

use color_eyre::Section;
use color_eyre::config::HookBuilder;
use color_eyre::eyre::{self, Result, WrapErr};
use std::io::{self, IsTerminal, Read};
use std::path::{Path, PathBuf};

/// Expand any combination of a relative path, tilde, and environment variables into an absolute path
pub(crate) fn expand_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
  let path = path.as_ref();
  let path_string = path.display().to_string();
  let path_expanded =
    shellexpand::full(&path_string).context(format!("Failed to expand: {:?}", path))?;

  Ok(PathBuf::from(path_expanded.into_owned()))
}

/// Install an adjustable color_eyre hook
pub(crate) fn install_color_eyre(args: &super::Args) -> Result<()> {
  let mut builder = HookBuilder::new();

  if args.no_color {
    builder = builder.theme(color_eyre::config::Theme::new());
  }

  if args.no_location_section {
    builder = builder.display_location_section(false);
  }

  if args.no_backtrace_section {
    builder = builder.display_env_section(false);
  }

  builder
    .install()
    .context("Failed to install the color_eyre hook")
}

/// Splits the input string by <>, trims whitespace, and returns a `Vec<String>` of non-empty values
pub(crate) fn extract_values(input: &str) -> Vec<String> {
  input
    .split("<>")
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect()
}

/// Gets input either from the $MINEWORK_ENVIN environment variable or stdin, and returns it as a vector of strings
pub(crate) fn get_non_interactive_input(bail_message: &str) -> Result<Vec<String>> {
  let bail_message = format!(
    "Non-interactive mode requires input either via $MINEWORK_ENVIN environment variable or stdin with the former getting higher priority.\n\n\
    {bail_message}"
  );

  if let Ok(env) = std::env::var("MINEWORK_ENVIN") {
    let values = extract_values(&env);
    if !values.is_empty() {
      return Ok(values);
    }
  }

  if io::stdin().is_terminal() {
    eyre::bail!(bail_message);
  }

  let mut buffer = String::new();
  io::stdin()
    .read_to_string(&mut buffer)
    .map_err(|e| eyre::eyre!("Failed to read from stdin: {e:?}"))?;

  let values = extract_values(&buffer);
  if !values.is_empty() {
    return Ok(values);
  }

  eyre::bail!(bail_message);
}

pub(crate) async fn get_minecraft_versions() -> Result<Vec<String>> {
  Ok(
    ferinth::Ferinth::default()
      .tag_list_game_versions()
      .await?
      .into_iter()
      .map(|v| v.version)
      .collect(),
  )
}

/// Configuration for non-interactive input parsing
pub(crate) struct NonInteractiveInput<'a> {
  /// Number of expected input fields
  pub(crate) fields: usize,
  /// Description of each field for the help message
  pub(crate) descriptions: &'a [&'a str],
  /// Example commands to show in the help message
  pub(crate) examples: &'a [&'a str],
}

impl<'a> NonInteractiveInput<'a> {
  /// Generate the complete bail message
  fn bail_message(&self) -> String {
    let field_list = self
      .descriptions
      .iter()
      .enumerate()
      .map(|(i, desc)| format!("  {}. {}", i + 1, desc))
      .collect::<Vec<_>>()
      .join("\n");

    let examples = self
      .examples
      .iter()
      .map(|ex| format!("  $ {}", ex))
      .collect::<Vec<_>>()
      .join("\n");

    format!(
      "Input is expected to have {} string(s) concatenated using <> operator, with each position mapping to:\n{}\n\nSome examples for reference:\n{}",
      self.fields, field_list, examples
    )
  }

  /// Parse and validate input count
  pub(crate) fn parse(&self) -> Result<Vec<String>> {
    let bail_message = self.bail_message();
    let mut lines = crate::utils::get_non_interactive_input(&bail_message)?;

    if lines.len() < self.fields {
      return Err(
        eyre::eyre!("Got {} argument(s), expected {}.", lines.len(), self.fields)
          .suggestion(format!("\n{bail_message}")),
      );
    }

    Ok(lines.drain(..self.fields).collect())
  }
}

pub(crate) fn validate_directory<P: AsRef<Path>>(path: P) -> Result<()> {
  let path = path.as_ref();

  if !path.exists() {
    eyre::bail!("Directory {:?} does not exist", path);
  }

  if !path.is_dir() {
    eyre::bail!("While {:?} exists, it is not a directory", path);
  }

  Ok(())
}

/// Validates a target string against a list of allowed variants.
///
/// # Arguments
///
/// * `variants` - An iterable collection of string-like values representing valid options
/// * `target` - The string to validate
/// * `inclusive` - Controls validation behavior:
///   - `None` or `Some(true)`: Target MUST exist in variants (inclusive check). This is the default behaviour.
///   - `Some(false)`: Target MUST NOT exist in variants (exclusive check, for uniqueness)
/// ```
pub(crate) fn validate_against_enum<S>(
  variants: &[S],
  target: &str,
  inclusive: Option<bool>,
) -> Result<()>
where
  S: AsRef<str>,
{
  let exists = variants.iter().any(|v| v.as_ref() == target);

  match inclusive {
    Some(false) => {
      if exists {
        return Err(eyre::eyre!("{target:?} is a duplicate entry"));
      }
    }
    _ => {
      if !exists {
        let options = variants
          .iter()
          .map(|v| v.as_ref())
          .collect::<Vec<_>>()
          .join(", ");

        return Err(
          eyre::eyre!("Invalid value {target:?}")
            .suggestion(format!("\nSupported versions include {options}")),
        );
      }
    }
  }

  Ok(())
}
