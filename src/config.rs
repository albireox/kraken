use serde::{Deserialize, Serialize};

use crate::changelog::determine_changelog_date_format;
use crate::cli::Args;
use crate::cli::ChangelogDateFormat;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KrakenConfig {
    pub changelog_date_format: Option<ChangelogDateFormat>,
    pub changelog_path: Option<String>,
}

pub const DEFAULT_KRAKEN_CONFIG: KrakenConfig = KrakenConfig {
    changelog_date_format: Some(ChangelogDateFormat::Auto),
    changelog_path: None,
};

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

    // If no changelog path is not set, use the default "CHANGELOG.md".
    // We do this here and not in DEFAULT_KRAKEN_CONFIG because we cannot
    // set a constant with a string value that is not known at compile time.
    if let None = kraken_config.changelog_path {
        kraken_config.changelog_path = Some("CHANGELOG.md".to_string());
    }

    if let Some(ChangelogDateFormat::Auto) = kraken_config.changelog_date_format {
        // Determine the changelog date format automatically.
        if let Some(ref changelog_path) = kraken_config.changelog_path {
            kraken_config.changelog_date_format =
                determine_changelog_date_format(changelog_path).ok();
        }
    }
}
