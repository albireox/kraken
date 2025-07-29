use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use version_compare::Version;

use crate::config::KrakenConfig;

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
        Err(e) => return Err("Failed to parse pyproject.toml: ".to_string() + &e.to_string()),
    };

    if let None = pyproject.kraken {
        pyproject.kraken = Some(KrakenConfig::default());
    }

    Ok(pyproject)
}

pub fn get_package_version() -> Result<String, String> {
    // Retrieves the package version from `pyproject.toml`.

    let result = Command::new("uv").args(["version", "--short"]).output();

    if let Err(e) = result {
        return Err(format!("Failed to get package version: {}", e));
    }

    let output = result.unwrap();

    return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
}

pub fn update_version(release_version: &str) -> Result<(), String> {
    // Updates the version in `pyproject.toml`.

    // Run the uv version command.
    if let Err(e) = Command::new("uv")
        .args(["version", release_version])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        return Err(format!("Failed to update uv version: {}", e));
    }

    Ok(())
}

pub fn exit_with_error(message: String) -> ! {
    // Prints an error message and exits the program.

    eprintln!("{}", format!("Error: {}", message).red());
    std::process::exit(1);
}

pub fn git_add_commit_tag_push(commit_message: String, tag: bool) -> Result<(), String> {
    // Adds, commits, and pushes changes to the git repository.

    for file in &vec!["pyproject.toml", "CHANGELOG.md", "uv.lock"] {
        if let Err(e) = Command::new("git")
            .args(["add", file])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            return Err(format!("Failed to add {} to git: {}", file, e));
        }
    }

    let release_version = &get_package_version().unwrap();

    if let Err(e) = Command::new("git")
        .args(["commit", "-m", commit_message.as_str()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        return Err(format!("Failed to commit changes to git: {}", e));
    }

    if let Err(e) = Command::new("git")
        .args(["push"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        return Err(format!("Failed to push changes to git: {}", e));
    }

    if tag {
        if let Err(e) = Command::new("git")
            .args([
                "tag",
                "-a",
                release_version,
                "-m",
                format!("Release {}", release_version).as_str(),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            return Err(format!("Failed to create git tag: {}", e));
        }

        if let Err(e) = Command::new("git")
            .args(["push", "--tags"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            return Err(format!("Failed to push --tags: {}", e));
        }
    }

    Ok(())
}

pub fn bump_to_prerelease() -> Result<(), String> {
    // Bumps the version to a pre-release version.
    let result = Command::new("uv")
        .args(["version", "--bump", "patch", "--bump", "alpha"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if let Err(e) = result {
        return Err(format!("Failed to bump version to pre-release: {}", e));
    }

    Ok(())
}
