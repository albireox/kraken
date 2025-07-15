use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::process::Command;
use version_compare::Version;

use crate::config::{DEFAULT_KRAKEN_CONFIG, KrakenConfig};

pub fn check_uv_version() -> Result<String, String> {
    // Checks if the `uv` command is available and retrieves its version.

    let min_version = Version::from("0.7.20").unwrap();

    let version: String = match Command::new("uv")
        .args(["self", "version", "--short"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                return Err(String::from("Failed to get uv version"));
            }
        }
        Err(e) => return Err(format!("Error executing uv: {}", e)),
    };

    let version = match version.split(" ").next() {
        Some(v) => Version::from(v).unwrap(),
        None => return Err("Failed parsing uv version".into()),
    };

    if version < min_version {
        return Err(format!(
            "uv version {} is less than the minimum required version {}",
            version, min_version
        ));
    }

    Ok(version.to_string())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PyProject {
    pub project: Project,
    pub kraken: Option<KrakenConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub version: String,
}

pub fn read_pyproject() -> Result<PyProject, String> {
    // Reads the `pyproject.toml` file.

    let content = match std::fs::read_to_string("pyproject.toml") {
        Ok(content) => content,
        Err(e) => return Err("Failed to read pyproject.toml: ".to_string() + &e.to_string()),
    };

    let mut pyproject: PyProject = match toml::from_str(&content) {
        Ok(pyproject) => pyproject,
        Err(e) => exit_with_error("Failed to parse pyproject.toml: ".to_string() + &e.to_string()),
    };

    if let None = pyproject.kraken {
        pyproject.kraken = Some(DEFAULT_KRAKEN_CONFIG);
    };

    Ok(pyproject)
}

pub fn exit_with_error(message: String) -> ! {
    // Prints an error message and exits the program.

    eprintln!("{}", format!("Error: {}", message).red());
    std::process::exit(1);
}
