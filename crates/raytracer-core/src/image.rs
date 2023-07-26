use crate::Color;

pub struct Image {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Image {
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[Color] {
        &self.data
    }
}
