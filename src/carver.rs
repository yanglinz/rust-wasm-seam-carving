use web_sys::ImageData;

extern crate web_sys;

#[derive(Copy, Clone)]
struct ImageContext {
    width: u32,
    height: u32,
}

// Zero-indexed representation of image pixel coordinates
#[derive(Copy, Clone)]
struct PixelPosition {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone, PartialEq)]
enum PixelStatus {
    Live,
    Seam,
}

#[derive(Copy, Clone)]
struct ImagePixel {
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
    return index as usize;
}

// Same as the get_pixel_index helper, just with the inverse logic.
fn get_pixel_position(context: ImageContext, index: usize) -> PixelPosition {
    let x = (index as u32).rem_euclid(context.width);
    let y = index as u32 / context.width;
    return PixelPosition { x: x, y: y };
}

// Helper to get a given pixel's neighbor index
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
    let will_overflow = false
        || (pos.y == 0 && offset_y == -1)
        || (pos.y == context.height - 1 && offset_y == 1)
        || (pos.x == 0 && offset_x == -1)
        || (pos.x == context.width - 1 && offset_x == 1);
    if will_overflow {
        return None;
    }

    let new_x = pos.x as i8 + offset_x;
    let new_y = pos.y as i8 + offset_y;
    return Some(get_pixel_index(
        context,
        PixelPosition {
            x: new_x as u32,
            y: new_y as u32,
        },
    ));
}

// Helper to get a given pixel's neighbor pixel
fn get_neighbor_pixel(
    context: ImageContext,
    image_pixel_matrix: &Vec<ImagePixel>,
    index: usize,
    offset_x: i8,
    offset_y: i8,
) -> Option<ImagePixel> {
    let index = get_neighbor_pixel_index(context, index, offset_x, offset_y);
    let pixel = match index {
        None => None,
        _ => Some(image_pixel_matrix[index.unwrap()]),
    };
    return pixel;
}

// We can initialize the image "matrix" with some placeholder values.
fn get_image_pixel_matrix(context: ImageContext, image_data: ImageData) -> Vec<ImagePixel> {
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

    let data = image_data.data();
    for h in 0..h_matrix {
        for w in 0..w_matrix {
            let start = (h * w_matrix + w) * 4;
            let start_index = start as usize;
            let pos = PixelPosition {
                x: context.width,
                y: context.height,
            };
            let pixel = ImagePixel {
                r: data[start_index + 0],
                g: data[start_index + 1],
                b: data[start_index + 2],
                a: data[start_index + 3],
                status: PixelStatus::Live,
                position: pos,
                energy: -1.0,
                seam_energy: -1.0,
            };
            matrix[get_pixel_index(context, pos)] = pixel;
        }
    }

    return matrix;
}

// Helper function to calculate the image pixel "energy" from its neighbors.
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

    return (left_energy + right_energy).sqrt();
}

// Mark the energy for every pixel in the image matrix.
fn mark_energy_map(context: ImageContext, image_pixel_matrix: &mut Vec<ImagePixel>) {
    // TODO: Consider splitting the read/write portion of the matrix
    // to avoid having to clone a fairly large vector in each iteration.
    let pixel_matrix_clone = image_pixel_matrix.clone();
    for (i, pixel) in image_pixel_matrix.iter_mut().enumerate() {
        let left = get_neighbor_pixel(context, &pixel_matrix_clone, i, -1, 0);
        let right = get_neighbor_pixel(context, &pixel_matrix_clone, i, 1, 0);
        pixel.energy = get_energy(pixel.clone(), left, right);
    }
}

fn mark_seam(context: ImageContext, image_pixel_matrix: &mut Vec<ImagePixel>) {
    let w_matrix = context.width as usize;
    let h_matrix = context.height as usize;

    for h in 0..h_matrix {
        for w in 0..w_matrix {
            let index = get_pixel_index(
                context,
                PixelPosition {
                    x: w as u32,
                    y: h as u32,
                },
            );

            // For the first row, seam energy is just the copy of pixel energy
            if h == 0 {
                image_pixel_matrix[index].seam_energy = image_pixel_matrix[index].energy;
            } else {
                // TODO: Convert to use get_neighbor_pixel_index

                let is_left_border = w == 0;
                // TODO: We'll have to consider dead pixels
                let is_right_border = w == w_matrix;

                // if is_left_border {
                //     let center = image_pixel_matrix[get_pixel_index(context, h - 1, w)];
                //     let right = image_pixel_matrix[get_pixel_index(context, h - 1, w + 1)];
                //     let min = center.seam_energy.min(right.seam_energy);
                //     image_pixel_matrix[index].seam_energy =
                //         image_pixel_matrix[index].seam_energy + min;
                // } else if is_right_border {
                //     let center = image_pixel_matrix[get_pixel_index(context, h - 1, w)];
                //     let left = image_pixel_matrix[get_pixel_index(context, h - 1, w - 1)];
                //     let min = center.seam_energy.min(left.seam_energy);
                //     image_pixel_matrix[index].seam_energy =
                //         image_pixel_matrix[index].seam_energy + min;
                // } else {
                //     let center = image_pixel_matrix[get_pixel_index(context, h - 1, w)];
                //     let left = image_pixel_matrix[get_pixel_index(context, h - 1, w - 1)];
                //     let right = image_pixel_matrix[get_pixel_index(context, h - 1, w + 1)];
                //     let min = center
                //         .seam_energy
                //         .min(left.seam_energy)
                //         .min(right.seam_energy);
                //     image_pixel_matrix[index].seam_energy =
                //         image_pixel_matrix[index].seam_energy + min;
                // }
            }
        }
    }

    let mut x = 0;
    let mut y = 0;
    for h in (0..h_matrix).rev() {
        let is_last = h == h_matrix - 1;
        let is_first = h == 0;
        if is_first || is_last {
            let start = (h_matrix - 1) * w_matrix;
            let end = h_matrix * w_matrix;
            let last_row = &image_pixel_matrix[start..end];
            let mut min_energy = 9999.0;
            for (i, p) in last_row.iter().enumerate() {
                if p.seam_energy < min_energy {
                    min_energy = p.seam_energy;
                    x = i;
                    y = h;
                }
            }
        } else {
            let current_index = get_pixel_index(
                context,
                PixelPosition {
                    x: x as u32,
                    y: y as u32,
                },
            );
            let top_left = get_neighbor_pixel_index(context, current_index, -1, 1);
            let top = get_neighbor_pixel_index(context, current_index, 0, 1);
            let top_right = get_neighbor_pixel_index(context, current_index, 1, 1);

            if top_left.is_none() && top.is_none() && top_right.is_none() {
                // Do nothing
            } else {
                // let pixels = vec![top_left, top, top_right];
                // let min = pixels.into_iter().filter(|x| x.is_some()).min();

                // TODO: Attach index to pixels; it'll make the traversal a lot easier
            }
        }

        // Mark the pixel
        let index = get_pixel_index(
            context,
            PixelPosition {
                x: x as u32,
                y: y as u32,
            },
        );
        image_pixel_matrix[index].status = PixelStatus::Seam;
    }

    // Debugging artifacts
    // web_sys::console::log_1(&"Debug 0".into());
    // web_sys::console::log_1(&x.to_string().into());
    // web_sys::console::log_1(&y.to_string().into());
}

