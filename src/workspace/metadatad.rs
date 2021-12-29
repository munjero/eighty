// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of Eighty.
//
// Copyright (c) 2021 Wei Tang.
//
// Eighty is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Eighty is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Eighty. If not, see <http://www.gnu.org/licenses/>.

use crate::{
    document::{DocumentMetadata, DocumentName, DocumentType},
    file::FileMetadata,
    site::{SiteMetadata, SiteName},
    Error,
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use walkdir::WalkDir;

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct MetadatadWorkspace {
    pub root_path: PathBuf,
    pub sites: HashMap<SiteName, MetadatadSite>,
}

impl MetadatadWorkspace {
    pub fn new(root_path: &Path) -> Result<Self, Error> {
        let mut sites = HashMap::new();

        let root_subfolders = fs::read_dir(root_path)?;

        for site_folder in root_subfolders {
            let site_folder = site_folder?;
            let site_name = site_folder
                .file_name()
                .into_string()
                .map_err(|_| Error::PathContainNonUnicode)?;

            if site_name.starts_with(".") || site_name.starts_with("_") {
                continue;
            }

            let site = Arc::new(SiteMetadata::new(
                SiteName(site_name.clone()),
                &site_folder.path(),
            )?);
            let item = MetadatadSite::new(site.clone())?;

            sites.insert(SiteName(site_name), item);
        }

        Ok(Self {
            sites,
            root_path: root_path.to_owned(),
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct MetadatadSite {
    pub site: Arc<SiteMetadata>,
    pub documents: HashMap<DocumentName, Arc<DocumentMetadata>>,
    pub files: Arc<HashMap<PathBuf, FileMetadata>>,
}

impl MetadatadSite {
    pub fn new(site: Arc<SiteMetadata>) -> Result<Self, Error> {
        let mut documents = HashMap::new();
        let mut files = HashMap::new();

        let walker = WalkDir::new(&site.source_path)
            .into_iter()
            .filter_entry(|entry| {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name == "_posts" && entry.file_type().is_dir() {
                        return true;
                    }

                    if file_name.starts_with(".") || file_name.starts_with("_") {
                        return false;
                    }

                    return true;
                }

                return false;
            });

        for entry in walker {
            let entry = entry?;

            if entry.file_type().is_file() {
                let modified = fs::metadata(entry.path())?.modified()?;

                let typ = if let Some(extension) = entry.path().extension() {
                    let extension = extension.to_str().ok_or(Error::PathContainNonUnicode)?;

                    match extension {
                        "md" => Some(DocumentType::Markdown),
                        "adoc" => Some(DocumentType::AsciiDoc),
                        _ => None,
                    }
                } else {
                    None
                };

                if let Some(typ) = typ {
                    let document = DocumentMetadata::new(&site, entry.path(), typ, modified)?;
                    documents.insert(document.name.clone(), Arc::new(document));
                } else {
                    let rel_file_path = entry.path().strip_prefix(&site.source_path)?;
                    let content = fs::read(entry.path())?;

                    let file = FileMetadata {
                        site: site.clone(),
                        path: rel_file_path.to_owned(),
                        source_path: entry.path().to_owned(),
                        content,
                        modified,
                    };

                    files.insert(file.path.clone(), file);
                }
            }
        }

        Ok(Self {
            site,
            documents,
            files: Arc::new(files),
        })
    }
}
