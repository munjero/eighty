use crate::Error;
use serde::{Deserialize, Serialize};
use std::{path::Path, process::Command};

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsciiDocOutput {
    pub document: AsciiDocOutputDocument,
    pub specs: Vec<AsciiDocOutputSpec>,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsciiDocOutputDocument {
    pub title: String,
    pub license: Option<String>,
    pub license_code: Option<String>,
    pub author: Option<String>,
    pub email: Option<String>,
    pub order: Option<usize>,
    pub toc: Option<String>,
    pub created: Option<String>,
    pub description: String,
    pub content: String,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsciiDocOutputSpec {
    pub id: String,
    pub description: String,
    pub discuss: String,
    pub url: String,
}

pub fn process_asciidoc(site_path: &Path, rel_path: &Path) -> Result<AsciiDocOutput, Error> {
    let output = Command::new("eighty-asciidoc")
        .arg(rel_path)
        .current_dir(site_path)
        .output()?;

    if !output.status.success() {
        return Err(Error::RunCommandFailed);
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}
