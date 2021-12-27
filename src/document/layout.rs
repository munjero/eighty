use serde::{Deserialize, Serialize};
use std::sync::Arc;
use super::{DocumentMetadata, RenderedDocument};
use handlebars::Handlebars;
use crate::Error;

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentContext {
    pub site_title: String,
    pub site_title_only: bool,
    pub site_base_url: String,

    pub page_title: String,
    pub page_description: Option<String>,
    pub page_description_content: Option<String>,
    pub page_url: String,

    pub has_site_links: bool,
    pub site_links: Vec<DocumentContextSiteLink>,

    pub toc: Option<String>,
    pub page_content: String,

    pub page_author_url: String,
    pub page_author: String,
    pub page_copyright_years: String,

    pub page_license: Option<String>,
    pub page_license_code: Option<String>,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentContextSiteLink {
    pub url: String,
    pub name: String,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct LayoutedDocument {
    pub metadata: Arc<DocumentMetadata>,
    pub content: String,
}

impl LayoutedDocument {
    pub fn new(rendered: Arc<RenderedDocument>, handlebars: &Handlebars) -> Result<LayoutedDocument, Error> {
        let site_config = &rendered.metadata.site.config;

        let context = DocumentContext {
            site_title: site_config.title.clone(),
            site_title_only: false,
            site_base_url: site_config.base_url.clone(),

            page_title: rendered.title.clone(),
            page_description: None,
            page_description_content: None,
            page_url: rendered.metadata.url(),

            has_site_links: false,
            site_links: Vec::new(),

            toc: None,
            page_content: rendered.content.clone(),

            page_author_url: "https://social.that.world/@wei".to_string(),
            page_author: "Wei Tang".to_string(),
            page_copyright_years: "2019-2021".to_string(),

            page_license: None,
            page_license_code: None,
        };

        let layouted = handlebars.render("document", &context)?;

        Ok(LayoutedDocument {
            metadata: rendered.metadata.clone(),
            content: layouted,
        })
    }
}
