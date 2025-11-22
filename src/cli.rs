pub(crate) mod completion;
pub(crate) mod profile;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Minework helps you manage your Minecraft mods, mod packs, resource packs, data packs, shaders and plugins from Modrinth.
#[derive(Parser)]
#[command(version, about)]
pub(crate) struct Args {
  #[command(subcommand)]
  pub(crate) subcommand: SubCommand,
  /// Specify the file path to read the configuration from.
  #[arg(short, long, default_value = "$XDG_CONFIG_HOME/minework/config.json")]
  pub(crate) config_file: PathBuf,
  /// Disable colored outputs. Minework also supports the $NO_COLOR environment variable (https://no-color.org) to disable colors.
  #[arg(short = 'C', long)]
  pub(crate) no_color: bool,
  /// Disable the file location section in error outputs.
  #[arg(short = 'L', long)]
  pub(crate) no_location_section: bool,
  /// Disable the backtrace section in error outputs.
  #[arg(short = 'B', long)]
  pub(crate) no_backtrace_section: bool,
  /// Run in non-interactive mode. Input is expected either via the $MINEWORK_ENVIN environment variable or stdin with the former getting higher priority.
  #[arg(short = 'I', long)]
  pub(crate) non_interactive: bool,
}

/// Manage profiles
#[derive(Subcommand)]
pub(crate) enum SubCommand {
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
pub(crate) enum ProfileCommand {
  /// Create a new profile. Tries to enages a picker if no default name is provided.
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
  Info { name: Option<String> },
  /// Switch to another profile. Tries to enages a picker if no default name is provided.
  #[command(visible_alias = "s")]
  Switch { name: Option<String> },
  /// Delete an existing profile. Tries to enages a picker if no default name is provided.
  #[command(visible_aliases = &["d", "remove", "r"])]
  Delete { name: Option<String> },
}

#[derive(Subcommand)]
pub(crate) enum ModCommand {}

#[derive(Subcommand)]
pub(crate) enum ModpackCommand {}

#[derive(Subcommand)]
pub(crate) enum ResourcepackCommand {}

#[derive(Subcommand)]
pub(crate) enum DatapackCommand {}

#[derive(Subcommand)]
pub(crate) enum ShaderCommand {}

#[derive(Subcommand)]
pub(crate) enum PluginCommand {}

#[derive(Subcommand)]
/// Supported shells.
pub(crate) enum Shell {
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
