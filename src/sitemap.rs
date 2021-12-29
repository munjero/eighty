use crate::document::DocumentName;
use std::{fmt, ops::Deref};

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct LocalSitemap {
    pub breadcrumb: Breadcrumb,
    pub children: Vec<BreadcrumbItem>,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Breadcrumb(pub Vec<BreadcrumbItem>);

impl Deref for Breadcrumb {
    type Target = Vec<BreadcrumbItem>;

    fn deref(&self) -> &Vec<BreadcrumbItem> {
        &self.0
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct BreadcrumbItem {
    pub title: String,
    pub document_name: DocumentName,
    pub description: String,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SitemapItem {
    pub item: BreadcrumbItem,
    pub children: Vec<SitemapItem>,
}

impl SitemapItem {
    pub fn maybe_insert(&mut self, item: BreadcrumbItem) -> bool {
        if self.item.document_name.is_ancestor_of(&item.document_name) {
            let mut inserted = false;

            for child in &mut self.children {
                if child.maybe_insert(item.clone()) {
                    inserted = true;
                    break;
                }
            }

            if !inserted {
                self.children.push(SitemapItem {
                    item,
                    children: Vec::new(),
                });
            }

            true
        } else {
            false
        }
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Sitemap(pub Vec<SitemapItem>);

impl Sitemap {
    pub fn insert(&mut self, item: BreadcrumbItem) {
        let mut inserted = false;

        for child in &mut self.0 {
            if child.maybe_insert(item.clone()) {
                inserted = true;
                break;
            }
        }

        if !inserted {
            self.0.push(SitemapItem {
                item,
                children: Vec::new(),
            });
        }
    }

    pub fn local(&self, document_name: &DocumentName) -> Option<LocalSitemap> {
        if document_name.is_root() {
            return Some(LocalSitemap {
                breadcrumb: Breadcrumb(Vec::new()),
                children: self
                    .0
                    .iter()
                    .map(|item| item.item.clone())
                    .collect(),
            });
        }

        let mut breadcrumb = Vec::new();
        let mut current = &self.0;

        loop {
            let target = current.iter().find(|item| {
                item.item.document_name.is_ancestor_of(&document_name)
                    || item.item.document_name == *document_name
            });

            if let Some(target) = target {
                if target.item.document_name == *document_name {
                    return Some(LocalSitemap {
                        breadcrumb: Breadcrumb(breadcrumb),
                        children: target
                            .children
                            .iter()
                            .map(|item| item.item.clone())
                            .collect(),
                    });
                } else {
                    breadcrumb.push(target.item.clone());

                    current = &target.children;
                }
            } else {
                return None;
            }
        }
    }
}

impl From<Vec<BreadcrumbItem>> for Sitemap {
    fn from(mut name_titles: Vec<BreadcrumbItem>) -> Sitemap {
        name_titles.sort_by_key(|k| k.document_name.clone());
        let ordered_name_titles = name_titles;

        let mut sitemap = Sitemap(Vec::new());
        for item in &ordered_name_titles {
            if !item.document_name.is_root() {
                sitemap.insert(item.clone());
            }
        }

        sitemap
    }
}

impl Deref for Sitemap {
    type Target = Vec<SitemapItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Sitemap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for child in &self.0 {
            fmt_sitemap_item(f, child, "")?;
        }

        Ok(())
    }
}

fn fmt_sitemap_item(f: &mut fmt::Formatter<'_>, item: &SitemapItem, prefix: &str) -> fmt::Result {
    write!(f, "{}- {}: {}\n", prefix, item.item.document_name, item.item.title)?;
    for child in &item.children {
        fmt_sitemap_item(f, child, &(prefix.to_owned() + "\t"))?;
    }

    Ok(())
}
