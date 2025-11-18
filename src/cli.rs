pub(crate) mod completion;
pub(crate) mod profile;

use clap::{Parser, Subcommand};

/// Minework helps you manage your Minecraft mods, mod packs, resource packs, data packs, shaders and plugins from Modrinth
#[derive(Parser)]
#[command(version, about)]
pub(crate) struct Args {
  #[command(subcommand)]
  pub(crate) subcommands: SubCommands,
  /// Specify the file path to read the configuration from
  #[arg(short, long, default_value = "$XDG_CONFIG_HOME/minework/config.json")]
  pub(crate) config_file: String,
}

/// Manage profiles
#[derive(Subcommand)]
pub(crate) enum SubCommands {
  /// Manage profiles
  #[command(subcommand, visible_alias = "pr")]
  Profile(ProfileCommands),
  /// Manage mods in the active profile
  #[command(subcommand, visible_alias = "m")]
  Mod(ModCommands),
  /// Manage modpacks in the active profile
  #[command(subcommand, visible_alias = "mp")]
  Modpack(ModpackCommands),
  /// Manage resourcepacks in the active profile
  #[command(subcommand, visible_alias = "rp")]
  Resourcepack(ResourcepackCommands),
  /// Manage datapacks in the active profile
  #[command(subcommand, visible_alias = "dp")]
  Datapack(DatapackCommands),
  /// Manage shaders in the active profile
  #[command(subcommand, visible_alias = "s")]
  Shader(ShaderCommands),
  /// Manage plugins in the active profile
  #[command(subcommand, visible_alias = "pl")]
  Plugin(PluginCommands),
  /// Generate completions for your shell
  #[command(subcommand)]
  Completion(Shells),
}

#[derive(Subcommand)]
pub(crate) enum ProfileCommands {
  /// Create a new profile. Tries to enages a picker if no default name is provided
  #[command(visible_aliases = &["c", "add", "a"])]
  Create { name: Option<String> },
  /// Edit an existing profile. Tries to default to active profile
  #[command(visible_alias = "e")]
  Edit { name: Option<String> },
  /// List all existing profiles
  #[command(visible_alias = "l")]
  List,
  /// Show information about an existing profile. Tries to default to active profile
  #[command(visible_alias = "i")]
  Info { name: Option<String> },
  /// Switch to another profile. Tries to enages a picker if no default name is provided
  #[command(visible_alias = "s")]
  Switch { name: Option<String> },
  /// Delete an existing profile. Tries to enages a picker if no default name is provided
  #[command(visible_aliases = &["d", "remove", "r"])]
  Delete { name: Option<String> },
}

#[derive(Subcommand)]
pub(crate) enum ModCommands {}

#[derive(Subcommand)]
pub(crate) enum ModpackCommands {}

#[derive(Subcommand)]
pub(crate) enum ResourcepackCommands {}

#[derive(Subcommand)]
pub(crate) enum DatapackCommands {}

#[derive(Subcommand)]
pub(crate) enum ShaderCommands {}

#[derive(Subcommand)]
pub(crate) enum PluginCommands {}

#[derive(Subcommand)]
pub(crate) enum Shells {
  /// Generate completions for Bash
  Bash,
  /// Generate completions for ZSH
  Zsh,
  /// Generate completions for Fish
  Fish,
  /// Generate completions for Elvish
  Elvish,
  /// Generate completions for PowerShell
  Powershell,
  /// Generate completions for Nushell
  Nushell,
}
