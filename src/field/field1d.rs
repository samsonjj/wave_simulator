use std::f32::consts::PI;

use macroquad::prelude::*;

use super::{Field, Pixel};

pub struct Field1D {
    pixels: Vec<Vec<Pixel>>,
}

impl Field for Field1D {
    fn render(&self) {
        let pixel_width = 1.5;
        let pixel_height = 40.0;
        let offset_x = 50.0;
        let offset_y = 50.0;

        for j in 0..self.width() {
            let u = self.pixels[0][j].u;
            let red = u as u8;
            let green = (-u) as u8;
            draw_rectangle(
                offset_x + pixel_width * j as f32,
                offset_y,
                pixel_width,
                pixel_height,
                Color::from_rgba(red, green, 0, 255),
            );
        }
    }
    fn update(&mut self) {
        let mut field_deltas = vec![vec![0f32; self.width()]; self.height()];
        for j in 0..self.width() {
            field_deltas[0][j] += self.force(j, j as i32 + 1);
            field_deltas[0][j] += self.force(j, j as i32 - 1);
        }
        // update volocities
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.pixels[i][j].v += field_deltas[i][j];
            }
        }
        // update values
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.pixels[i][j].u += self.pixels[i][j].v;
            }
        }
    }
}

impl Field1D {
    pub fn new() -> Self {
        Self {
            // pixels: vec![vec![0f32; 16]]
            // pixels: Self::pixels_centered(),
            pixels: Self::pixels_at_end(),
        }
    }
    fn pixels_centered() -> Vec<Vec<Pixel>> {
        let mut pixels = vec![vec![Pixel::zero(); 256]];
        // let half = pixels[0].len() / 2;
        for j in 120..136 {
            pixels[0][j] = Pixel { u: 255.0, v: 0.0 };
        }
        pixels
    }
    fn pixels_at_end() -> Vec<Vec<Pixel>> {
        let mut pixels = vec![vec![Pixel::zero(); 256]];
        // let half = pixels[0].len() / 2;
        for j in 0..20 {
            pixels[0][j] = Pixel {
                u: (j as f32 * PI * 0.025).cos() * 255.0,
                v: 0.0,
            };
        }
        pixels
    }

    // attempt to simulate the second derivative
    fn force(&self, target: usize, source: i32) -> f32 {
        if source < 0 || source >= self.width() as i32 {
            return 0f32;
        }
        let source = source as usize;
        // c^2
        (0.05) * (self.pixels[0][source].u - self.pixels[0][target].u)
    }
    fn width(&self) -> usize {
        self.pixels[0].len()
    }
    fn height(&self) -> usize {
        self.pixels.len()
    }
}
