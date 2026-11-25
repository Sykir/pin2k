//! Native Rust Pinball 2000 cabinet & desktop runtime entrypoint.

mod audio;
mod backend;
mod cli;
mod compositor;
mod demo_scripts;
mod raster;
mod runner;
mod telemetry;
mod web_ui;
mod window;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    runner::run(cli::Cli::parse())
}
