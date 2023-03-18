mod imgedit {
    use image::{ImageBuffer};

    pub fn fill_image(mut image: ImageBuffer<image::Luma<u8>, Vec<u8>>, color: image::Luma<u8>) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
        let (width, height) = image.dimensions();

        for i in 0..width {
            for j in 0..height {
                image.put_pixel(i, j, color);
            }
        }

        image
    }
}