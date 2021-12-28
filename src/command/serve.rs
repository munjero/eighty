use std::{path::Path, sync::Arc};
use crate::Error;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use crate::store::{LayoutedStore, SitemapStore};
use crate::workspace::{MetadatadWorkspace, RenderedWorkspace};
use crate::site::SiteName;
use crate::asset::AssetStore;

pub struct Context {
    pub metadata: Arc<MetadatadWorkspace>,
    pub rendered: Arc<RenderedWorkspace>,
    pub assets: Arc<AssetStore>,
    pub layouted: Arc<LayoutedStore>,
    pub sitemaps: Arc<SitemapStore>,
    pub site_name: SiteName,
}

async fn handle(req: Request<Body>, context: Arc<Context>) -> Result<Response<Body>, Error> {
    let site = context.metadata.get(&context.site_name).ok_or(Error::SiteNotExist)?;
    let layouted = context.layouted.documents.get(&context.site_name).ok_or(Error::SiteNotExist)?;

    let uri_path = Path::new(req.uri().path());
    let rel_path = uri_path.strip_prefix(&site.site.config.base_url)?;

    if let Some(document_name) = layouted.documents.keys().find(|item| item.is_matched(&rel_path)) {
        let document = layouted.documents.get(&document_name).ok_or(Error::DocumentNotFound)?;

        return Ok(Response::builder()
                  .header("Content-Type", "text/html")
                  .body(document.content.clone().into())?)
    }

    if let Some(asset_content) = context.assets.assets.get(rel_path) {
        return Ok(Response::builder()
                  .body(asset_content.clone().into())?)
    }

    return Err(Error::DocumentNotFound)
}

#[tokio::main]
pub async fn serve(root_path: &Path, site_name: &str) -> Result<(), Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let context = Arc::new(build(root_path, SiteName(site_name.to_string())).await?);

    let make_svc = make_service_fn(move |_conn| {
        let context = context.clone();

        async move {
            Ok::<_, Error>(service_fn(move |req| handle(req, context.clone())))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("listening on port 8000");
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

async fn build(root_path: &Path, site_name: SiteName) -> Result<Context, Error> {
    let root_path = root_path.to_owned();

    let context = tokio::task::spawn_blocking(move || -> Result<_, Error> {
        let site_metadata_store = Arc::new(MetadatadWorkspace::new(&root_path)?);
        let rendered_store = Arc::new(RenderedWorkspace::new(&site_metadata_store)?);
        let asset_store = Arc::new(AssetStore::new(&root_path)?);
        let sitemap_store = Arc::new(SitemapStore::new(&rendered_store)?);
        let layouted_store = Arc::new(LayoutedStore::new(&rendered_store, &sitemap_store, &asset_store)?);

        let context = Context {
            metadata: site_metadata_store,
            rendered: rendered_store,
            assets: asset_store,
            layouted: layouted_store,
            sitemaps: sitemap_store,
            site_name,
        };

        Ok(context)
    }).await??;

    Ok(context)
}
