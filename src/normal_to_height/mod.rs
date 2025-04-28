#![allow(unused)]

use std::{
    ops::{Add, Div, Mul, Sub},
    path::PathBuf,
};

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

fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub struct NormalToHeight {
    dim: (u32, u32),
    im: ImageBuffer<Rgb<f32>, Vec<f32>>,
}

impl NormalToHeight {
    pub fn new(image_path: PathBuf) -> image::ImageResult<Self> {
        let im = image::open(image_path)?;
        let im = im.to_rgb32f();

        let dim = im.dimensions();

        Ok(NormalToHeight { dim, im })
    }

    fn get_direction(&self, x: u32, y: u32, direction: Direction) -> Option<&Rgb<f32>> {
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
        im: &ImageBuffer<Rgb<f32>, Vec<f32>>,
        x: u32,
        y: u32,
        diagonal: Diagonal,
    ) -> Option<&Rgb<f32>> {
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

    fn to_rgb(
        mut buffer: ImageBuffer<Rgb<f32>, Vec<f32>>,
        min: Rgb<f32>,
        max: Rgb<f32>,
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        for pixel in buffer.pixels_mut() {
            let mut data = pixel.0;

            for (i, v) in data.into_iter().enumerate() {
                data[i] = map_range((min.0[i], max.0[i]), (0.0, 1.0), v)
            }

            pixel.0 = data;
        }

        ImageBuffer::from_par_fn(buffer.width(), buffer.height(), |x, y| {
            let pixel = buffer.get_pixel(x, y);

            let data = pixel.0;

            Rgb([
                (data[0] * 255.0) as u8,
                (data[1] * 255.0) as u8,
                (data[2] * 255.0) as u8,
            ])
        })
    }

    pub fn execute(&self) {
        let mut buffer: ImageBuffer<Rgb<f32>, Vec<f32>> =
            image::ImageBuffer::new(self.dim.0, self.dim.1);

        let mut min_values = Rgb([0.0f32, 0.0, 0.0]);
        let mut max_values = Rgb([0.0f32, 0.0, 0.0]);

        for (_, row) in self.im.enumerate_rows() {
            for (x, y, pixel) in row {
                let Some(down_right) =
                    NormalToHeight::get_diagonal(&self.im, x, y, Diagonal::DownLeft)
                else {
                    continue;
                };

                // subtract one from the other
                let mut new = pixel.map2(down_right, |a, b| b - a);

                let bp = buffer.get_pixel(x, y);

                if let Some(bp_up_left) =
                    NormalToHeight::get_diagonal(&buffer, x, y, Diagonal::UpRight)
                {
                    new = new.map2(bp_up_left, |a, b| a + b);
                }

                new.0.iter().enumerate().for_each(|(i, v)| {
                    if *v > max_values.0[i] {
                        max_values.0[i] = *v;
                    } else if *v < min_values.0[i] {
                        min_values.0[i] = *v;
                    }
                });

                buffer.put_pixel(x, y, new);
            }
        }

        let buffer = NormalToHeight::to_rgb(buffer, min_values, max_values);

        buffer.save("buffer_up_right.png").expect("could not save buffer");
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
