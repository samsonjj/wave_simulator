use macroquad::{
    color::BLACK,
    math::{vec2, Vec2},
    text::draw_text,
};

use crate::{game::GameState, observer::Observer};

pub struct ControlPanel {}

impl Observer for ControlPanel {
    fn render(&self, game: &crate::game::Game) {
        draw_text(
            format!(
                "{}",
                match game.state {
                    GameState::Running => "Running",
                    GameState::Paused => "Paused",
                }
            )
            .as_str(),
            Self::top_left().x,
            Self::top_left().y,
            20.0,
            BLACK,
        );
    }

    fn update(&mut self, game: &crate::game::Game) {}
}
impl ControlPanel {
    pub fn new() -> Self {
        Self {}
    }
    fn top_left() -> Vec2 {
        vec2(15.0, 150.0)
    }
}
