//! CLI entrypoint for `pin2k-rom-tool`.

mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    commands::dispatch(cli::Cli::parse())
}
