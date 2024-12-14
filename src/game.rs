use std::time::{Duration, Instant};

use macroquad::prelude::*;
use miniquad::window::quit;

use crate::field::{self, Field, Field1D, Field2D};

const UPDATES_PER_FRAME: u32 = 1;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Running,
    Paused,
}

#[derive(Copy, Clone)]
pub enum FieldType {
    Field1D,
    Field2D,
}

pub struct Game {
    pub field: Box<dyn Field>,
    pub field_type: FieldType,
    pub state: GameState,
    pub step: i32,
    pub just_updated: bool,
    pub start_time: Instant,
    pub rendering_duration: Duration,
    pub update_duration: Duration,
}

impl Game {
    pub fn new(field_type: FieldType) -> Game {
        Game {
            field: Self::init_field(field_type, false),
            field_type,
            state: GameState::Paused,
            step: 0,
            just_updated: false,
            start_time: Instant::now(),
            rendering_duration: Duration::ZERO,
            update_duration: Duration::ZERO,
        }
    }

    pub fn init_field(field_type: FieldType, vectorized: bool) -> Box<dyn Field> {
        match field_type {
            FieldType::Field1D => Box::new(Field1D::new()),
            FieldType::Field2D => Box::new(Field2D::new(vectorized)),
        }
    }

    pub fn update(&mut self) {
        let start = Instant::now();
        if is_key_pressed(KeyCode::Escape) {
            quit();
        }
        if is_key_pressed(KeyCode::Space) {
            self.state = match self.state {
                GameState::Running => GameState::Paused,
                GameState::Paused => GameState::Running,
            }
        }
        if is_key_pressed(KeyCode::R) {
            self.field = Self::init_field(self.field_type, false);
        } else if is_key_pressed(KeyCode::T) {
            self.field = Self::init_field(self.field_type, true);
        }
        let should_update = if self.state == GameState::Running {
            true
        } else if is_key_pressed(KeyCode::Period) {
            true
        } else {
            false
        };
        if should_update {
            for i in 0..UPDATES_PER_FRAME {
                self.field.update();
                self.step += 1;
            }
        }
        self.just_updated = should_update;
        self.update_duration = start.elapsed();
    }
    pub fn render(&mut self) {
        let start = Instant::now();
        self.field.render();
        self.rendering_duration = start.elapsed();
    }
}
