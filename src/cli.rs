pub use clap::{Parser, ValueEnum};
pub use clap_cargo::style::CLAP_STYLING;
use serde::{Deserialize, Serialize};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(styles = CLAP_STYLING)]
pub struct Args {
    /// Version to release.
    #[arg()]
    pub new_version: String,

    /// Changelog date format.
    #[arg(long, value_enum)]
    pub changelog_date_format: Option<ChangelogDateFormat>,

    /// Path to the CHANGELOG file.
    #[arg(long)]
    pub changelog_path: Option<String>,

    /// Bump the version after release.
    #[arg(long)]
    pub bump_after_release: bool,

    /// Do not bump the version after release.
    #[arg(long, conflicts_with = "bump_after_release")]
    pub no_bump_after_release: bool,

    /// Commit changes to the git repository.
    #[arg(long, default_value_t = true)]
    pub commit_changes: bool,

    /// Do not commit changes to the git repository.
    #[arg(long, conflicts_with = "commit_changes")]
    pub no_commit_changes: bool,
}

#[derive(Copy, Clone, ValueEnum, Debug, Serialize, Deserialize)]
#[serde(rename = "changelog_date_format")]
#[serde(rename_all = "lowercase")]
pub enum ChangelogDateFormat {
    /// Automatically determine the date format.
    Auto,
    /// Long format date, e.g. June 21, 2025.
    Long,
    /// Short format date, e.g. 2025-06-21.
    Short,
}
