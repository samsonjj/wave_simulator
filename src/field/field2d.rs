use ndarray::prelude::*;
use std::f32::consts::PI;

use macroquad::prelude::*;

use super::{Field, Pixel};

const PROPAGATION_SPEED: f32 = 0.01;

pub struct Field2D {
    u: Array2<f32>,
    v: Array2<f32>,
    vectorized: bool,
}

impl Field for Field2D {
    fn render(&self) {
        let offset_x = 50.0;
        let offset_y = 50.0;

        let mut image = Image::gen_image_color(self.width() as u16, self.height() as u16, WHITE);

        for y in 0..self.height() {
            for x in 0..self.width() {
                let u = *self.u.get((x, y)).unwrap();
                let red = u as u8;
                let green = (-u) as u8;
                image.set_pixel(x as u32, y as u32, Color::from_rgba(red, green, 0, 255));
            }
        }

        let texture = Texture2D::from_image(&image);
        draw_texture_ex(
            &texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(400., 400.)),
                ..Default::default()
            },
        );
    }

    fn update(&mut self) {
        let mut field_deltas = vec![vec![0f32; self.height()]; self.width()];

        if self.vectorized {
            // the following 4 updates ensure reflective boundaries
            let r = 1. * &self.u.slice(s![1..-1, -2..-1]);
            r.assign_to(self.u.slice_mut(s![1..-1, -1..]));

            let r = 1. * &self.u.slice(s![1..-1, 1..2]);
            r.assign_to(self.u.slice_mut(s![1..-1, ..1]));

            let r = 1. * &self.u.slice(s![-2..-1, 1..-1]);
            r.assign_to(self.u.slice_mut(s![-1.., 1..-1]));

            let r = 1. * &self.u.slice(s![1..2, 1..-1]);
            r.assign_to(self.u.slice_mut(s![..1, 1..-1]));

            let result = &self.v.slice(s![1..-1, 1..-1])
                + (PROPAGATION_SPEED
                    * &(-4. * &self.u.slice(s![1..-1, 1..-1])
                        + &self.u.slice(s![2.., 1..-1])
                        + &self.u.slice(s![..-2, 1..-1])
                        + &self.u.slice(s![1..-1, 2..])
                        + &self.u.slice(s![1..-1, ..-2])));

            result.assign_to(self.v.slice_mut(s![1..-1, 1..-1]));
        } else {
            for x in 0..self.width() {
                for y in 0..self.height() {
                    field_deltas[x][y] += self.force((x, y), (x as i32 + 1, y as i32));
                    field_deltas[x][y] += self.force((x, y), (x as i32 - 1, y as i32));
                    field_deltas[x][y] += self.force((x, y), (x as i32, y as i32 - 1));
                    field_deltas[x][y] += self.force((x, y), (x as i32, y as i32 + 1));

                    field_deltas[x][y] += 0.0625 * self.force((x, y), (x as i32, y as i32 + 2));
                    field_deltas[x][y] += 0.0625 * self.force((x, y), (x as i32, y as i32 - 2));
                    field_deltas[x][y] += 0.0625 * self.force((x, y), (x as i32 + 2, y as i32));
                    field_deltas[x][y] += 0.0625 * self.force((x, y), (x as i32 - 2, y as i32));
                }
            }
            for y in 0..self.height() {
                for x in 0..self.width() {
                    *self.v.get_mut((x, y)).unwrap() += field_deltas[x][y];
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
        let pixels = Self::pixels_centered(64, 63);
        Self {
            u: pixels.0,
            v: pixels.1,
            vectorized,
        }
    }
    fn pixels_centered(width: usize, height: usize) -> (Array2<f32>, Array2<f32>) {
        // const WIDTH: usize = 64;
        // const HEIGHT: usize = 64;
        let WIDTH = width;
        let HEIGHT = height;
        let mut u = Array2::default((WIDTH, HEIGHT));
        let v = Array2::default((WIDTH, HEIGHT));
        let center = vec2(WIDTH as f32 / 2., HEIGHT as f32 / 2.);
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let distance = center.distance(vec2(i as f32, j as f32)) * 0.1;
                if distance <= PI / 2. {
                    *u.get_mut((i, j)).unwrap() = 255. * distance.cos();
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
        PROPAGATION_SPEED
            * (self.u.get((source.0, source.1)).unwrap()
                - self.u.get((target.0, target.1)).unwrap())
    }
    fn width(&self) -> usize {
        self.u.shape()[0]
    }
    fn height(&self) -> usize {
        self.u.shape()[1]
    }
}
