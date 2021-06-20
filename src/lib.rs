use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

mod carver;

#[wasm_bindgen]
pub fn resize(
    ctx: &CanvasRenderingContext2d,
    ctx2: &CanvasRenderingContext2d,
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
    let mut image_data = carver::get_resized_image_data(
        image_data_current,
        width_current,
        height_current,
        width_target,
        height_target,
    );
    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut image_data),
        width_target,
        height_target,
    )?;

    ctx2.put_image_data(&data, 0.0, 0.0)
}
