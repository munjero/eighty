use crate::{
    site::SiteName,
    workspace::{FullWorkspace, MetadatadWorkspace, RenderedWorkspace, SimplePostWorkspace},
    Error,
};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use std::{net::SocketAddr, path::Path, sync::Arc};

pub struct Context {
    pub metadatad: MetadatadWorkspace,
    pub rendered: RenderedWorkspace,
    pub full: FullWorkspace,
    pub post: SimplePostWorkspace,
    pub site_name: SiteName,
}

async fn handle(req: Request<Body>, context: Arc<Context>) -> Result<Response<Body>, Error> {
    let site = context
        .post
        .get(&context.site_name)
        .ok_or(Error::SiteNotExist)?;

    let uri_path = Path::new(req.uri().path());
    let rel_path = uri_path.strip_prefix(&site.site.config.base_url)?;
    let index_rel_path = rel_path.join("index.html");

    let content = site.files.get(&rel_path.to_owned()).map(|p| (rel_path, p))
        .or(site.files.get(&index_rel_path.to_owned()).map(|p| (index_rel_path.as_ref(), p)));

    if let Some((content_path, content)) = content {
        let mut response = Response::builder();

        if content_path.extension().and_then(|v| v.to_str()) == Some("html") {
            response = response.header("Content-Type", "text/html");
        }

        return Ok(response.body(content.clone().into())?);
    } else {
        return Ok(Response::builder().status(StatusCode::NOT_FOUND).body("Not found".into())?);
    }
}

#[tokio::main]
pub async fn serve(root_path: &Path, site_name: &str) -> Result<(), Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let context = Arc::new(build(root_path, SiteName(site_name.to_string())).await?);

    let make_svc = make_service_fn(move |_conn| {
        let context = context.clone();

        async move { Ok::<_, Error>(service_fn(move |req| handle(req, context.clone()))) }
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
        let metadatad = MetadatadWorkspace::new(&root_path)?;
        let rendered = RenderedWorkspace::new(&metadatad)?;
        let full = FullWorkspace::new(&rendered)?;
        let post = SimplePostWorkspace::new(&full)?;

        let context = Context {
            metadatad,
            rendered,
            full,
            post,
            site_name,
        };

        Ok(context)
    })
    .await??;

    Ok(context)
}
