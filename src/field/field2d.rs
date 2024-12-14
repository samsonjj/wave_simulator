use std::f32::consts::PI;

use macroquad::prelude::*;

use super::{Field, Pixel};

pub struct Field2D {
    pixels: Vec<Vec<Pixel>>,
}

impl Field for Field2D {
    fn render(&self) {
        let pixel_width = 400. / self.width() as f32;
        let pixel_height = 400. / self.height() as f32;
        let offset_x = 50.0;
        let offset_y = 50.0;

        for i in 0..self.height() {
            for j in 0..self.width() {
                let u = self.pixels[i][j].u;
                let red = u as u8;
                let green = (-u) as u8;
                draw_rectangle(
                    offset_x + pixel_width * j as f32,
                    offset_y + pixel_height * i as f32,
                    pixel_width,
                    pixel_height,
                    Color::from_rgba(red, green, 0, 255),
                );
            }
        }
    }
    fn update(&mut self) {
        let mut field_deltas = vec![vec![0f32; self.width()]; self.height()];
        for i in 0..self.height() {
            for j in 0..self.width() {
                field_deltas[i][j] += self.force((j, i), (j as i32 + 1, i as i32));
                field_deltas[i][j] += self.force((j, i), (j as i32 - 1, i as i32));
                field_deltas[i][j] += self.force((j, i), (j as i32, i as i32 - 1));
                field_deltas[i][j] += self.force((j, i), (j as i32, i as i32 + 1));

                field_deltas[i][j] += 0.0625 * self.force((j, i), (j as i32, i as i32 + 2));
                field_deltas[i][j] += 0.0625 * self.force((j, i), (j as i32, i as i32 - 2));
                field_deltas[i][j] += 0.0625 * self.force((j, i), (j as i32 + 2, i as i32));
                field_deltas[i][j] += 0.0625 * self.force((j, i), (j as i32 - 2, i as i32));
            }
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

impl Field2D {
    pub fn new() -> Self {
        Self {
            // pixels: vec![vec![0f32; 16]]
            pixels: Self::pixels_centered(),
            // pixels: Self::pixels_at_end(),
        }
    }
    fn pixels_centered() -> Vec<Vec<Pixel>> {
        const WIDTH: usize = 64;
        const HEIGHT: usize = 64;
        let mut pixels = vec![vec![Pixel::zero(); WIDTH]; HEIGHT];
        // pixels[HEIGHT / 2][WIDTH / 2] = Pixel { u: 5000.0, v: 0.0 };
        let center = vec2(WIDTH as f32 / 2., HEIGHT as f32 / 2.);
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let distance = center.distance(vec2(i as f32, j as f32)) * 0.1;
                if distance <= PI / 2. {
                    pixels[i][j] = Pixel {
                        u: 255. * distance.cos(),
                        v: 0.,
                    };
                }
            }
        }
        pixels
    }
    fn pixels_at_end() -> Vec<Vec<Pixel>> {
        let mut pixels = vec![vec![Pixel::zero(); 128]; 128];
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
    fn force(&self, target: (usize, usize), source: (i32, i32)) -> f32 {
        if source.0 < 0 || source.0 >= self.width() as i32 {
            return 0f32;
        }
        if source.1 < 0 || source.1 >= self.height() as i32 {
            return 0f32;
        }
        let source = (source.0 as usize, source.1 as usize);
        // c^2
        (0.005) * (self.pixels[source.1][source.0].u - self.pixels[target.1][target.0].u)
    }
    fn width(&self) -> usize {
        self.pixels[0].len()
    }
    fn height(&self) -> usize {
        self.pixels.len()
    }
}
