mod changelog;
mod cli;
mod config;
mod tools;

use crate::changelog::update_changelog;
use crate::cli::{Args, Parser};
use crate::config::update_config;
use crate::tools::{
    bump_to_prerelease, check_uv_version, exit_with_error, get_package_version,
    git_add_commit_tag_push, read_pyproject, update_version,
};
use colored::Colorize;

fn main() {
    // Parse command line arguments.
    let args = Args::parse();

    // Check the uv version to ensure compatibility.
    if let Err(e) = check_uv_version() {
        exit_with_error(e);
    }

    // Read the pyproject.toml file to get the project configuration.
    if let Err(e) = read_pyproject() {
        exit_with_error(e);
    }
    let pyproject = read_pyproject().unwrap();

    // Unpack the kraken configuration from the pyproject.
    let mut kraken_config = pyproject.kraken.unwrap();

    // Update the kraken configuration based on command line arguments.
    update_config(&args, &mut kraken_config);

    // Update the changelog version.
    if let Err(e) = update_changelog(&args.new_version, &kraken_config) {
        exit_with_error(e);
    };

    // Update the version in pyproject.toml using uv.
    if let Err(e) = update_version(&args.new_version) {
        exit_with_error(e);
    }

    // Add, commit, and push changes to the git repository.
    if let Some(true) = kraken_config.commit_changes {
        let commit_message = format!("Release {}", args.new_version);
        if let Err(e) = git_add_commit_tag_push(commit_message) {
            exit_with_error(e);
        }

        println!(
            "{}",
            format!("Changes committed and pushed to git repository.").bright_black()
        );

        // Bump the version after release if specified.
        if let Some(true) = kraken_config.bump_after_release {
            if let Err(e) = bump_to_prerelease() {
                exit_with_error(e);
            }

            // Get the new version after bumping to pre-release.
            let new_version = get_package_version();
            if let Err(e) = new_version {
                exit_with_error(e);
            }

            // Commit the changes after bumping to pre-release.
            let commit_message = format!("Bump version to {}", new_version.as_ref().unwrap());
            if let Err(e) = git_add_commit_tag_push(commit_message) {
                exit_with_error(e);
            }

            println!(
                "{}",
                format!("Version bumped to {}", new_version.as_ref().unwrap()).bright_black()
            );
        }
    }
}
