use crate::Color;

/// An image, containing an array of colors.
pub struct Image {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Image {
    /// Build an image from an array of colors.
    pub fn from_pixels(width: u32, pixels: Vec<Color>) -> Self {
        let pixels_count = pixels.len();
        if pixels_count < width as usize {
            // FIXME
            panic!("width should be less than the pixels count")
        }
        let height = pixels_count as u32 / width;

        Self {
            width,
            height,
            data: pixels,
        }
    }

    /// Get the image width.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the image height.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get the image pixels.
    pub fn pixels(&self) -> &[Color] {
        &self.data
    }
}
