#[macro_use]
extern crate serde;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate miette;
#[macro_use]
extern crate prettytable;

use clap::Parser;
use cli::{
    commands::{self, Commands},
    Cli,
};
use miette::{Context, IntoDiagnostic, Result};
use node::is_node_project;

mod cli;
mod node;
mod npm;

fn main() -> Result<()> {
    let args = Cli::parse();

    if !is_node_project() {
        panic!("Not an npm project");
    }

    let npm_executable_path = which::which("npm")
        .into_diagnostic()
        .wrap_err("NPM not found")?;

    match args.command {
        Commands::CheckDep {
            package_name,
            target_version,
        } => {
            commands::check_upgrade::execute(&npm_executable_path, &package_name, &target_version)?
        }
    }
    Ok(())
}
