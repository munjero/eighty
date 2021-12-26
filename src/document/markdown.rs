use std::{path::Path, process::Command};
use serde::{Serialize, Deserialize};
use crate::Error;

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownOutput {
    pub title: String,
    pub description: String,
    pub description_content: String,
    pub order: Option<usize>,
    pub content: String,
    pub toc: String,
}

pub fn process_markdown(site_path: &Path, rel_path: &Path) -> Result<MarkdownOutput, Box<dyn std::error::Error>> {
    let output = Command::new("eighty-pandoc")
        .arg(rel_path)
        .current_dir(site_path)
        .output()?;

    if !output.status.success() {
        return Err(Box::new(Error::RunCommandFailed))
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}
