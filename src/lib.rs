use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

mod carver;

#[wasm_bindgen]
pub fn resize(ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
    return carver::resize(ctx);
}
