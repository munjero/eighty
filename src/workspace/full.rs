use crate::{
    asset::AssetStore,
    document::{DocumentMetadata, DocumentName, RenderedData, Spec},
    file::FileMetadata,
    site::{SiteMetadata, SiteName},
    sitemap::{LocalSitemap, Sitemap},
    workspace::{RenderedSite, RenderedWorkspace},
    layout,
    Error,
};
use handlebars::Handlebars;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub struct FullWorkspace {
    pub root_path: PathBuf,
    pub assets: AssetStore,
    pub sites: HashMap<SiteName, FullSite>,
    pub spec_site: FullSpecSite,
}

impl FullWorkspace {
    pub fn new(rendered: &RenderedWorkspace) -> Result<Self, Error> {
        let assets = AssetStore::new(&rendered.root_path)?;

        let sites = rendered
            .sites
            .iter()
            .map(|(name, site)| Ok((name.clone(), FullSite::new(&site, &assets.handlebars)?)))
            .collect::<Result<HashMap<SiteName, FullSite>, Error>>()?;

        let mut specs = Vec::new();
        for (_, site) in &sites {
            for (_, document) in &site.documents {
                for spec in &document.rendered.specs {
                    specs.push(spec.clone());
                }
            }
        }

        let spec_site = FullSpecSite::new(specs, &assets.handlebars)?;

        Ok(Self {
            root_path: rendered.root_path.clone(),
            assets,
            sites,
            spec_site,
        })
    }
}

pub struct FullSite {
    pub site: Arc<SiteMetadata>,
    pub documents: HashMap<DocumentName, FullDocument>,
    pub files: Arc<HashMap<PathBuf, FileMetadata>>,
    pub sitemap: Sitemap,
}

impl FullSite {
    pub fn new(rendered: &RenderedSite, handlebars: &Handlebars) -> Result<Self, Error> {
        let name_titles = rendered
            .documents
            .iter()
            .map(|(k, v)| (k.clone(), v.data.title.clone()))
            .collect::<Vec<_>>();

        let sitemap = Sitemap::from(name_titles.clone());

        let full_documents = rendered
            .documents
            .iter()
            .map(|(k, v)| {
                Ok((
                    k.clone(),
                    FullDocument {
                        site_metadata: v.site_metadata.clone(),
                        metadata: v.metadata.clone(),
                        rendered: v.data.clone(),
                        content: layout::document(&v, &sitemap, handlebars)?,
                        local_sitemap: sitemap.local(&k).ok_or(Error::DocumentNotFound)?,
                    },
                ))
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self {
            site: rendered.site.clone(),
            documents: full_documents,
            files: rendered.files.clone(),
            sitemap,
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct FullDocument {
    pub site_metadata: Arc<SiteMetadata>,
    pub metadata: Arc<DocumentMetadata>,
    pub rendered: Arc<RenderedData>,
    pub content: String,
    pub local_sitemap: LocalSitemap,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct FullSpecSite {
    pub specs: HashMap<String, FullSpec>,
    pub index_content: String,
}

impl FullSpecSite {
    pub fn new(specs: Vec<Spec>, handlebars: &Handlebars) -> Result<Self, Error> {
        let index_content = layout::spec_index(&specs, handlebars)?;
        let mut full_specs = HashMap::new();

        for spec in specs {
            let full_spec = FullSpec::new(spec, handlebars)?;
            full_specs.insert(full_spec.data.id.clone(), full_spec);
        }

        Ok(FullSpecSite {
            specs: full_specs,
            index_content,
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct FullSpec {
    pub data: Spec,
    pub redirect_content: String,
    pub path: PathBuf,
    pub folder_path: PathBuf,
}

impl FullSpec {
    pub fn new(spec: Spec, handlebars: &Handlebars) -> Result<Self, Error> {
        let redirect_content = layout::spec_redirect(&spec, handlebars)?;
        let folder_path = {
            let mut path = PathBuf::new();
            path.push(spec.id.clone());
            path
        };
        let path = {
            let mut path = PathBuf::new();
            path.push(spec.id.clone());
            path.push("index.html");
            path
        };

        Ok(FullSpec {
            data: spec,
            redirect_content,
            path,
            folder_path,
        })
    }
}
