mod fixture;

#[cfg(test)]
mod integration {
    use super::*;

    #[test]
    fn test_helper() {
        let (dimensions, rgba) = fixture::get_fixture_image("beach-1.jpeg");
        assert_eq!(dimensions.width, 300);
        assert_eq!(dimensions.height, 180);
        assert_eq!(rgba.len(), 300 * 180 * 4);
    }

    #[test]
    fn test_beach_example() {
        let (dimensions, image_data) = fixture::get_fixture_image("beach-1.jpeg");

        let mut carver =
            seam_carving::SeamCarver::from_vec(image_data, dimensions.width, dimensions.height);
        let steps = 100;
        for _ in 0..steps {
            carver.mark_seam();
            carver.delete_seam();
        }

        let resized_image_data = carver.image_data_vec();
        let expected_size = (dimensions.width - steps) * dimensions.height * 4;
        assert_eq!(resized_image_data.len(), expected_size as usize);
        fixture::save_fixture_image(
            "beach-1-resized.jpeg",
            dimensions.width - steps,
            dimensions.height,
            resized_image_data,
        );
    }

    #[test]
    fn test_beach_flipped_example() {
        let (dimensions, image_data) = fixture::get_fixture_image("beach-1-flipped.jpeg");
        let mut carver =
            seam_carving::SeamCarver::from_vec(image_data, dimensions.width, dimensions.height);
        let steps = 100;
        for _ in 0..steps {
            carver.mark_seam();
            carver.delete_seam();
        }

        let resized_image_data = carver.image_data_vec();
        let expected_size = (dimensions.width - steps) * dimensions.height * 4;
        assert_eq!(resized_image_data.len(), expected_size as usize);
        fixture::save_fixture_image(
            "beach-1-flipped-resized.jpeg",
            dimensions.width - steps,
            dimensions.height,
            resized_image_data,
        );
    }
}
