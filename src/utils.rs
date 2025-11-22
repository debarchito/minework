//! Holds a bunch of useful functions used through out the crate in one place.

use color_eyre::Section;
use color_eyre::config::HookBuilder;
use color_eyre::eyre::{self, Result, WrapErr};
use std::env;
use std::io::{self, IsTerminal, Read};
use std::path::{Path, PathBuf};

/// Expand any combination of a relative path, tilde, and environment variables into an absolute path.
///
/// # Arguments
///
/// * `path` - The target to expand.
pub(crate) fn expand_path(path: impl AsRef<Path>) -> Result<PathBuf> {
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
pub(crate) fn setup_hook(args: &crate::cli::Args) -> Result<()> {
  let mut builder = HookBuilder::new();

  // https://no-color.org
  let no_color = args.no_color
    || env::var("NO_COLOR")
      .ok()
      .map(|v| !v.is_empty() && (v == "1" || v.eq_ignore_ascii_case("true")))
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

/// Splits the input string at every `<>`, trims whitespace from individual entries, and returns them as a vector of non-empty strings.
///
/// # Arguments
///
/// * `string` - The target to split.
pub(crate) fn extract_values(string: impl AsRef<str>) -> Vec<String> {
  string
    .as_ref()
    .split("<>")
    .map(|s| s.trim().to_owned())
    .filter(|s| !s.is_empty())
    .collect()
}

/// Gets input either via the `$MINEWORK_ENVIN` environment variable or `stdin`, while extracting useful values using the `crate::utils::extract_values` function.
///
/// # Arguments
///
/// * `suggestion` - The message to print in the `Suggestion` section when no input is provided.
pub(crate) fn get_non_interactive_input<'a>(suggestion: impl Into<String>) -> Result<Vec<String>> {
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

/// Configuration for non-interactive input parsing.
pub(crate) struct NonInteractiveInput<'a> {
  /// Number of expected input fields.
  pub(crate) fields: usize,
  /// Description of each field for the help message.
  pub(crate) descriptions: &'a [&'a str],
  /// Example commands to show in the help message.
  pub(crate) examples: &'a [&'a str],
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
      "\n   Input is expected to have {} field(s) concatenated using the <> operator with each position mapping to:\n{}\n\n   Here are some examples for your reference:\n{}",
      self.fields, field_list, examples
    )
  }

  /// Parse and validate input count.
  pub(crate) fn parse(&self) -> Result<Vec<String>> {
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

/// Validate the existance of the provided path and it being a directory.
///
/// # Arguments
///
/// * `path` - The target to validate.
pub(crate) fn validate_directory(path: impl AsRef<Path>) -> Result<()> {
  let path = path.as_ref();

  if !path.exists() {
    eyre::bail!("Directory {:?} does not exist", path);
  }

  if !path.is_dir() {
    eyre::bail!("While {:?} exists, it is not a directory", path);
  }

  Ok(())
}

/// Validates the provided string against a list of variants.
///
/// # Arguments
///
/// * `variants` - An iterable collection of string-like values representing valid options.
/// * `string` - The string to validate.
/// * `inclusive` - Controls validation behavior:
///   - `None` or `Some(true)`: Target MUST exist in variants (inclusive check). This is the default behaviour.
///   - `Some(false)`: Target MUST NOT exist in variants (exclusive check, for uniqueness).
pub(crate) fn validate_against_enum(
  variants: &[impl AsRef<str>],
  string: &str,
  inclusive: Option<bool>,
) -> Result<()> {
  let exists = variants.iter().any(|v| v.as_ref() == string);

  match (inclusive, exists) {
    (Some(false), true) => Err(eyre::eyre!("{string:?} is a duplicate entry")),
    (None | Some(true), false) => {
      use crossterm::style::Stylize;

      let options = variants
        .iter()
        .take(20)
        .map(|v| format!("     • {}", v.as_ref().green()))
        .collect::<Vec<_>>()
        .join("\n");

      let total = variants.len();
      let query = format!(
        "{} {} {} {} {} {}",
        "curl".cyan(),
        "-s https://api.modrinth.com/v2/tag/game_version |".yellow(),
        "jq".cyan(),
        "-r '.[].version' |".yellow(),
        "fzf".cyan(),
        "-f '<QUERY>'".yellow(),
      );
      let more_msg = format!(
        "\n     ... and {} more.\n\n   Search all versions using {}",
        total - 20,
        query,
      );

      Err(eyre::eyre!("Invalid entry {string:?}").suggestion(format!(
        "\n   Supported Minecraft versions include:\n{options}{more_msg}"
      )))
    }
    _ => Ok(()),
  }
}
