use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};

extern crate console_error_panic_hook;
use std::panic;

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
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        // Find a more consice way to create the vector
        let mut image_data: Vec<u8> = vec![];
        let mut image_data_copy: Vec<u8> = vec![];
        for d in ctx
            .get_image_data(0.0, 0.0, width as f64, height as f64)
            .unwrap()
            .data()
            .iter()
        {
            image_data.push(*d);
            image_data_copy.push(*d);
        }

        let context = carver::ImageContext {
            width: width,
            height: height,
        };
        let image_matrix = carver::get_image_pixel_matrix(context, image_data_copy);

        SeamCarver {
            width: width,
            height: height,
            image_data: image_data,
            image_matrix: image_matrix,
        }
    }

    fn assert_invariant(&mut self) {}

    pub fn mark_seam(&mut self) {
        let context = carver::ImageContext {
            width: self.width,
            height: self.height,
        };
        carver::mark_pixel_position(context, &mut self.image_matrix);
        carver::mark_energy_map(context, &mut self.image_matrix);
        carver::mark_seam_energy_map(context, &mut self.image_matrix);
        carver::mark_seam(context, &mut self.image_matrix);
    }

    pub fn delete_seam(&mut self) {
        let context = carver::ImageContext {
            width: self.width,
            height: self.height,
        };
        carver::remove_seam(context, &mut self.image_matrix);

        // Randomly delete things for now
        self.image_data.drain(0..self.width as usize); // Remove the first n elements

        let new_context = carver::ImageContext {
            width: self.width - 1,
            height: self.height,
        };
        let new_image_data =
            carver::get_image_data_from_pixels(new_context, &mut self.image_matrix);

        self.image_data = new_image_data;

        // log!("image_data: {}", self.image_data.len());
        // log!("new_image_data: {}", new_image_data.len());

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
