//! Holds all implementations of `profile` subcommands and shared constants.

pub mod create;
pub mod info;

const SUPPORTED_MOD_LOADERS: [&str; 2] = ["None", "Fabric"];
