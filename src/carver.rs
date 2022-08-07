extern crate web_sys;

#[derive(Copy, Clone)]
pub struct ImageContext {
    pub width: u32,
    pub height: u32,
}

// Zero-indexed representation of image pixel coordinates.
#[derive(Copy, Clone)]
struct PixelPosition {
    x: u32,
    y: u32,
}

// Binary represenation of whether a pixel is a seam.
#[derive(Copy, Clone, PartialEq)]
enum PixelStatus {
    Live,
    Seam,
}

// Wrapper struc to represent all the state we need to track for a given pixel.
#[derive(Copy, Clone)]
pub struct ImagePixel {
    // Color representation
    r: u8,
    g: u8,
    b: u8,
    a: u8,

    // Stateful metadata
    status: PixelStatus,
    position: PixelPosition,
    energy: f32,
    seam_energy: f32,
}

// Similar to the ImageData.data from the web canvas API, we'll represent the
// 2D image internally with a 1D vector/array. This is a helper function to convert
// the (x, y) coordinates of the 2D "matrix" into the index of the 1D vector.
fn get_pixel_index(context: ImageContext, pos: PixelPosition) -> usize {
    if pos.x >= context.width || pos.y >= context.height {
        panic!("PixelPosition's x and/or y exceeds image dimensions");
    }
    let index = (context.width * pos.y) + pos.x;
    index as usize
}

// Same as the get_pixel_index helper, just with the inverse logic.
fn get_pixel_position(context: ImageContext, index: usize) -> PixelPosition {
    let x = (index as u32).rem_euclid(context.width);
    let y = index as u32 / context.width;
    PixelPosition { x, y }
}

// For the energy calculation, we need a helper to get the index of pixels that
// are adjacent (top, bottom, left, right) for a given pixel.
fn get_neighbor_pixel_index(
    context: ImageContext,
    index: usize,
    offset_x: i8,
    offset_y: i8,
) -> Option<usize> {
    let _valid_offset = [-1, 0, 1];
    if !_valid_offset.contains(&offset_x) || !_valid_offset.contains(&offset_y) {
        panic!("Received invalid offset values");
    }

    let pos = get_pixel_position(context, index);
    let will_overflow = (pos.y == 0 && offset_y == -1)
        || (pos.y == context.height - 1 && offset_y == 1)
        || (pos.x == 0 && offset_x == -1)
        || (pos.x == context.width - 1 && offset_x == 1);
    if will_overflow {
        return None;
    }

    let new_x = pos.x as i32 + offset_x as i32;
    let new_y = pos.y as i32 + offset_y as i32;
    Some(get_pixel_index(
        context,
        PixelPosition {
            x: new_x as u32,
            y: new_y as u32,
        },
    ))
}

// Wrapper for get_neighbor_pixel_index to get the actual pixel state.
fn get_neighbor_pixel(
    context: ImageContext,
    image_pixel_matrix: &[ImagePixel],
    index: usize,
    offset_x: i8,
    offset_y: i8,
) -> Option<ImagePixel> {
    let index = get_neighbor_pixel_index(context, index, offset_x, offset_y);
    match index {
        None => None,
        _ => Some(image_pixel_matrix[index.unwrap()]),
    }
}

// Helper to initialize the image pixel matrix with its proper initial state.
pub fn get_image_pixel_matrix(context: ImageContext, image_data: Vec<u8>) -> Vec<ImagePixel> {
    let w_matrix = context.width as usize;
    let h_matrix = context.height as usize;
    let placeholder = ImagePixel {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
        status: PixelStatus::Live,
        position: PixelPosition { x: 0, y: 0 },
        energy: -1.0,
        seam_energy: -1.0,
    };
    let mut matrix = vec![placeholder; w_matrix * h_matrix];

    for h in 0..h_matrix {
        for w in 0..w_matrix {
            let start = (h * w_matrix + w) * 4;
            let start_index = start as usize;
            let pos = PixelPosition {
                x: w as u32,
                y: h as u32,
            };
            let pixel = ImagePixel {
                r: image_data[start_index],
                g: image_data[start_index + 1],
                b: image_data[start_index + 2],
                a: image_data[start_index + 3],
                status: PixelStatus::Live,
                position: pos,
                energy: -1.0,
                seam_energy: -1.0,
            };
            let index = get_pixel_index(context, pos);
            matrix[index] = pixel;
        }
    }

    matrix
}

