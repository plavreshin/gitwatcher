use std::io::{Error, ErrorKind};
use std::process::Command;

pub struct GitFlow {
    repo_path: String,
}

impl GitFlow {
    pub fn new(repo_path: String) -> Self {
        GitFlow { repo_path }
    }

    pub fn get_changed_files(&self) -> Result<Vec<String>, Error> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(&["status", "--porcelain"])
            .output()
            .map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to execute git command: {}", e),
                )
            })?;

        if !output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let changed_files: Vec<String> = output_str
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line[3..].to_string()) // Skip the status flags (first 3 characters)
            .collect();

        Ok(changed_files)
    }

    pub fn get_diff_as_text(&self) -> Result<Vec<String>, Error> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(&["diff"])
            .output()
            .map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to execute git diff: {}", e),
                )
            })?;

        if !output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let diff_text = String::from_utf8_lossy(&output.stdout).to_string();
        let lines: Vec<String> = diff_text.lines().map(String::from).collect();

        Ok(lines)
    }
}