fn remove_seam(context: ImageContext, image_pixel_matrix: &mut Vec<ImagePixel>) {
    // TODO: Implement seam removal
}

pub fn get_resized_image_data(
    image_data: ImageData,
    width_current: u32,
    height_current: u32,
    width_target: u32,
    height_target: u32,
) -> Vec<u8> {
    let context = ImageContext {
        width: width_current,
        height: height_current,
    };

    let mut matrix = get_image_pixel_matrix(context, image_data);
    let steps = width_current - width_target;
    for s in 0..steps {
        let context = ImageContext {
            width: width_current - s,
            height: height_current,
        };

        mark_energy_map(context, &mut matrix);
        mark_seam(context, &mut matrix);
        remove_seam(context, &mut matrix);
    }

    let mut data = Vec::new();
    for h in 0..height_current {
        for w in 0..width_current {
            let index = get_pixel_index(
                context,
                PixelPosition {
                    x: w as u32,
                    y: h as u32,
                },
            );
            let pixel = matrix[index];
            data.push(pixel.r);
            data.push(pixel.g);
            data.push(50);
            data.push(pixel.a);
        }
    }

    return data;
}

#[cfg(test)]
mod tests {
    use super::*;

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
        return ImagePixel {
            r: r,
            g: g,
            b: b,
            a: 255,
            status: PixelStatus::Live,
            position: PixelPosition { x: 0, y: 0 },
            energy: -1.0,
            seam_energy: -1.0,
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

    #[test]
    fn test_mark_energy_map() {
        let context = ImageContext {
            width: 3,
            height: 3,
        };

        #[rustfmt::skip]
        let rgb_matrix = vec![
            (100, 100, 100), (0, 100, 0), (200, 200, 0),
            (100, 0, 0),     (0, 100, 0), (0, 100, 200),
            (100, 0, 0),     (0, 100, 0), (0, 100, 200),
        ];

        #[rustfmt::skip]
        let expected_energy = vec![
            141.42136, 264.57513, 223.6068, 
            141.42136, 244.94897, 200.0, 
            141.42136, 244.94897, 200.0
        ];

        let mut image_pixel_matrix = vec![];
        for (r, g, b) in rgb_matrix {
            image_pixel_matrix.push(ImagePixel {
                r: r as u8,
                g: g as u8,
                b: b as u8,
                a: 255,
                status: PixelStatus::Live,
                position: PixelPosition { x: 0, y: 0 },
                energy: -1.0,
                seam_energy: -1.0,
            })
        }
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
        let rgb_matrix = vec![
            (100, 100, 100), (0, 100, 0), (200, 200, 0), (0, 100, 0),  (100, 20, 0),
            (100, 0, 0),     (0, 100, 0), (0, 100, 200), (150, 50, 0), (100, 20, 0),
            (100, 0, 0),     (0, 100, 0), (0, 100, 200), (200, 20, 0), (100, 20, 0),
        ];

        #[rustfmt::skip]
        let expected_seam = vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
        ];

        let mut image_pixel_matrix = vec![];
        for (r, g, b) in rgb_matrix {
            image_pixel_matrix.push(ImagePixel {
                r: r as u8,
                g: g as u8,
                b: b as u8,
                a: 255,
                status: PixelStatus::Live,
                position: PixelPosition { x: 0, y: 0 },
                energy: -1.0,
                seam_energy: -1.0,
            })
        }
        mark_energy_map(context, &mut image_pixel_matrix);
        mark_seam(context, &mut image_pixel_matrix);
        let seam_matrix: Vec<bool> = image_pixel_matrix
            .iter()
            .map(|p| p.status == PixelStatus::Seam)
            .collect();
        assert_eq!(seam_matrix, expected_seam);
    }
}
