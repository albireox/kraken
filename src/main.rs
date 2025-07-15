mod changelog;
mod cli;
mod config;
mod tools;

use crate::changelog::update_changelog;
use crate::cli::{Args, Parser};
use crate::config::{DEFAULT_KRAKEN_CONFIG, update_config};
use crate::tools::{check_uv_version, exit_with_error, read_pyproject};

fn main() {
    // Parse command line arguments.
    let args = Args::parse();

    // Check the uv version to ensure compatibility.
    if let Err(e) = check_uv_version() {
        exit_with_error(e);
    }

    // Read the pyproject.toml file to get the project configuration.
    // unwrap() is safe here because we handle errors in read_pyproject().
    let pyproject = read_pyproject().unwrap();

    // Unpack the kraken configuration from the pyproject.
    let mut kraken_config = pyproject.kraken.unwrap_or(DEFAULT_KRAKEN_CONFIG);

    // Update the kraken configuration based on command line arguments.
    update_config(&args, &mut kraken_config);

    if let Err(e) = update_changelog(&args.new_version, &kraken_config) {
        exit_with_error(e);
    };
}
