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
