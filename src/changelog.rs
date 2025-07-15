use chrono::Local;
use regex::Regex;

use crate::cli::ChangelogDateFormat;
use crate::config::KrakenConfig;
use crate::tools::exit_with_error;

pub fn determine_changelog_date_format(
    changelog_path: &str,
) -> Result<ChangelogDateFormat, String> {
    // Determine the changelog date format based on command line arguments.

    let contents = match std::fs::read_to_string(changelog_path) {
        Ok(contents) => contents,
        Err(e) => exit_with_error("Failed to read changelog file: ".to_string() + &e.to_string()),
    };

    let re_short = Regex::new(r"#+.*\s*-\s*(\d{4}-\d{2}-\d{2})").unwrap();
    let re_long = Regex::new(r"#+.*\s*-\s*([A-Za-z]+ \d{1,2}, \d{4})").unwrap();

    if re_short.is_match(&contents) {
        return Ok(ChangelogDateFormat::Short);
    } else if re_long.is_match(&contents) {
        return Ok(ChangelogDateFormat::Long);
    }

    // If no date format is found, exit with an error.
    exit_with_error("No valid date format found in the changelog file.".to_string())
}

pub fn update_changelog(new_version: &str, config: &KrakenConfig) -> Result<(), String> {
    // Update the changelog next release header with the current date.

    let current_date = Local::now();

    let date_format = config.changelog_date_format.unwrap();
    let formatted_date = match date_format {
        ChangelogDateFormat::Long => current_date.format("%B %d, %Y").to_string(),
        ChangelogDateFormat::Short => current_date.format("%Y-%m-%d").to_string(),
        _ => return Err("Unsupported date format for changelog.".to_string()),
    };

    // Get the contents of the changelog file.
    let changelog_path = config.changelog_path.clone().unwrap();

    let contents = match std::fs::read_to_string(&changelog_path) {
        Ok(contents) => contents,
        Err(e) => {
            return Err(
                format!("Failed to read changelog file {changelog_path}: ") + &e.to_string()
            );
        }
    };

    // Perform a check to ensure the "Next release" header exists.
    let next_release_re = Regex::new(r"(?mi)^#+\s*Next (release|version).*\n$").unwrap();
    if !next_release_re.is_match(&contents) {
        return Err("Next release header not found in the changelog.".to_string());
    }
    println!(
        "{}",
        "$1".to_string() + &format!("{} - {}\n", new_version, formatted_date)
    );
    // Replace the "Next release" header with the current date.
    let next_release_replace_re =
        Regex::new(r"(?mi)^(#+\s*)(Next (release|version)).*\n$").unwrap();
    let updated_contents = next_release_replace_re.replace(
        &contents,
        format!("${{1}}{} - {}\n", new_version, formatted_date),
    );

    // Replace the contents of the changelog file.
    if let Err(e) = std::fs::write(changelog_path, updated_contents.as_bytes()) {
        return Err(format!(
            "Failed to write to changelog file: {}",
            &e.to_string()
        ));
    }

    // Here you would implement the logic to update the changelog file with the new date.
    Ok(())
}
