use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorRange(u8);

#[wasm_bindgen]
pub struct SeamCarver {
    width: u32,
    height: u32,
    image_data: Vec<u8>,
    // image_metadata
}

#[wasm_bindgen]
impl SeamCarver {
    pub fn new(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> SeamCarver {
        log!("width: {}", width);
        log!("height: {}", height);

        SeamCarver {
            width: 64,
            height: 64,
            image_data: vec![41, 155, 16, 245],
        }
    }

    fn assert_invariant(&mut self) {}

    pub fn mark_seam(&mut self) {
        log!("mark_seam")
    }

    pub fn delete_seam(&mut self) {
        log!("delete_seam")
    }

    pub fn image_data_ptr(&self) -> *const u8 {
        self.image_data.as_ptr()
    }
}

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}
