use eighty::{
    workspace::{MetadatadWorkspace, RenderedWorkspace, FullWorkspace, SimplePostWorkspace},
    Error,
};
use std::{path::Path, fs};

pub fn build(root_path: &Path, target_path: &Path) -> Result<(), Error> {
    let metadatad = MetadatadWorkspace::new(&root_path)?;
    let rendered = RenderedWorkspace::new(&metadatad)?;
    let full = FullWorkspace::new(&rendered)?;
    let post = SimplePostWorkspace::new(&full)?;

    for (site_name, site) in &post.0 {
        let site_target_path = target_path.join(&site_name.0);

        for (file_path, file) in &site.files {
            let file_target_path = site_target_path.join(file_path);

            fs::create_dir_all(file_target_path.parent().ok_or(Error::InvalidPathComponent)?)?;
            fs::write(file_target_path, &file[..])?;
        }
    }

    Ok(())
}
