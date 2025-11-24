//! Bunch of helpful validation functions.

use color_eyre::Section;
use color_eyre::eyre::{self, Result};
use std::path::Path;

/// Validate the existance of the provided path and it being a directory.
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

/// Validates the provided string against a list of variants.
///
/// # Arguments
///
/// * `variants` - An iterable collection of string-like values representing valid options.
/// * `string` - The string to validate.
/// * `inclusive` - Controls validation behavior:
///   - `None` or `Some(true)`: Target MUST exist in variants (inclusive check). This is the default behaviour.
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
  use crossterm::style::Stylize;

  let options = minecraft_versions
    .iter()
    .take(20)
    .map(|v| format!("     • {}", v.as_ref().green()))
    .collect::<Vec<_>>()
    .join("\n");

  let total = minecraft_versions.len();
  let query = format!(
    "{} {} {} {} {} {}",
    "curl".cyan(),
    "-s https://api.modrinth.com/v2/tag/game_version |".yellow(),
    "jq".cyan(),
    "-r '.[].version' |".yellow(),
    "fzf".cyan(),
    "-f '<QUERY>'".yellow(),
  );

  let more_msg = if total > 20 {
    format!(
      "\n     ... and {} more.\n\n   Search all versions using:\n     {}",
      total - 20,
      query
    )
  } else {
    format!("\n\n   Search all versions using:\n     {}", query)
  };

  let context = format!("\n   Supported Minecraft versions include:\n{options}{more_msg}");

  against_enum(minecraft_versions, version, None, Some(context))
}

pub mod inquire {
  //! `inquire`-specific versions of the validation functions.

  use ::inquire::validator::{ErrorMessage, StringValidator, Validation};
  use std::error::Error;
  use std::path::PathBuf;

  /// Internal validator that checks if a string exists in a list of variants.
  #[derive(Clone)]
  struct EnumValidator {
    variants: Vec<String>,
    inclusive: bool,
  }

  impl StringValidator for EnumValidator {
    fn validate(&self, s: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
      let exists = self.variants.iter().any(|v| v.as_str() == s);
      match (self.inclusive, exists) {
        (false, true) => Ok(Validation::Invalid(ErrorMessage::Custom(format!(
          "{s:?} is a duplicate entry"
        )))),
        (true, false) => {
          let options = self
            .variants
            .iter()
            .map(|v| format!("  • {}", v))
            .collect::<Vec<_>>()
            .join("\n");
          Ok(Validation::Invalid(ErrorMessage::Custom(format!(
            "Invalid entry {s:?}. Valid options:\n{options}"
          ))))
        }
        _ => Ok(Validation::Valid),
      }
    }
  }

  /// Creates a validator that checks if a string exists in a list of variants.
  ///
  /// # Arguments
  ///
  /// * `variants` - An iterable collection of string-like values representing valid options.
  /// * `inclusive` - Controls validation behavior:
  ///   - `None` or `Some(true)`: Value MUST exist in variants (inclusive check). This is the default behaviour.
  ///   - `Some(false)`: Value MUST NOT exist in variants (exclusive check, for uniqueness).
  pub fn against_enum(
    variants: &[impl AsRef<str>],
    inclusive: Option<bool>,
  ) -> Box<dyn StringValidator> {
    Box::new(EnumValidator {
      variants: variants.iter().map(|v| v.as_ref().to_string()).collect(),
      inclusive: inclusive.unwrap_or(true),
    })
  }

  /// Internal validator that checks if a path exists and is a directory.
  #[derive(Clone)]
  struct DirectoryValidator;

  impl StringValidator for DirectoryValidator {
    fn validate(&self, s: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
      match shellexpand::full(s) {
        Ok(expanded) => {
          let path = PathBuf::from(expanded.as_ref());
          if !path.exists() {
            Ok(Validation::Invalid(ErrorMessage::Custom(
              "Path does not exist".to_owned(),
            )))
          } else if !path.is_dir() {
            Ok(Validation::Invalid(ErrorMessage::Custom(
              "Path is not a directory".to_owned(),
            )))
          } else {
            Ok(Validation::Valid)
          }
        }
        Err(e) => Ok(Validation::Invalid(ErrorMessage::Custom(format!(
          "Invalid path: {}",
          e
        )))),
      }
    }
  }

  /// Creates a validator that checks if a path exists and is a directory.
  ///
  /// Supports shell expansion (e.g., `~`, `$HOME`) and validates the expanded path.
  pub fn is_valid_directory() -> Box<dyn StringValidator> {
    Box::new(DirectoryValidator)
  }

  /// Internal validator for non-empty strings.
  #[derive(Clone)]
  struct NonEmptyValidator {
    field_name: String,
  }

  impl StringValidator for NonEmptyValidator {
    fn validate(&self, s: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
      if s.trim().is_empty() {
        Ok(Validation::Invalid(ErrorMessage::Custom(format!(
          "{} cannot be empty",
          self.field_name
        ))))
      } else {
        Ok(Validation::Valid)
      }
    }
  }

  /// Creates a validator that ensures a string is non-empty.
  ///
  /// # Arguments
  ///
  /// * `field_name` - Name of the field being validated, used in error messages.
  pub fn is_non_empty(field_name: impl Into<String>) -> Box<dyn StringValidator> {
    Box::new(NonEmptyValidator {
      field_name: field_name.into(),
    })
  }
}
