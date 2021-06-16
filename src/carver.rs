use web_sys::ImageData;

extern crate web_sys;

#[derive(Copy, Clone)]
struct ImageContext {
    width: u32,
    height: u32,
}

#[derive(Copy, Clone)]
enum DirectionX {
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum DirectionY {
    Top,
    Bottom,
}

// Zero-indexed representation of image pixel coordinates
#[derive(Copy, Clone)]
struct PixelPosition {
    x: u32,
    y: u32,
}

// We need to:
// 1. Make it easier to get neighbors
// 2. Make it easier to track position of pixels

// TODO:
// 2. Add position tracking to ImagePixel
// 3. Modify functions to take and return PixelPosition
// 4. Add more tests with vec!
// 5. Change all h/w to x/y where it makes sense

#[derive(Copy, Clone, PartialEq)]
enum RelativeDirection {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

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

    // Stateful metadata
    // position: Foo
    energy: f32,
    seam_energy: f32,
    status: PixelState,
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
// TODO: This seems quite long. Can we shorten the fn?
fn get_neighbor_pixel_index(
    context: ImageContext,
    current_index: usize,
    direction: RelativeDirection,
) -> Option<usize> {
    return None;
    // let (h, w) = get_pixel_position(context, current_index);

    // let is_top = direction == RelativeDirection::Top
    //     || direction == RelativeDirection::TopLeft
    //     || direction == RelativeDirection::TopRight;
    // let is_bottom = direction == RelativeDirection::BottomLeft
    //     || direction == RelativeDirection::Bottom
    //     || direction == RelativeDirection::BottomRight;
    // let is_left = direction == RelativeDirection::TopLeft
    //     || direction == RelativeDirection::Left
    //     || direction == RelativeDirection::BottomLeft;
    // let is_right = direction == RelativeDirection::TopRight
    //     || direction == RelativeDirection::Right
    //     || direction == RelativeDirection::BottomRight;

    // // Top edge
    // if h == 0 && is_top {
    //     return None;
    // }
    // // Bottom edge
    // if h == context.width as usize && is_bottom {
    //     return None;
    // }
    // // Left edge
    // if w == 0 && is_left {
    //     return None;
    // }
    // if w == context.width as usize && is_right {
    //     return None;
    // }

    // let mut offset_h: i8 = 0;
    // let mut offset_w: i8 = 0;
    // if is_top {
    //     offset_h = 1;
    // }
    // if is_bottom {
    //     offset_h = -1;
    // }
    // if is_left {
    //     offset_w = -1;
    // }
    // if is_right {
    //     offset_w = 1;
    // }

    // return Some(get_pixel_index(
    //     context,
    //     h + offset_h as usize,
    //     w + offset_w as usize,
    // ));
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
        energy: -1.0,
        seam_energy: -1.0,
        status: PixelState::Live,
    };
    let mut matrix = vec![placeholder; w_matrix * h_matrix];

    let data = image_data.data();
    for h in 0..h_matrix {
        for w in 0..w_matrix {
            let start = (h * w_matrix + w) * 4;
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
                energy: -1.0,
                seam_energy: -1.0,
                status: PixelState::Live,
            };
            let pos = PixelPosition {
                x: w as u32,
                y: h as u32,
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

// Mark the energy for every pixel in the image matrix.
fn mark_energy_map(context: ImageContext, image_pixel_matrix: &mut Vec<ImagePixel>) {
    // TODO: Consider splitting the read/write portion of the matrix
    // to avoid having to clone a fairly large vector in each iteration.
    let pixel_matrix_clone = image_pixel_matrix.clone();
    let matrix_width = context.width as usize;

    // TODO: We'll need to account for dead pixels
    for (i, pixel) in image_pixel_matrix.iter_mut().enumerate() {
        let mut left: Option<ImagePixel> = None;
        // let pos = get_pixel_position(context, i as usize);
        // if w > 0 {
        //     left = Some(pixel_matrix_clone[get_pixel_index(context, h, w - 1)]);
        // }

        let mut right: Option<ImagePixel> = None;
        // if w < matrix_width - 1 {
        //     right = Some(pixel_matrix_clone[get_pixel_index(context, h, w + 1)]);
        // }

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
            let top_left =
                get_neighbor_pixel_index(context, current_index, RelativeDirection::TopLeft);
            let top = get_neighbor_pixel_index(context, current_index, RelativeDirection::Top);
            let top_right =
                get_neighbor_pixel_index(context, current_index, RelativeDirection::TopRight);

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
        image_pixel_matrix[index].status = PixelState::Seam;
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

    fn get_pixel(r: u8, g: u8, b: u8) -> ImagePixel {
        return ImagePixel {
            r: r,
            g: g,
            b: b,
            a: 255,
            energy: -1.0,
            seam_energy: -1.0,
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
}
