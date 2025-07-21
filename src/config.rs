use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::changelog::determine_changelog_date_format;
use crate::cli::Args;
use crate::cli::ChangelogDateFormat;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KrakenConfig {
    #[serde_inline_default(Some(ChangelogDateFormat::Auto))]
    pub changelog_date_format: Option<ChangelogDateFormat>,

    #[serde_inline_default(Some("CHANGELOG.md".to_string()))]
    pub changelog_path: Option<String>,

    #[serde_inline_default(Some(true))]
    pub bump_after_release: Option<bool>,

    #[serde_inline_default(Some(true))]
    pub commit_changes: Option<bool>,
}

impl Default for KrakenConfig {
    fn default() -> Self {
        Self {
            changelog_date_format: Some(ChangelogDateFormat::Auto),
            changelog_path: Some("CHANGELOG.md".to_string()),
            bump_after_release: Some(true),
            commit_changes: Some(true),
        }
    }
}

pub fn update_config(args: &Args, kraken_config: &mut KrakenConfig) {
    // Update the kraken configuration based on command line arguments.

    // If a changelog date format is provided in the CLI, update the config.
    if let Some(date_format) = args.changelog_date_format {
        kraken_config.changelog_date_format = Some(date_format);
    }

    // If a changelog path is provided in the CLI, update the config.
    if let Some(path) = &args.changelog_path {
        kraken_config.changelog_path = Some(path.to_string());
    }

    // If the changelog date format is set to auto, determine it based on the changelog file.
    if let Some(ChangelogDateFormat::Auto) = kraken_config.changelog_date_format {
        // Determine the changelog date format automatically.
        if let Some(ref changelog_path) = kraken_config.changelog_path {
            kraken_config.changelog_date_format =
                determine_changelog_date_format(changelog_path).ok();
        }
    }

    // Set the bump_after_release flag based on command line arguments.
    if args.bump_after_release {
        kraken_config.bump_after_release = Some(true);
    } else if args.no_bump_after_release {
        kraken_config.bump_after_release = Some(false);
    }

    // Set the commit_changes flag based on command line arguments.
    if args.commit_changes {
        kraken_config.commit_changes = Some(true);
    } else if args.no_commit_changes {
        kraken_config.commit_changes = Some(false);
    }
}
