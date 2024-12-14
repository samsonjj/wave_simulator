use std::f32::consts::PI;
use ndarray::prelude::*;

use macroquad::prelude::*;

use super::{Field, Pixel};

pub struct Field2D {
    u: Array2<f32>,
    v: Array2<f32>,
    vectorized: bool,
}

impl Field for Field2D {
    fn render(&self) {
        let pixel_width = 400. / self.width() as f32;
        let pixel_height = 400. / self.height() as f32;
        let offset_x = 50.0;
        let offset_y = 50.0;

        for i in 0..self.height() {
            for j in 0..self.width() {
                let u = *self.u.get((i, j)).unwrap();
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

        if self.vectorized {
            let result = &self.v.slice(s![1..-1, 1..-1])
                + (0.005 * &(-4. * &self.u.slice(s![1..-1, 1..-1])
                + &self.u.slice(s![2.., 1..-1])
                + &self.u.slice(s![..-2, 1..-1])
                + &self.u.slice(s![1..-1, 2..])
                + &self.u.slice(s![1..-1, ..-2])));

            result.assign_to(self.v.slice_mut(s![1..-1, 1..-1]));
        } else {
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
            for i in 0..self.height() {
                for j in 0..self.width() {
                    *self.v.get_mut((i, j)).unwrap() += field_deltas[i][j];
                }
            }
        }

        // update volocities
                
        // update values
        self.u = &self.u + &self.v;
    }
}

impl Field2D {
    pub fn new(vectorized: bool) -> Self {
        let pixels = Self::pixels_centered();
        Self {
            // pixels: vec![vec![0f32; 16]]
            u: pixels.0,
            v: pixels.1,
            vectorized,
            // pixels: Self::pixels_at_end(),
        }
    }
    fn pixels_centered() -> (Array2<f32>, Array2<f32>) {
        const WIDTH: usize = 64;
        const HEIGHT: usize = 64;
        let mut u = Array2::default((WIDTH, HEIGHT));
        let v = Array2::default((WIDTH, HEIGHT));
        let center = vec2(WIDTH as f32 / 2., HEIGHT as f32 / 2.);
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let distance = center.distance(vec2(i as f32, j as f32)) * 0.1;
                if distance <= PI / 2. {
                    *u.get_mut((i, j)).unwrap() =
                        255. * distance.cos();
                }
            }
        }
        (u, v)
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
        (0.005) * (self.u.get((source.1, source.0)).unwrap() - self.u.get((target.1, target.0)).unwrap())
    }
    fn width(&self) -> usize {
        self.u.shape()[1]
    }
    fn height(&self) -> usize {
        self.u.shape()[0]
    }
}
