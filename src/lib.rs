use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

mod carver;

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
    image_matrix: Vec<carver::ImagePixel>,
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

        let context = carver::ImageContext {
            width: width,
            height: height,
        };
        let image_matrix = carver::get_image_pixel_matrix(context, image_data);

        SeamCarver {
            width: width,
            height: height,
            image_data: image_data,
            image_matrix: image_matrix,
        }
    }

    fn assert_invariant(&mut self) {}

    pub fn mark_seam(&mut self) {
        // Do nothing for now
        log!("mark_seam")
    }

    pub fn delete_seam(&mut self) {
        log!("delete_seam");

        // Randomly delete things for now
        self.image_data.drain(0..self.width as usize); // Remove the first n elements
        self.width -= 1;
    }

    pub fn image_data_ptr(&self) -> *const u8 {
        self.image_data.as_ptr()
    }
}

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}