// Helper to update each pixel's position value in the image matrix vector.
pub fn mark_pixel_position(context: ImageContext, image_pixel_matrix: &mut [ImagePixel]) {
    for (i, pixel) in image_pixel_matrix.iter_mut().enumerate() {
        let pos = get_pixel_position(context, i);
        pixel.position = pos;
    }
}

// Helper to calculate the energy of a pixel - which is the core to the algorithm.
// We remove seams made up of pixels "line" of the lowest total energy.
// An individual energy is calculated as the "distance" of colors between a pixel
// and its neighbors. Higher the "distance", higher the "energy".
fn get_energy(
    pixel: ImagePixel,
    pixel_left: Option<ImagePixel>,
    pixel_right: Option<ImagePixel>,
) -> f32 {
    let p_r = pixel.r as f32;
    let p_g = pixel.g as f32;
    let p_b = pixel.b as f32;

    let left_energy = match pixel_left {
        None => 0.0,
        _ => {
            let l = pixel_left.unwrap();
            (p_r - l.r as f32).powi(2) + (p_g - l.g as f32).powi(2) + (p_b - l.b as f32).powi(2)
        }
    };
    let right_energy = match pixel_right {
        None => 0.0,
        _ => {
            let r = pixel_right.unwrap();
            (p_r - r.r as f32).powi(2) + (p_g - r.g as f32).powi(2) + (p_b - r.b as f32).powi(2)
        }
    };

    (left_energy + right_energy).sqrt()
}

// Helper to update each pixel's energy value in the image matrix vector.
pub fn mark_energy_map(context: ImageContext, image_pixel_matrix: &mut [ImagePixel]) {
    let w_matrix = context.width as usize;
    let h_matrix = context.height as usize;

    for y in 0..h_matrix {
        for x in 0..w_matrix {
            let index = get_pixel_index(
                context,
                PixelPosition {
                    x: x as u32,
                    y: y as u32,
                },
            );

            let left = get_neighbor_pixel(context, image_pixel_matrix, index, -1, 0);
            let right = get_neighbor_pixel(context, image_pixel_matrix, index, 1, 0);

            let pixel = image_pixel_matrix[index];
            image_pixel_matrix[index].energy = get_energy(pixel, left, right);
        }
    }
}

// An seam energy map is related to the energy map. The idea is that to efficiently
// calculate the seam on each iteration, we can use dynamic programming to
// pre-calculate all the lowest possible energy paths for the current iteration.
// So the seam energy map is the "sum" of the lowest energy total possible for a pixel.
pub fn mark_seam_energy_map(context: ImageContext, image_pixel_matrix: &mut [ImagePixel]) {
    let w_matrix = context.width as usize;
    let h_matrix = context.height as usize;

    for y in 0..h_matrix {
        for x in 0..w_matrix {
            let index = get_pixel_index(
                context,
                PixelPosition {
                    x: x as u32,
                    y: y as u32,
                },
            );

            if y == 0 {
                // For the first row, seam energy is just the copy of pixel energy
                image_pixel_matrix[index].seam_energy = image_pixel_matrix[index].energy;
            } else {
                // For all other rows, we have to calculate the minimum possible energy based on its top neighbors.
                let top_left = get_neighbor_pixel(context, image_pixel_matrix, index, -1, -1);
                let top = get_neighbor_pixel(context, image_pixel_matrix, index, 0, -1);
                let top_right = get_neighbor_pixel(context, image_pixel_matrix, index, 1, -1);
                let min = vec![top_left, top, top_right]
                    .iter()
                    .filter(|p| p.is_some())
                    .map(|p| p.unwrap().seam_energy)
                    .fold(f32::INFINITY, |a, b| a.min(b));
                image_pixel_matrix[index].seam_energy = image_pixel_matrix[index].energy + min;
            }
        }
    }
}

