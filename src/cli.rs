pub use clap::{Parser, ValueEnum};
pub use clap_cargo::style::CLAP_STYLING;
use serde::{Deserialize, Serialize};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(styles = CLAP_STYLING)]
pub struct Args {
    /// Version to release
    #[arg(value_name = "NEW-VERSION")]
    pub new_version: String,

    /// Changelog date format
    #[arg(long, value_enum)]
    pub changelog_date_format: Option<ChangelogDateFormat>,

    /// Path to the CHANGELOG file
    #[arg(long)]
    pub changelog_path: Option<String>,
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
