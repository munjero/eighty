use std::{path::Path, sync::Arc};
use crate::Error;
use crate::store::{RenderedStore};
use crate::workspace::MetadatadWorkspace;

pub fn build(root_path: &Path) -> Result<(), Error> {
    let site_metadata_store = Arc::new(MetadatadWorkspace::new(root_path)?);
    let rendered_store = Arc::new(RenderedStore::new(&site_metadata_store)?);

    Ok(())
}
