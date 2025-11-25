mod cli;
mod config;
mod utils;

use clap::Parser;
use cli::*;
use color_eyre::eyre::Result;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
  let mut args = Args::parse();

  utils::setup_hook(&args)?;

  if let SubCommand::Completion(shell) = args.subcommand {
    completion::generate(shell);
    return Ok(());
  }

  let config_file = utils::expand_path(&args.config_file)?;
  let config = Config::init(&config_file)?;
  args.config_file = config_file;

  match &args.subcommand {
    SubCommand::Profile(ProfileCommand::Create { name }) => {
      profile::create::init(name.as_ref(), config, &args).await?
    }
    SubCommand::Profile(ProfileCommand::Info { name }) => {
      profile::info::init(name.as_ref(), config, &args)?
    }
    _ => (),
  }

  Ok(())
}
