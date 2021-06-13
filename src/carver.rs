use web_sys::ImageData;

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

fn get_energy(
    pixel: ImagePixel,
    pixel_left: Option<ImagePixel>,
    pixel_right: Option<ImagePixel>,
) -> f32 {
    let p_r = pixel.r as f32;
    let p_g = pixel.g as f32;
    let p_b = pixel.b as f32;

    let mut left_energy = 0.0;
    for l in pixel_left.iter() {
        let l_r = l.r as f32;
        let l_g = l.g as f32;
        let l_b = l.b as f32;
        left_energy = (p_r - l_r).powi(2) + (p_g - l_g).powi(2) + (p_b - l_b).powi(2)
    }

    let mut right_energy = 0.0;
    for r in pixel_right.iter() {
        let r_r = r.r as f32;
        let r_g = r.g as f32;
        let r_b = r.b as f32;
        right_energy = (p_r - r_r).powi(2) + (p_g - r_g).powi(2) + (p_b - r_b).powi(2)
    }

    return (left_energy + right_energy).sqrt();
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

pub fn get_resized_image_data(
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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_pixel(r: u8, g: u8, b: u8) -> ImagePixel {
        return ImagePixel {
            r: r,
            g: g,
            b: b,
            a: 255,
            energy: -1,
            status: PixelState::Live,
        };
    }

    #[test]
    fn test_get_pixel_energy_same_pixel() {
        let energy = get_energy(
            get_pixel(0, 0, 0),
            Some(get_pixel(0, 0, 0)),
            Some(get_pixel(0, 0, 0)),
        );
        assert_eq!(energy, 0.0);
    }

    #[test]
    fn test_get_pixel_energy_nones() {
        let energy = get_energy(get_pixel(0, 0, 0), None, None);
        assert_eq!(energy, 0.0);
    }

    #[test]
    fn test_get_pixel_energy_real() {
        let energy = get_energy(
            get_pixel(0, 0, 255),
            Some(get_pixel(0, 128, 0)),
            Some(get_pixel(255, 0, 0)),
        );
        assert_eq!(energy, 459.8467);
    }
}
