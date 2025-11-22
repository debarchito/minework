//! Implements all the completion subcommands

use clap::CommandFactory;

/// Generates shell completion scripts.
///
/// Writes the completion script for the specified shell to standard output.
/// Supported shells include Bash, Zsh, Fish, Elvish, PowerShell, and Nushell.
pub(crate) fn generate(shell: super::Shell) {
  use super::Shell;
  use clap_complete::generate;
  use clap_complete::shells::Shell::*;
  use clap_complete_nushell::Nushell;

  let mut cmd = super::Args::command();
  let bin = cmd.get_name().to_string();
  let mut stdout = std::io::stdout();

  match shell {
    Shell::Bash => generate(Bash, &mut cmd, bin, &mut stdout),
    Shell::Zsh => generate(Zsh, &mut cmd, bin, &mut stdout),
    Shell::Fish => generate(Fish, &mut cmd, bin, &mut stdout),
    Shell::Elvish => generate(Elvish, &mut cmd, bin, &mut stdout),
    Shell::Powershell => generate(PowerShell, &mut cmd, bin, &mut stdout),
    Shell::Nushell => generate(Nushell, &mut cmd, bin, &mut stdout),
  }
}
