use std::time::Instant;

use macroquad::prelude::*;
use miniquad::window::quit;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Running,
    Paused,
}

pub struct Game {
    pub field: Field,
    pub state: GameState,
    pub step: i32,
    pub just_updated: bool,
    pub start_time: Instant,
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: Field::new(),
            state: GameState::Paused,
            step: 0,
            just_updated: false,
            start_time: Instant::now(),
        }
    }
    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.state = match self.state {
                GameState::Running => GameState::Paused,
                GameState::Paused => GameState::Running,
            }
        }
        let should_update = if self.state == GameState::Running {
            true
        } else if is_key_pressed(KeyCode::Period) {
            true
        } else {
            false
        };
        if should_update {
            self.field.update();
            self.step += 1;
            self.field.print();
        }
        self.just_updated = should_update;
    }
    pub fn render(&self) {
        self.field.render();
    }
}

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

pub struct Field {
    pixels: Vec<Vec<Pixel>>,
}

impl Field {
    fn print(&self) {
        let us = self.pixels[0]
            .iter()
            .map(|pixel| pixel.u)
            .collect::<Vec<f32>>();
    }
    fn new() -> Self {
        Self {
            // pixels: vec![vec![0f32; 16]]
            pixels: Self::pixels_centered(),
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
    fn update(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            quit();
        }
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
}
