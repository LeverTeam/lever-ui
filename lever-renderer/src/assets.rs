use image::GenericImageView;
use std::path::Path;

pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

pub fn load_image_from_bytes(bytes: &[u8]) -> Result<ImageData, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(bytes)?;
    let (width, height) = img.dimensions();
    let rgba = img.to_rgba8().into_raw();

    Ok(ImageData {
        width,
        height,
        rgba,
    })
}

pub fn load_image_from_path(
    path: impl AsRef<Path>,
) -> Result<ImageData, Box<dyn std::error::Error>> {
    let img = image::open(path)?;
    let (width, height) = img.dimensions();
    let rgba = img.to_rgba8().into_raw();

    Ok(ImageData {
        width,
        height,
        rgba,
    })
}
