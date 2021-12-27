use crate::document::DocumentName;
use std::fmt;

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct LocalSitemap {
    pub breadcrumb: Breadcrumb,
    pub children: Vec<BreadcrumbItem>,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Breadcrumb {
    pub items: Vec<BreadcrumbItem>
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct BreadcrumbItem {
    pub title: String,
    pub document_name: DocumentName,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SitemapItem {
    pub title: String,
    pub document_name: DocumentName,
    pub children: Vec<SitemapItem>,
}

impl SitemapItem {
    pub fn maybe_insert(&mut self, document_name: DocumentName, title: String) -> bool {
        if self.document_name.is_ancestor_of(&document_name) {
            let mut inserted = false;

            for child in &mut self.children {
                if child.maybe_insert(document_name.clone(), title.clone()) {
                    inserted = true;
                    break
                }
            }

            if !inserted {
                self.children.push(SitemapItem { title, document_name, children: Vec::new() });
            }

            true
        } else {
            false
        }
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Sitemap {
    pub items: Vec<SitemapItem>,
}

impl Sitemap {
    pub fn insert(&mut self, document_name: DocumentName, title: String) {
        let mut inserted = false;

        for child in &mut self.items {
            if child.maybe_insert(document_name.clone(), title.clone()) {
                inserted = true;
                break
            }
        }

        if !inserted {
            self.items.push(SitemapItem { title, document_name, children: Vec::new() });
        }
    }

    pub fn local(&self, document_name: &DocumentName) -> Option<LocalSitemap> {
        if document_name.is_root() {
            return Some(LocalSitemap {
                breadcrumb: Breadcrumb { items: Vec::new() },
                children: self.items.iter().map(|item| BreadcrumbItem {
                    title: item.title.clone(),
                    document_name: item.document_name.clone(),
                }).collect(),
            });
        }

        let mut breadcrumb = Vec::new();
        let mut current = &self.items;

        loop {
            let target = current.iter().find(|item| {
                item.document_name.is_ancestor_of(&document_name) || item.document_name == *document_name
            });

            if let Some(target) = target {
                if target.document_name == *document_name {
                    return Some(LocalSitemap {
                        breadcrumb: Breadcrumb { items: breadcrumb },
                        children: target.children.iter().map(|item| BreadcrumbItem {
                            title: item.title.clone(),
                            document_name: item.document_name.clone(),
                        }).collect(),
                    });
                } else {
                    breadcrumb.push(BreadcrumbItem {
                        title: target.title.clone(),
                        document_name: target.document_name.clone(),
                    });

                    current = &target.children;
                }
            } else {
                return None
            }
        }
    }
}

impl fmt::Display for Sitemap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for child in &self.items {
            fmt_sitemap_item(f, child, "")?;
        }

        Ok(())
    }
}

fn fmt_sitemap_item(f: &mut fmt::Formatter<'_>, item: &SitemapItem, prefix: &str) -> fmt::Result {
    write!(f, "{}- {}: {}\n", prefix, item.document_name, item.title)?;
    for child in &item.children {
        fmt_sitemap_item(f, child, &(prefix.to_owned() + "\t"))?;
    }

    Ok(())
}