// Once the seam energy map is marked, we can determine the seam by taking the lowest
// total seam enegy of the bottom edge, and traverse to the top of the image by
// iteratively taking the lowest top-adjacent seam energy.
pub fn mark_seam(context: ImageContext, image_pixel_matrix: &mut [ImagePixel]) {
    let w_matrix = context.width as usize;
    let h_matrix = context.height as usize;

    let mut x = 0;
    let mut y = 0;
    for h in (0..h_matrix).rev() {
        let is_last = h == h_matrix - 1;
        let is_first = h == 0;
        if is_first || is_last {
            let start = h * w_matrix;
            let end = start + w_matrix;
            let last_row = &image_pixel_matrix[start..end];
            let mut min = f32::INFINITY;
            for (i, p) in last_row.iter().enumerate() {
                if p.seam_energy < min {
                    min = p.seam_energy;
                    x = i;
                    y = h;
                }
            }
        } else {
            let index = get_pixel_index(
                context,
                PixelPosition {
                    x: x as u32,
                    y: y as u32,
                },
            );
            let top_left = get_neighbor_pixel(context, image_pixel_matrix, index, -1, -1);
            let top = get_neighbor_pixel(context, image_pixel_matrix, index, 0, -1);
            let top_right = get_neighbor_pixel(context, image_pixel_matrix, index, 1, -1);
            let neighbors: Vec<ImagePixel> = vec![top_left, top, top_right]
                .iter()
                .flatten()
                .copied()
                .collect();
            let mut min = f32::INFINITY;
            for p in neighbors {
                if p.seam_energy < min {
                    min = p.seam_energy;
                    x = p.position.x as usize;
                    y = p.position.y as usize;
                }
            }
        }

        let index = get_pixel_index(
            context,
            PixelPosition {
                x: x as u32,
                y: y as u32,
            },
        );
        image_pixel_matrix[index].status = PixelStatus::Seam;
    }
}

// Finally, we can remove the seam from the vector. We'll also need to make sure to update
// our internal state of the width and height accordingly.
pub fn remove_seam(_context: ImageContext, image_pixel_matrix: &mut Vec<ImagePixel>) {
    image_pixel_matrix.retain(|p| p.status != PixelStatus::Seam);
}

