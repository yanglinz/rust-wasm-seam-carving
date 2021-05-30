use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

fn get_new_image_data() -> Vec<u8> {
    let mut data = Vec::new();

    for x in 0..180000 {
        let i: u8 = 100;
        data.push(i);
    }

    return data;
}

pub fn resize(ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
    let width = 300;
    let height = 150;

    let mut image_data = get_new_image_data();
    let foo = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut image_data), width, height)?;
    ctx.put_image_data(&foo, 0.0, 0.0)
}
