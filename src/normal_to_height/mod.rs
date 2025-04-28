#![allow(unused)]

use std::path::PathBuf;

use image::{ImageBuffer, Pixel, Rgb};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Diagonal {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub struct NormalToHeight {
    dim: (u32, u32),
    im: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl NormalToHeight {
    pub fn new(image_path: PathBuf) -> image::ImageResult<Self> {
        let im = image::open(image_path)?;
        let im = im.to_rgb32f();

        let dim = im.dimensions();

        Ok(NormalToHeight { dim, im })
    }

    fn get_direction(&self, x: u32, y: u32, direction: Direction) -> Option<&Rgb<u8>> {
        match direction {
            Direction::Up => {
                if let Some(y) = y.checked_sub(1) {
                    return self.im.get_pixel_checked(x, y);
                }

                None
            }
            Direction::Down => self.im.get_pixel_checked(x, y + 1),
            Direction::Left => {
                if let Some(x) = x.checked_sub(1) {
                    return self.im.get_pixel_checked(x, y);
                }

                None
            }
            Direction::Right => self.im.get_pixel_checked(x + 1, y),
        }
    }

    fn get_diagonal(
        im: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        x: u32,
        y: u32,
        diagonal: Diagonal,
    ) -> Option<&Rgb<u8>> {
        match diagonal {
            Diagonal::UpLeft => {
                if let (Some(y), Some(x)) = (y.checked_sub(1), x.checked_sub(1)) {
                    return im.get_pixel_checked(x, y);
                }
                None
            }
            Diagonal::UpRight => {
                if let Some(y) = y.checked_sub(1) {
                    return im.get_pixel_checked(x + 1, y);
                }
                None
            }
            Diagonal::DownLeft => {
                if let Some(x) = x.checked_sub(1) {
                    return im.get_pixel_checked(x, y + 1);
                }
                None
            }
            Diagonal::DownRight => {
                return im.get_pixel_checked(x + 1, y + 1);
            }
        }
    }

    pub fn execute(&self) {
        let mut buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
            image::ImageBuffer::new(self.dim.0, self.dim.1);

        for (_, row) in self.im.enumerate_rows() {
            for (x, y, pixel) in row {

                let Some(down_right) =
                    NormalToHeight::get_diagonal(&self.im, x, y, Diagonal::DownRight)
                else {
                    continue;
                };

                // subtract one from the other
                let mut new = pixel.map2(down_right, |a, b| b.saturating_sub(a));

                let bp = buffer.get_pixel(x, y);

                if let Some(bp_up_left) =
                    NormalToHeight::get_diagonal(&buffer, x, y, Diagonal::UpLeft)
                {
                    new = new.map2(bp_up_left, |a, b| a.saturating_add(b));
                }

                buffer.put_pixel(x, y, new);
            }
        }

        buffer.save("buffer.png").expect("could not save buffer");
    }
}

#[cfg(test)]
mod test {
    use super::NormalToHeight;

    #[test]
    fn test_new() {
        let n = NormalToHeight::new("src/normal_to_height/test.jpg".into()).unwrap();
        n.execute();
    }
}
