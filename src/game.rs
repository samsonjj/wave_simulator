use std::time::Instant;

use macroquad::prelude::*;
use miniquad::window::quit;

use crate::field::{self, Field, Field1D, Field2D};

#[derive(PartialEq, Eq)]
pub enum GameState {
    Running,
    Paused,
}

pub enum FieldType {
    Field1D,
    Field2D,
}

pub struct Game {
    pub field: Box<dyn Field>,
    pub state: GameState,
    pub step: i32,
    pub just_updated: bool,
    pub start_time: Instant,
}

impl Game {
    pub fn new(field_type: FieldType) -> Game {
        let field: Box<dyn Field> = match field_type {
            FieldType::Field1D => Box::new(Field1D::new()),
            FieldType::Field2D => Box::new(Field2D::new()),
        };
        Game {
            field,
            state: GameState::Paused,
            step: 0,
            just_updated: false,
            start_time: Instant::now(),
        }
    }
    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            quit();
        }
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
        }
        self.just_updated = should_update;
    }
    pub fn render(&self) {
        self.field.render();
    }
}
