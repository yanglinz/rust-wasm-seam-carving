use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

mod carver;

#[wasm_bindgen]
pub fn resize(
    ctx: &CanvasRenderingContext2d,
    width_current: u32,
    height_current: u32,
    width_target: u32,
    height_target: u32,
) -> Result<(), JsValue> {
    return carver::resize(
        ctx,
        width_current,
        height_current,
        width_target,
        height_target,
    );
}
