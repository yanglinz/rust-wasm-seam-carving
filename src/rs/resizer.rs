use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

extern crate web_sys;

fn get_resized_image_data(
    image_data: ImageData,
    width_current: u32,
    height_current: u32,
    width_target: u32,
    height_target: u32,
) -> Vec<u8> {
    let mut matrix = rs::seam_carver::get_image_pixel_matrix(image_data, width_current, height_current);
    let steps = width_current - width_target;
    for _ in 0..steps {
        crate::rs::mark_energy_map(&mut matrix);
        crate::seam_carver::mark_seam(&mut matrix);
        crate::seam_carver::remove_seam(&mut matrix);
    }

    let mut data = Vec::new();
    for h in 0..height_current {
        for w in 0..width_current {
            // TODO: Get the real values from pixel
            let pixel = matrix[h as usize][w as usize];
            data.push(5);
            data.push(21);
            data.push(23);
            data.push(255);
        }
    }

    return data;
}

pub fn resize(
    ctx: &CanvasRenderingContext2d,
    width_current: u32,
    height_current: u32,
    width_target: u32,
    height_target: u32,
) -> Result<(), JsValue> {
    let width = width_current;
    let height = height_current;
    let width_select = width_current as f64;
    let height_select = height_current as f64;

    let image_data_current = ctx.get_image_data(0.0, 0.0, width_select, height_select)?;
    let mut image_data = get_resized_image_data(
        image_data_current,
        width_current,
        height_current,
        width_target,
        height_target,
    );
    let data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut image_data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}
