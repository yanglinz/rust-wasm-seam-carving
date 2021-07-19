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
    pub width: u32,
    pub height: u32,
    image_data: Vec<u8>,
    // image_metadata
}

#[wasm_bindgen]
impl SeamCarver {
    pub fn new(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> SeamCarver {
        // Find a more consice way to create the vector
        let mut image_data: Vec<u8> = vec![];
        for d in ctx
            .get_image_data(0.0, 0.0, width as f64, height as f64)
            .unwrap()
            .data()
            .iter()
        {
            image_data.push(*d);
        }

        SeamCarver {
            width: width,
            height: height,
            image_data: image_data,
        }
    }

    fn assert_invariant(&mut self) {}

    pub fn mark_seam(&mut self) {
        // Do nothing for now
        log!("mark_seam")
    }

    pub fn delete_seam(&mut self) {
        // Randomly delete things for now
        self.width -= 1;

        // Remove width * 4 elements

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
