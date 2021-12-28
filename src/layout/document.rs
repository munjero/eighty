use crate::{
    sitemap::{Sitemap, SitemapItem},
    document::RenderedDocument,
    Error,
};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentContext {
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

    pub sitemap: Vec<DocumentContextSitemapItem>,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentContextSiteLink {
    pub url: String,
    pub name: String,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentContextSitemapItem {
    pub title: String,
    pub path: String,
    pub children: Vec<DocumentContextSitemapItem>,
}

impl From<SitemapItem> for DocumentContextSitemapItem {
    fn from(item: SitemapItem) -> Self {
        Self {
            title: item.title,
            path: format!("{}", item.document_name.folder_path().display()),
            children: item
                .children
                .iter()
                .map(|child| child.clone().into())
                .collect(),
        }
    }
}

pub fn layout(
    rendered: &RenderedDocument,
    sitemap: &Sitemap,
    handlebars: &Handlebars,
) -> Result<String, Error> {
    let site_config = &rendered.site_metadata.config;

    let context = DocumentContext {
        site_title: site_config.title.clone(),
        site_title_only: false,
        site_base_url: site_config.base_url.clone(),

        page_title: rendered.data.title.clone(),
        page_description: Some(rendered.data.description.clone()),
        page_description_content: rendered.data.description_content.clone(),
        page_url: format!(
            "{}{}",
            rendered.site_metadata.config.url,
            rendered.metadata.name.folder_path().display()
        ),

        has_site_links: false,
        site_links: site_config.links.iter().map(|link| DocumentContextSiteLink {
            url: link.url.clone(),
            name: link.name.clone(),
        }).collect(),

        toc: rendered.data.toc.clone(),
        page_content: rendered.data.content.clone(),

        page_author_url: "https://social.that.world/@wei".to_string(),
        page_author: "Wei Tang".to_string(),
        page_copyright_years: "2019-2021".to_string(),

        page_license: rendered.data.license.clone(),
        page_license_code: rendered.data.license_code.clone(),

        sitemap: sitemap.iter().map(|child| child.clone().into()).collect(),
    };

    let layouted = handlebars.render("document/main", &context)?;

    Ok(layouted)
}