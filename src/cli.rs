//! Everything related to the cli.

pub mod completion;
pub mod profile;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Minework helps you manage your Minecraft mods, mod packs, resource packs, data packs, shaders and plugins from Modrinth.
#[derive(Parser)]
#[command(version, about)]
pub struct Args {
  #[command(subcommand)]
  pub subcommand: SubCommand,
  /// Specify the file path to read the configuration from.
  #[arg(short, long, default_value = "$XDG_CONFIG_HOME/minework/config.json")]
  pub config_file: PathBuf,
  /// Disable colored outputs. Additionally, minework respects the $NO_COLOR environment variable (<https://no-color.org>) to disable colors.
  #[arg(short = 'C', long)]
  pub no_color: bool,
  /// Disable the file location section in error outputs.
  #[arg(short = 'L', long)]
  pub no_location_section: bool,
  /// Disable the backtrace section in error outputs.
  #[arg(short = 'B', long)]
  pub no_backtrace_section: bool,
  /// Run in non-interactive mode. Input is expected either via the $MINEWORK_ENVIN environment variable or stdin with the former getting higher priority.
  #[arg(short = 'I', long)]
  pub non_interactive: bool,
}

/// Manage profiles
#[derive(Subcommand)]
pub enum SubCommand {
  /// Manage profiles.
  #[command(subcommand, visible_alias = "pr")]
  Profile(ProfileCommand),
  /// Manage mods in the active profile.
  #[command(subcommand, visible_alias = "m")]
  Mod(ModCommand),
  /// Manage modpacks in the active profile.
  #[command(subcommand, visible_alias = "mp")]
  Modpack(ModpackCommand),
  /// Manage resourcepacks in the active profile.
  #[command(subcommand, visible_alias = "rp")]
  Resourcepack(ResourcepackCommand),
  /// Manage datapacks in the active profile.
  #[command(subcommand, visible_alias = "dp")]
  Datapack(DatapackCommand),
  /// Manage shaders in the active profile.
  #[command(subcommand, visible_alias = "s")]
  Shader(ShaderCommand),
  /// Manage plugins in the active profile.
  #[command(subcommand, visible_alias = "pl")]
  Plugin(PluginCommand),
  /// Generate completions for your shell.
  #[command(subcommand)]
  Completion(Shell),
}

#[derive(Subcommand)]
pub enum ProfileCommand {
  /// Create a new profile. Tries to engage a picker if no default name is provided in interactive mode.
  #[command(visible_aliases = &["c", "add", "a"])]
  Create { name: Option<String> },
  /// Edit an existing profile. Tries to default to active profile.
  #[command(visible_alias = "e")]
  Edit { name: Option<String> },
  /// List all existing profiles.
  #[command(visible_alias = "l")]
  List,
  /// Show information about an existing profile. Tries to default to active profile.
  #[command(visible_alias = "i")]
  Info {
    name: Option<String>,
    /// Open the picker to interactively select a profile. It is not supported in non-interactive mode.
    #[arg(short, long)]
    picker: bool,
  },
  /// Switch to another profile. Tries to engage a picker if no default name is provided in interactive mode.
  #[command(visible_alias = "s")]
  Switch { name: Option<String> },
  /// Delete an existing profile. Tries to engage a picker if no default name is provided in interactive mode.
  #[command(visible_aliases = &["d", "remove", "r"])]
  Delete { name: Option<String> },
}

#[derive(Subcommand)]
pub enum ModCommand {}

#[derive(Subcommand)]
pub enum ModpackCommand {}

#[derive(Subcommand)]
pub enum ResourcepackCommand {}

#[derive(Subcommand)]
pub enum DatapackCommand {}

#[derive(Subcommand)]
pub enum ShaderCommand {}

#[derive(Subcommand)]
pub enum PluginCommand {}

#[derive(Subcommand)]
/// Supported shells.
pub enum Shell {
  /// Generate completions for Bash.
  Bash,
  /// Generate completions for ZSH.
  Zsh,
  /// Generate completions for Fish.
  Fish,
  /// Generate completions for Elvish.
  Elvish,
  /// Generate completions for PowerShell.
  Powershell,
  /// Generate completions for Nushell.
  Nushell,
}
