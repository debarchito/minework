//! Holds all implementations of `profile` subcommands and shared constants.

pub mod create;
pub mod delete;
pub mod info;
pub mod list;

const SUPPORTED_MOD_LOADERS: [&str; 2] = ["None", "Fabric"];
