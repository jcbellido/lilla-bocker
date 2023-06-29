use std::path::PathBuf;

use crate::fs::get_files_from_dir;
use anyhow::Result;
use image::GenericImageView;

/// Invoked as part of the image generation
pub fn generate_miniatures(sources: &PathBuf, target: &PathBuf) -> Result<()> {
    tracing::info!("Starting miniatures");
    tracing::info!("Listing images under: `{:#?}`", sources);
    let existing_images = get_files_from_dir(sources.clone(), |de_path| {
        let Some(ext) = de_path.extension() else {
                return false;
            };
        ext == crate::generator_constants::IMAGES_EXT
    })?;

    // TODO Add rayon and go wide with this :)
    if existing_images.is_empty() {
        tracing::warn!("No images found to generate miniatures of");
    }

    for pb_img in existing_images {
        build_miniature(pb_img, target)?;
    }

    Ok(())
}

fn build_miniature(img_path: PathBuf, target_dir: &PathBuf) -> Result<()> {
    tracing::debug!("Creating miniature for: `{:#?}`", img_path);
    let img = image::open(&img_path)?;

    let dimensions = img.dimensions();
    let width = dimensions.0 / 4;
    let height = dimensions.1 / 4;

    let resized = img.resize(width, height, image::imageops::FilterType::CatmullRom);

    let Some(image_filename) = img_path.components().last() else {
        anyhow::bail!("Can't extract last component of path: `{:#?}`", img_path);
    };

    let target_filename = target_dir.join(image_filename);

    resized.save(target_filename)?;

    Ok(())
}
