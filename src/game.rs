use std::time::{Duration, Instant};

use macroquad::prelude::*;
use miniquad::window::quit;

use crate::field::{Field, Field1D, Field1DInit, Field2D, Field2DInit};

const UPDATES_PER_FRAME: u32 = 10;

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

#[derive(Copy, Clone)]
pub enum InitVariant {
    Field1D(Field1DInit),
    Field2D(Field2DInit),
}

impl InitVariant {
    pub fn cycle(self) -> Self {
        match self {
            Self::Field1D(i) => Self::Field1D(i.cycle()),
            Self::Field2D(i) => Self::Field2D(i.cycle()),
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Field1D(i) => i.label(),
            Self::Field2D(i) => i.label(),
        }
    }
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
    pub init_variant: InitVariant,
}

impl Game {
    pub fn new(field_type: FieldType) -> Game {
        let init_variant = match field_type {
            FieldType::Field1D => InitVariant::Field1D(Field1DInit::AtEnd),
            FieldType::Field2D => InitVariant::Field2D(Field2DInit::Standing),
        };
        Game {
            field: Self::init_field(field_type, false, init_variant),
            field_type,
            state: GameState::Paused,
            step: 0,
            just_updated: false,
            start_time: Instant::now(),
            rendering_duration: Duration::ZERO,
            update_duration: Duration::ZERO,
            init_variant,
        }
    }

    pub fn init_field(field_type: FieldType, vectorized: bool, init_variant: InitVariant) -> Box<dyn Field> {
        match (field_type, init_variant) {
            (FieldType::Field1D, InitVariant::Field1D(i)) => Box::new(Field1D::new(i)),
            (FieldType::Field2D, InitVariant::Field2D(i)) => Box::new(Field2D::new(vectorized, i)),
            // fallback (shouldn't happen)
            (FieldType::Field1D, _) => Box::new(Field1D::new(Field1DInit::AtEnd)),
            (FieldType::Field2D, _) => Box::new(Field2D::new(vectorized, Field2DInit::Zero)),
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
        if is_key_pressed(KeyCode::N) {
            self.init_variant = self.init_variant.cycle();
            self.field = Self::init_field(self.field_type, false, self.init_variant);
            self.step = 0;
        } else if is_key_pressed(KeyCode::R) {
            self.field = Self::init_field(self.field_type, false, self.init_variant);
            self.step = 0;
        } else if is_key_pressed(KeyCode::T) {
            self.field = Self::init_field(self.field_type, true, self.init_variant);
            self.step = 0;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            // self.field.(mouse_position());
        }

        let should_update = if self.state == GameState::Running {
            true
        } else if is_key_pressed(KeyCode::Period) {
            true
        } else {
            false
        };
        if should_update {
            for _ in 0..UPDATES_PER_FRAME {
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
