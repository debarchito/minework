mod cli;
mod config;
mod utils;

use clap::Parser;
use cli::*;
use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let mut args = Args::parse();

  utils::setup_hook(&args)?;

  if let SubCommand::Completion(shell) = args.subcommand {
    completion::generate(shell);
    return Ok(());
  }

  let config_file = utils::expand_path(&args.config_file)?;
  let config = config::init(&config_file)?;
  args.config_file = config_file;

  if let SubCommand::Profile(ProfileCommand::Create { name }) = &args.subcommand {
    profile::create(name, config, &args).await?
  }

  Ok(())
}
