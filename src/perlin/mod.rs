#![allow(unused)]

use image::{GrayImage, Luma};
use rand::{rngs::ThreadRng, Rng};

use crate::vector2::Vector2;

#[derive(Debug)]
struct GridPoint {
    pos: Vector2,
    vector: Vector2,
}

type Grid = Vec<Vec<GridPoint>>;

pub struct Perlin {
    size: (u32, u32),
    subdivisions: u32,
    subsize: (f32, f32),
    image: GrayImage,
    grid: Grid,
}

impl Perlin {
    fn generate_grid(size: (u32, u32), subdivisions: u32) -> ((f32, f32), Grid) {
        let x = size.0 as f32 / subdivisions as f32;
        let y = size.1 as f32 / subdivisions as f32;

        let mut grid: Vec<Vec<GridPoint>> = Vec::new();
        let mut rng = rand::thread_rng();

        for row_idx in 0..=subdivisions {
            let x_pos = x * row_idx as f32;

            dbg!(&x_pos);

            let mut row = Vec::new();

            for col in 0..=subdivisions {
                let y_pos = y * col as f32;

                dbg!(&y_pos);
                row.push(GridPoint {
                    pos: Vector2 { x: x_pos, y: y_pos },
                    vector: Vector2::random(&mut rng),
                });
            }

            grid.push(row);
        }

        ((x, y), grid)
    }

    pub fn new(size: (u32, u32), subdivisions: u32, image: GrayImage) -> Self {
        let (subsize, grid) = Perlin::generate_grid(size, subdivisions);

        Perlin {
            size,
            subdivisions,
            image,
            grid,
            subsize,
        }
    }

    fn check_bounds(&self, point: &GridPoint) -> u8 {
        let mut check: u8 = 0b1111;

        if point.pos.y == 0.0 {
            check &= 0b0011
        } else if point.pos.y == (self.size.1 as f32) {
            check &= 0b1100
        }

        if point.pos.x == 0.0 {
            check &= 0b0101
        } else if point.pos.x == (self.size.0 as f32) {
            check &= 0b1010
        }

        check
    }

    fn loop_through(
        point: &GridPoint,
        x_start: u32,
        mut x_end: u32,
        y_start: u32,
        mut y_end: u32,
        image: &mut GrayImage,
    ) {
        let (image_x, image_y) = image.dimensions();

        for x in x_start..x_end {
            for y in y_start..y_end {
                let mut offset = Vector2::new(x, y).subtract(&point.pos);
                offset.normalize();

                let dot = offset.dot(&point.vector);
                // let dot_normalized = (dot /2.0) + 0.5 ;
                let dotu8 = (dot * 255.0) as u8;

                image.put_pixel(x, y, Luma([dotu8]));
            }
        }
    }

    pub fn generate(&mut self) {
        let (x_offset, y_offset) = self.subsize;
        let (x_offset, y_offset) = (x_offset / 2.0, y_offset / 2.0);

        for row in self.grid.iter() {
            for point in row {
                let check = self.check_bounds(point);

                let (mut x_u32, mut y_u32) = point.pos.as_u32();

                if (check & 0b1000) != 0 {
                    println!("Up Left");
                    let x_start = (point.pos.x - x_offset) as u32;
                    let y_start = (point.pos.y - y_offset) as u32;

                    Perlin::loop_through(point, x_start, x_u32, y_start, y_u32, &mut self.image);
                }

                if (check & 0b0100) != 0 {
                    println!("Up Right");
                    let x_end = (point.pos.x + x_offset) as u32;
                    let y_start = (point.pos.y - y_offset) as u32;

                    Perlin::loop_through(point, x_u32, x_end, y_start, y_u32, &mut self.image);
                }

                if (check & 0b0010) != 0 {
                    println!("Down Left");
                    let x_start = (point.pos.x - x_offset) as u32;
                    let y_end = (point.pos.y + y_offset) as u32;

                    Perlin::loop_through(point, x_start, x_u32, y_u32, y_end, &mut self.image);
                }
                if (check & 0b0001) != 0 {
                    println!("Down Right");
                    let x_end = (point.pos.x + x_offset) as u32;
                    let y_end = (point.pos.y + y_offset) as u32;

                    Perlin::loop_through(point, x_u32, x_end, y_u32, y_end, &mut self.image);
                }
            }
        }

        self.image.save("result.png");
    }
}


