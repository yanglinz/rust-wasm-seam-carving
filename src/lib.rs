use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

extern crate console_error_panic_hook;
use std::panic;

// Uncomment to debug with console.log
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

#[wasm_bindgen]
pub struct SeamCarver {
    pub width: u32,
    pub height: u32,
    image_data: Vec<u8>,
    image_matrix: Vec<seam_carving::ImagePixel>,
}

#[wasm_bindgen]
impl SeamCarver {
    pub fn from_canvas(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> SeamCarver {
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        let image_data: Vec<u8> = ctx
            .get_image_data(0.0, 0.0, width as f64, height as f64)
            .unwrap()
            .data()
            .iter()
            .copied()
            .collect();
        let context = seam_carving::ImageContext { width, height };
        let image_matrix = seam_carving::get_image_pixel_matrix(context, image_data.clone());

        SeamCarver {
            width,
            height,
            image_data,
            image_matrix,
        }
    }

    pub fn from_vec(image_data: Vec<u8>, width: u32, height: u32) -> SeamCarver {
        let context = seam_carving::ImageContext { width, height };
        let image_matrix = seam_carving::get_image_pixel_matrix(context, image_data.clone());

        SeamCarver {
            width,
            height,
            image_data,
            image_matrix,
        }
    }

    pub fn mark_seam(&mut self) {
        let context = seam_carving::ImageContext {
            width: self.width,
            height: self.height,
        };
        seam_carving::mark_pixel_position(context, &mut self.image_matrix);
        seam_carving::mark_energy_map(context, &mut self.image_matrix);
        seam_carving::mark_seam_energy_map(context, &mut self.image_matrix);
        seam_carving::mark_seam(context, &mut self.image_matrix);

        self.image_data = seam_carving::get_image_data_from_pixels(context, &mut self.image_matrix);
    }

    pub fn delete_seam(&mut self) {
        seam_carving::remove_seam(
            seam_carving::ImageContext {
                width: self.width,
                height: self.height,
            },
            &mut self.image_matrix,
        );

        self.image_data = seam_carving::get_image_data_from_pixels(
            seam_carving::ImageContext {
                width: self.width - 1,
                height: self.height,
            },
            &mut self.image_matrix,
        );
        self.width -= 1;
    }

    pub fn image_data_ptr(&self) -> *const u8 {
        self.image_data.as_ptr()
    }

    pub fn image_data_vec(&self) -> Vec<u8> {
        self.image_data.clone()
    }
}

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}