pub fn get_image_data_from_pixels(
    context: ImageContext,
    image_pixel_matrix: &mut [ImagePixel],
) -> Vec<u8> {
    let mut data = Vec::new();
    for h in 0..context.height {
        for w in 0..context.width {
            let index = get_pixel_index(
                context,
                PixelPosition {
                    x: w as u32,
                    y: h as u32,
                },
            );
            let pixel = image_pixel_matrix[index];
            if pixel.status == PixelStatus::Seam {
                data.push(255);
                data.push(0);
                data.push(0);
                data.push(255);
            } else {
                data.push(pixel.r);
                data.push(pixel.g);
                data.push(pixel.b);
                data.push(pixel.a);
            }
        }
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_grayscale(gray_matrix: Vec<u8>) -> Vec<ImagePixel> {
        let mut matrix = vec![];
        for g in &gray_matrix {
            matrix.push(ImagePixel {
                r: *g,
                g: *g,
                b: *g,
                a: 255,
                status: PixelStatus::Live,
                position: PixelPosition { x: 0, y: 0 },
                energy: -1.0,
                seam_energy: -1.0,
            })
        }
        matrix
    }

    fn from_rgb(rgba_matrix: Vec<(u8, u8, u8)>) -> Vec<ImagePixel> {
        let mut matrix = vec![];
        for (r, g, b) in &rgba_matrix {
            matrix.push(ImagePixel {
                r: *r as u8,
                g: *g as u8,
                b: *b as u8,
                a: 255,
                status: PixelStatus::Live,
                position: PixelPosition { x: 0, y: 0 },
                energy: -1.0,
                seam_energy: -1.0,
            })
        }
        matrix
    }

    fn from_rgba(rgba_matrix: Vec<(u8, u8, u8, u8)>) -> Vec<ImagePixel> {
        let mut matrix = vec![];
        for (r, g, b, a) in &rgba_matrix {
            matrix.push(ImagePixel {
                r: *r as u8,
                g: *g as u8,
                b: *b as u8,
                a: *a as u8,
                status: PixelStatus::Live,
                position: PixelPosition { x: 0, y: 0 },
                energy: -1.0,
                seam_energy: -1.0,
            })
        }
        matrix
    }

    fn normalize(image_data: Vec<ImagePixel>) -> Vec<(u8, u8, u8, u8)> {
        image_data.iter().map(|p| (p.r, p.g, p.b, p.a)).collect()
    }

    #[test]
    fn test_get_pixel_index() {
        let context = ImageContext {
            width: 10,
            height: 10,
        };
        let test_cases = vec![
            // Tuple of x, y, expected_index
            (0, 0, 0),
            (1, 0, 1),
            (1, 1, 11),
            (0, 1, 10),
            (9, 9, 99),
        ];

        for (x, y, expected_index) in test_cases.iter() {
            let index = get_pixel_index(
                context,
                PixelPosition {
                    x: *x as u32,
                    y: *y as u32,
                },
            );
            assert_eq!(index, *expected_index);
        }
    }

    #[test]
    fn test_get_pixel_position() {
        let context = ImageContext {
            width: 10,
            height: 10,
        };
        let test_cases = vec![
            // Tuple of index, expected_x, expected_y
            (0, 0, 0),
            (1, 1, 0),
            (11, 1, 1),
            (10, 0, 1),
            (99, 9, 9),
        ];
        for (index, expected_x, expected_y) in test_cases.iter() {
            let pos = get_pixel_position(context, *index);
            assert_eq!(pos.x, *expected_x);
            assert_eq!(pos.y, *expected_y);
        }
    }

    #[test]
    fn test_get_pixel_index_position_mirror() {
        let context = ImageContext {
            width: 10,
            height: 10,
        };
        for i in 0..=99 {
            let pos = get_pixel_position(context, i);
            let index = get_pixel_index(context, pos);
            assert_eq!(i, index);
        }
    }

    #[test]
    fn test_get_neighbor_pixel_index() {
        let context = ImageContext {
            width: 10,
            height: 10,
        };
        let test_cases = vec![
            // Tuple of index, offset_x, offset_y, expected_index
            (0, 0, 0, Some(0)),
            (0, 1, 0, Some(1)),
            (0, 0, 1, Some(10)),
            (0, 1, 1, Some(11)),
            (9, 0, 1, Some(19)),
            (9, -1, 0, Some(8)),
            // Edge cases
            (0, -1, 0, None),
            (0, 0, -1, None),
            (9, 1, 0, None),
            (9, 1, 1, None),
            (99, 0, 1, None),
            (99, 1, 1, None),
        ];
        for (index, offset_x, offset_y, expected_index) in test_cases.iter() {
            let index = get_neighbor_pixel_index(context, *index, *offset_x, *offset_y);
            assert_eq!(index, *expected_index);
        }
    }

    #[test]
    fn test_get_neighbor_pixel_index_mirror() {
        let context = ImageContext {
            width: 10,
            height: 10,
        };
        for i in 0..=99 {
            let original = get_neighbor_pixel_index(context, i, 0, 0);
            assert_eq!(original, Some(i));

            let left = get_neighbor_pixel_index(context, i, -1, 0);
            if left.is_some() {
                let original = get_neighbor_pixel_index(context, left.unwrap(), 1, 0);
                assert_eq!(original, Some(i));
            }

            let right = get_neighbor_pixel_index(context, i, 1, 0);
            if right.is_some() {
                let original = get_neighbor_pixel_index(context, right.unwrap(), -1, 0);
                assert_eq!(original, Some(i));
            }

            let top = get_neighbor_pixel_index(context, i, 0, 1);
            if top.is_some() {
                let original = get_neighbor_pixel_index(context, top.unwrap(), 0, -1);
                assert_eq!(original, Some(i));
            }

            let bottom = get_neighbor_pixel_index(context, i, 0, -1);
            if bottom.is_some() {
                let original = get_neighbor_pixel_index(context, bottom.unwrap(), 0, 1);
                assert_eq!(original, Some(i));
            }
        }
    }

    fn get_pixel(r: u8, g: u8, b: u8) -> ImagePixel {
        ImagePixel {
            r: r,
            g: g,
            b: b,
            a: 255,
            status: PixelStatus::Live,
            position: PixelPosition { x: 0, y: 0 },
            energy: -1.0,
            seam_energy: -1.0,
        }
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

    #[test]
    fn test_get_image_pixel_matrix() {
        let context = ImageContext {
            width: 3,
            height: 3,
        };

        #[rustfmt::skip]
        let image_data = vec![
            100, 100, 100, 255,  0, 100, 0, 255,  200, 200, 0, 255,
            100, 100, 100, 255,  0, 100, 0, 255,  200, 200, 0, 255,
            100, 100, 100, 255,  0, 100, 0, 255,  200, 200, 0, 255,
        ];

        #[rustfmt::skip]
        let expected = from_rgba(vec![
            (100, 100, 100, 255), (0, 100, 0, 255), (200, 200, 0, 255),
            (100, 100, 100, 255), (0, 100, 0, 255), (200, 200, 0, 255),
            (100, 100, 100, 255), (0, 100, 0, 255), (200, 200, 0, 255),
        ]);
        let actual = get_image_pixel_matrix(context, image_data);
        assert_eq!(normalize(actual), normalize(expected));
    }

    #[test]
    fn test_mark_energy_map() {
        let context = ImageContext {
            width: 3,
            height: 3,
        };

        #[rustfmt::skip]
        let mut image_pixel_matrix = from_rgb(vec![
            (100, 100, 100), (0, 100, 0), (200, 200, 0),
            (100, 0, 0),     (0, 100, 0), (0, 100, 200),
            (100, 0, 0),     (0, 100, 0), (0, 100, 200),
        ]);

        #[rustfmt::skip]
        let expected_energy = vec![
            141.42136, 264.57513, 223.6068,
            141.42136, 244.94897, 200.0,
            141.42136, 244.94897, 200.0
        ];
        mark_energy_map(context, &mut image_pixel_matrix);
        let energy_matrix: Vec<f32> = image_pixel_matrix.iter().map(|p| p.energy).collect();
        assert_eq!(energy_matrix, expected_energy);
    }

    #[test]
    fn test_mark_seam() {
        let context = ImageContext {
            width: 5,
            height: 3,
        };

        #[rustfmt::skip]
        let mut image_pixel_matrix = from_rgb(vec![
            (100, 100, 100), (0, 100, 0), (200, 200, 0),  (0, 100, 0),  (90, 20, 0),
            (10, 0, 0),      (0, 100, 0), (0, 100, 130),  (150, 50, 0), (120, 20, 0),
            (111, 1, 4),     (0, 100, 0), (11, 50, 170),  (190, 20, 0), (110, 20, 0),
        ]);

        let expected_energy_map = vec![
            141.42136, 264.57513, 316.22775, 253.9685, 120.41595, 100.49876, 164.31677, 242.4871,
            209.04546, 42.426407, 148.78844, 231.64412, 305.55197, 261.2298, 80.0,
        ];

        #[rustfmt::skip]
        let expected_seam_energies = vec![
            141.42136, 264.57513, 316.22775, 253.9685, 120.41595,
            241.9201, 305.73813, 496.45563, 329.4614, 162.84235,
            390.70856, 473.5642, 611.2901, 424.07214, 242.84235
        ];

        mark_pixel_position(context, &mut image_pixel_matrix);
        mark_energy_map(context, &mut image_pixel_matrix);
        let energy_matrix: Vec<f32> = image_pixel_matrix.iter().map(|p| p.energy).collect();
        assert_eq!(energy_matrix, expected_energy_map);

        mark_seam_energy_map(context, &mut image_pixel_matrix);
        mark_seam(context, &mut image_pixel_matrix);
        let seam_energy_matrix: Vec<f32> =
            image_pixel_matrix.iter().map(|p| p.seam_energy).collect();
        assert_eq!(seam_energy_matrix, expected_seam_energies);

        #[rustfmt::skip]
        let expected_seam = vec![
            false, false, false, false, true,
            false, false, false, false, true,
            false, false, false, false, true,
        ];
        let seam_matrix: Vec<bool> = image_pixel_matrix
            .iter()
            .map(|p| p.status == PixelStatus::Seam)
            .collect();
        assert_eq!(seam_matrix, expected_seam);
    }

    #[test]
    fn test_mark_seam_medium() {
        let context = ImageContext {
            width: 10,
            height: 5,
        };

        #[rustfmt::skip]
        let mut image_pixel_matrix = from_grayscale(vec![
            10, 50, 10, 50, 10, 10, 50, 10, 50, 10,
            10, 50, 10, 50, 10, 10, 50, 10, 50, 10,
            10, 50, 10, 50, 10, 10, 50, 10, 50, 10,
            10, 50, 10, 50, 10, 10, 50, 10, 50, 10,
            10, 50, 10, 50, 10, 10, 50, 10, 50, 10,
        ]);

        #[rustfmt::skip]
        let expected_seam_energies = vec![
            69.282036, 97.97959, 97.97959, 97.97959, 69.282036, 69.282036, 97.97959, 97.97959, 97.97959, 69.282036,
            138.56407, 167.26163, 195.95918, 167.26163, 138.56407, 138.56407, 167.26163, 195.95918, 167.26163, 138.56407,
            207.8461, 236.54367, 265.2412, 236.54367, 207.8461, 207.8461, 236.54367, 265.2412, 236.54367, 207.8461,
            277.12814, 305.82568, 334.52325, 305.82568, 277.12814, 277.12814, 305.82568, 334.52325, 305.82568, 277.12814,
            346.4102, 375.10773, 403.80527, 375.10773, 346.4102, 346.4102, 375.10773, 403.80527, 375.10773, 346.4102
        ];

        mark_pixel_position(context, &mut image_pixel_matrix);
        mark_energy_map(context, &mut image_pixel_matrix);
        mark_seam_energy_map(context, &mut image_pixel_matrix);
        let seam_energy_matrix: Vec<f32> =
            image_pixel_matrix.iter().map(|p| p.seam_energy).collect();
        assert_eq!(seam_energy_matrix, expected_seam_energies);

        mark_seam(context, &mut image_pixel_matrix);

        #[rustfmt::skip]
        let expected_seam = vec![
            true, false, false, false, false, false, false, false, false, false,
            true, false, false, false, false, false, false, false, false, false,
            true, false, false, false, false, false, false, false, false, false,
            true, false, false, false, false, false, false, false, false, false,
            true, false, false, false, false, false, false, false, false, false
        ];
        let seam_matrix: Vec<bool> = image_pixel_matrix
            .iter()
            .map(|p| p.status == PixelStatus::Seam)
            .collect();
        assert_eq!(seam_matrix, expected_seam);
    }
}
