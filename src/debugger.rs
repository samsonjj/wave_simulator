use std::time::Instant;

use macroquad::prelude::*;

use crate::game::Game;

pub struct Debugger {
    enabled: bool,
    text: Vec<String>,
    last_frame_instant: Instant,
    frame_rate: f32,
}

const WIDTH: f32 = 300.0;

impl Debugger {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            text: vec![],
            last_frame_instant: Instant::now(),
            frame_rate: 0.0,
        }
    }
    pub fn clear(&mut self) {
        self.text.clear();
    }
    pub fn println(&mut self, s: &str) {
        self.text.push(s.to_string());
        self.text.push("\n".to_string());
    }
    fn top_left() -> Vec2 {
        vec2(screen_width() - WIDTH, 0.0)
    }
}

impl crate::observer::Observer for Debugger {
    fn update(&mut self, game: &crate::game::Game) {
        if game.just_updated || self.text.is_empty() {
            if game.step % 60 == 0 {
                self.frame_rate =
                    60.0 / self.last_frame_instant.elapsed().as_millis() as f32 * 1000.0;
                self.last_frame_instant = Instant::now();
            }
            self.clear();
            self.println(format!("Step: {}", game.step).as_str());
            self.println(format!("Time Elapsed: {:?}", game.start_time.elapsed()).as_str());
            self.println(format!("Framerate: {:?}", self.frame_rate).as_str());
        }
    }
    fn render(&self, _game: &Game) {
        const LEFT_MARGIN: f32 = 15.0;
        const TOP_MARGIN: f32 = 30.0;
        const LINE_HEIGHT: f32 = 10.0;
        draw_rectangle(Self::top_left().x, 0.0, WIDTH, screen_height(), GRAY);
        draw_rectangle(
            Self::top_left().x + 5.0,
            5.0,
            WIDTH - 10.,
            screen_height() - 10.0,
            LIGHTGRAY,
        );
        for (i, text) in self.text.iter().enumerate() {
            draw_text(
                text,
                Self::top_left().x + LEFT_MARGIN,
                TOP_MARGIN + i as f32 * LINE_HEIGHT as f32,
                20.0,
                BLACK,
            );
        }
    }
}
