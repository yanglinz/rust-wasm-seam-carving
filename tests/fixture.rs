use image::{ColorType, GenericImageView};
use std::path::{Path, PathBuf};

pub struct ImageDimension {
    pub width: u32,
    pub height: u32,
}

pub fn get_fixture_image(image_name: &str) -> (ImageDimension, Vec<u8>) {
    let path: PathBuf = [
        Path::new(file!()).parent().expect("no directory"),
        Path::new("fixtures"),
        Path::new(image_name),
    ]
    .iter()
    .collect();

    let img = image::open(path).expect("input image not found");
    let (width, height) = img.dimensions();
    let dimensions = ImageDimension {
        width,
        height,
    };
    let rgba = img.into_rgba8().into_vec();

    (dimensions, rgba)
}

pub fn save_fixture_image(image_name: &str, width: u32, height: u32, image_data: Vec<u8>) {
    let path: PathBuf = [
        Path::new(file!()).parent().expect("no directory"),
        Path::new("fixtures"),
        Path::new(image_name),
    ]
    .iter()
    .collect();

    image::save_buffer(path, &image_data, width, height, ColorType::Rgba8).unwrap();
}
