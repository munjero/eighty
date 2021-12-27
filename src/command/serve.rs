use std::{path::Path, sync::Arc};
use crate::Error;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use crate::store::{RenderedStore, SiteMetadataStore};

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
pub async fn serve(root_path: &Path) -> Result<(), Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    let (site_metadata_store, rendered_store) = build(root_path).await?;

    println!("listening on port 8000");
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

async fn build(root_path: &Path) -> Result<(Arc<SiteMetadataStore>, Arc<RenderedStore>), Error> {
    let root_path = root_path.to_owned();

    let (site_metadata_store, rendered_store) = tokio::task::spawn_blocking(move || -> Result<_, Error> {
        let site_metadata_store = Arc::new(SiteMetadataStore::new(&root_path)?);
        let rendered_store = Arc::new(RenderedStore::new(site_metadata_store.clone())?);

        Ok((site_metadata_store, rendered_store))
    }).await??;

    Ok((site_metadata_store, rendered_store))
}
