use eighty::{
    workspace::{MetadatadWorkspace, RenderedWorkspace},
    Error,
};
use std::{path::Path, sync::Arc};

pub fn build(root_path: &Path) -> Result<(), Error> {
    let site_metadata_store = Arc::new(MetadatadWorkspace::new(root_path)?);
    let rendered_store = Arc::new(RenderedWorkspace::new(&site_metadata_store)?);

    Ok(())
}
