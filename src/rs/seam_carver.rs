use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

extern crate web_sys;

#[derive(Copy, Clone, PartialEq)]
enum PixelState {
    Live,
    Seam,
    Dead,
}

#[derive(Copy, Clone)]
struct ImagePixel {
    // Color representation
    r: u8,
    g: u8,
    b: u8,
    a: u8,

    // Metadata
    energy: i32,
    status: PixelState,
}

fn mark_energy_map(image_pixel_matrix: &mut Vec<Vec<ImagePixel>>) {
    for (h, row) in image_pixel_matrix.iter_mut().enumerate() {
        for (w, pixel) in row.iter_mut().enumerate() {
            // TODO: Calculate the actual energy
            pixel.energy = 10
        }
    }
}

fn mark_seam(image_pixel_matrix: &mut Vec<Vec<ImagePixel>>) {
    fn mark_seam_row(image_pixel_row: &mut Vec<ImagePixel>) -> u8 {
        for p in image_pixel_row {
            // TODO: Use real real seam algo
            if p.energy == 10 {
                p.status = PixelState::Seam;
                return 0;
            }
        }

        return 0;
    }

    for row in image_pixel_matrix {
        mark_seam_row(row);
    }
}

fn remove_seam(image_pixel_matrix: &mut Vec<Vec<ImagePixel>>) {
    fn remove_seam_row(image_pixel_row: &mut Vec<ImagePixel>) -> u8 {
        // Shift everything to last column
        for p in image_pixel_row {
            if p.energy == 10 {
                p.status = PixelState::Seam;
                return 0;
            }
        }

        return 0;
    }

    for row in image_pixel_matrix {
        remove_seam_row(row);
    }
}

fn get_image_pixel_matrix(
    image_data: ImageData,
    width_current: u32,
    height_current: u32,
) -> Vec<Vec<ImagePixel>> {
    let w_matrix = width_current as usize;
    let h_matrix = height_current as usize;
    let placeholder = ImagePixel {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
        energy: -1,
        status: PixelState::Live,
    };
    let mut matrix = vec![vec![placeholder; w_matrix]; h_matrix];

    let data = image_data.data();
    for w in 0..width_current {
        for h in 0..height_current {
            let start = (h * width_current + w) * 4;
            let start_index = start as usize;

            let r = data[start_index + 0];
            let g = data[start_index + 1];
            let b = data[start_index + 2];
            let a = data[start_index + 3];
            let pixel = ImagePixel {
                r: r,
                g: g,
                b: b,
                a: a,
                energy: -1,
                status: PixelState::Live,
            };
            matrix[h as usize][w as usize] = pixel;
        }
    }

    return matrix;
}

fn get_resized_image_data(
    image_data: ImageData,
    width_current: u32,
    height_current: u32,
    width_target: u32,
    height_target: u32,
) -> Vec<u8> {
    let mut matrix = get_image_pixel_matrix(image_data, width_current, height_current);
    let steps = width_current - width_target;
    for _ in 0..steps {
        mark_energy_map(&mut matrix);
        mark_seam(&mut matrix);
        remove_seam(&mut matrix);
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
