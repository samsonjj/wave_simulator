mod field1d;
mod field2d;

pub use field1d::{Field1D, Field1DInit};
pub use field2d::{Field2D, Field2DInit};

use macroquad::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    u: f32,
    v: f32,
}
impl Pixel {
    fn zero() -> Self {
        Pixel { u: 0.0, v: 0.0 }
    }
}

pub trait Field {
    fn render(&self);
    fn update(&mut self);
}
