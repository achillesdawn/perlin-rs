use rand::{rngs::ThreadRng, Rng};
use std::f32::consts::PI;

const TAU: f32 = 2.0 * PI;

#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: u32, y: u32) -> Self {
        Vector2 {
            x: x as f32,
            y: y as f32,
        }
    }

    pub fn random(rng: &mut ThreadRng) -> Self {
        let random_number: f32 = rng.gen();
        let angle = random_number * TAU;

        Vector2 {
            x: angle.sin(),
            y: angle.cos(),
        }
    }

    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalized(&self) -> Self {
        let mag = self.magnitude();

        if mag != 0.0 {
            Vector2 {
                x: self.x / mag,
                y: self.y / mag,
            }
        } else {
            panic!("Magnitude Zero Could not normalize")
        }
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();

        if mag != 0.0 {
            self.x /= mag;
            self.y /= mag;
        }
    }

    pub fn dot(&self, mut other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn as_u32(&self) -> (u32, u32) {
        (self.x as u32, self.y as u32)
    }

    pub fn subtract(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}