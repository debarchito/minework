mod cli;
mod config;
use clap::Parser;
use cli::*;
use color_eyre::eyre::{Context, Result};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
  color_eyre::install()?;

  let args = Args::parse();

  if let SubCommands::Completion(shell) = args.subcommands {
    return Ok(completion::generate(shell));
  }

  let config_file_expanded = shellexpand::full(&args.config_file)
    .context(format!("Failed to expand \"{}\"", &args.config_file))?;
  let config_file = PathBuf::from(config_file_expanded.into_owned());
  let config = config::init(&config_file)?;

  match args.subcommands {
    SubCommands::Profile(ProfileCommands::Create { name }) => {
      profile::create(name, config, config_file).await?
    }
    _ => (),
  }

  Ok(())
}
