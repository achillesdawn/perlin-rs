#![allow(unused)]

use std::path::PathBuf;

use image::{ImageBuffer, Rgb};

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
        let im = im.to_rgb8();

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

    fn get_diagonal(&self, x: u32, y: u32, diagonal: Diagonal) -> Option<&Rgb<u8>> {
        match diagonal {
            Diagonal::UpLeft => {
                if let (Some(y), Some(x)) = (y.checked_sub(1), x.checked_sub(1)) {
                    return self.im.get_pixel_checked(x, y);
                }
                None
            }
            Diagonal::UpRight => {
                if let Some(y) = y.checked_sub(1) {
                    return self.im.get_pixel_checked(x + 1, y);
                }
                None
            }
            Diagonal::DownLeft => {
                if let Some(x) = x.checked_sub(1) {
                    return self.im.get_pixel_checked(x, y + 1);
                }
                None
            }
            Diagonal::DownRight => {
                return self.im.get_pixel_checked(x + 1, y + 1);
            }
        }
    }

    pub fn execute(&self) {
        for (_, row) in self.im.enumerate_rows() {
            for (x, y, pixel) in row {
                let down = self.get_direction(x, y, Direction::Down);
                let up = self.get_direction(x, y, Direction::Up);
                let left = self.get_direction(x, y, Direction::Left);
                let right = self.get_direction(x, y, Direction::Right);

                dbg!(down, up, left, right);
                break;
            }
            break;
        }
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
