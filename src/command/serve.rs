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

use eighty::{
    site::SiteName,
    workspace::{FullWorkspace, MetadatadWorkspace, RenderedWorkspace, SimplePostWorkspace},
    Error,
};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    fs,
    net::SocketAddr,
    path::Path,
    sync::{mpsc::channel, Arc, RwLock},
    thread,
    time::Duration,
};

pub struct Context {
    pub metadatad: MetadatadWorkspace,
    pub rendered: RenderedWorkspace,
    pub full: FullWorkspace,
    pub post: SimplePostWorkspace,
    pub site_name: SiteName,
}

async fn handle(
    req: Request<Body>,
    context: Arc<RwLock<Context>>,
) -> Result<Response<Body>, Error> {
    let context = context.read()?;
    let site = context
        .post
        .get(&context.site_name)
        .ok_or(Error::SiteNotExist)?;

    let uri_path = Path::new(req.uri().path());
    let rel_path = uri_path.strip_prefix(&site.base_url)?;
    let index_rel_path = rel_path.join("index.html");

    let content = site
        .files
        .get(&rel_path.to_owned())
        .map(|p| (rel_path, p))
        .or(site
            .files
            .get(&index_rel_path.to_owned())
            .map(|p| (index_rel_path.as_ref(), p)));

    if let Some((content_path, content)) = content {
        let mut response = Response::builder();

        if content_path.extension().and_then(|v| v.to_str()) == Some("html") {
            response = response.header("Content-Type", "text/html");
        }

        return Ok(response.body(content.clone().into())?);
    } else {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not found".into())?);
    }
}

#[tokio::main]
pub async fn serve(root_path: &Path, site_name: &str) -> Result<(), Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let context = Arc::new(RwLock::new(
        async_build(root_path, SiteName(site_name.to_string())).await?,
    ));

    let root_path = root_path.to_owned();
    let site_name = site_name.to_owned();
    let watch_context = context.clone();

    thread::spawn(move || {
        let watching = || -> Result<(), Error> {
            let (tx, rx) = channel();

            let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
            watcher.watch(root_path.clone(), RecursiveMode::Recursive)?;

            loop {
                match rx.recv() {
                    Ok(event) => {
                        let should_rebuild = match event {
                            DebouncedEvent::NoticeWrite(path)
                            | DebouncedEvent::NoticeRemove(path)
                            | DebouncedEvent::Create(path)
                            | DebouncedEvent::Write(path)
                            | DebouncedEvent::Chmod(path)
                            | DebouncedEvent::Remove(path) => {
                                should_rebuild_for_path(&path, &root_path)?
                            }
                            DebouncedEvent::Rename(p1, p2) => {
                                should_rebuild_for_path(&p1, &root_path)?
                                    || should_rebuild_for_path(&p2, &root_path)?
                            }
                            DebouncedEvent::Rescan => true,
                            DebouncedEvent::Error(err, _) => return Err(Error::Notify(err)),
                        };

                        if should_rebuild {
                            let mut context = watch_context.write()?;
                            *context =
                                build(&root_path, SiteName(site_name.to_string()), Some(&context))?;

                            println!("[workspace] rebuilt after source folder changes");
                        }
                    }
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        };

        match watching() {
            Ok(()) => println!("watching thread returned"),
            Err(e) => println!("watching thread error: {:?}", e),
        }
    });

    let make_svc = make_service_fn(move |_conn| {
        let context = context.clone();

        async move { Ok::<_, Error>(service_fn(move |req| handle(req, context.clone()))) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("[server] listening on port 8000");
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

async fn async_build(root_path: &Path, site_name: SiteName) -> Result<Context, Error> {
    let root_path = root_path.to_owned();

    let context = tokio::task::spawn_blocking(move || -> Result<_, Error> {
        build(&root_path, site_name, None)
    })
    .await??;

    Ok(context)
}

fn build(root_path: &Path, site_name: SiteName, old: Option<&Context>) -> Result<Context, Error> {
    let metadatad = MetadatadWorkspace::new(&root_path)?;
    let rendered = if let Some(old) = old {
        RenderedWorkspace::new_with_old(&metadatad, &old.rendered)?
    } else {
        RenderedWorkspace::new(&metadatad)?
    };
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
}

fn should_rebuild_for_path(path: &Path, root_path: &Path) -> Result<bool, Error> {
    let root_path = fs::canonicalize(root_path)?;

    let rel_path = path.strip_prefix(&root_path)?;
    Ok(rel_path
        .iter()
        .all(|label| !label.to_str().map(|l| l.starts_with(".")).unwrap_or(false)))
}
