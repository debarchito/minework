mod cli;
mod config;
use clap::Parser;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
  color_eyre::install()?;

  let args = cli::Args::parse();
  let _config = config::init(&args.config_file)?;

  Ok(())
}
