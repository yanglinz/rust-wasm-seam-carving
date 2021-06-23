use seam_carver;

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
        let (dimensions, rgba) = fixture::get_fixture_image("beach-1.jpeg");
        let resized_width = dimensions.width - 100;
        let resized_height = dimensions.height;

        let resized = seam_carver::resize_internal(
            rgba,
            dimensions.width,
            dimensions.height,
            resized_width,
            resized_height,
        );
        let expected_size = resized_width * resized_height * 4;
        assert_eq!(resized.len(), expected_size as usize);
        fixture::save_fixture_image(
            "beach-1-resized.jpeg",
            resized_width,
            resized_height,
            resized,
        );
    }

    #[test]
    fn test_beach_flipped_example() {
        let (dimensions, rgba) = fixture::get_fixture_image("beach-1-flipped.jpeg");
        let resized_width = dimensions.width - 100;
        let resized_height = dimensions.height;

        let resized = seam_carver::resize_internal(
            rgba,
            dimensions.width,
            dimensions.height,
            resized_width,
            resized_height,
        );
        let expected_size = resized_width * resized_height * 4;
        assert_eq!(resized.len(), expected_size as usize);
        fixture::save_fixture_image(
            "beach-1-flipped-resized.jpeg",
            resized_width,
            resized_height,
            resized,
        );
    }
}
