use std::time::{Duration, Instant};

use macroquad::prelude::*;

use crate::game::Game;
use crate::monitor::Monitor;

pub struct Debugger {
    text: Vec<String>,
    last_frame_instant: Instant,
    frame_time_monitor: Monitor,
    rendering_duration: Duration,
    update_duration: Duration,
}

const WIDTH: f32 = 300.0;

impl Debugger {
    pub fn new() -> Self {
        Self {
            text: vec![],
            last_frame_instant: Instant::now(),
            frame_time_monitor: Monitor::new(),
            rendering_duration: Duration::ZERO,
            update_duration: Duration::ZERO,
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

const FRAME_RATE_WINDOW: usize = 120;

impl crate::observer::Observer for Debugger {
    fn update(&mut self, game: &crate::game::Game) {
        if game.just_updated || self.text.is_empty() {
            self.rendering_duration += game.rendering_duration;
            self.update_duration += game.update_duration;
            self.frame_time_monitor.inc(self.last_frame_instant.elapsed().as_secs_f32());

            if game.step % 60 == 0 {
                self.last_frame_instant = Instant::now();
                self.rendering_duration = Duration::ZERO;
                self.update_duration = Duration::ZERO;
            }
            self.clear();
            self.println(format!("Step: {}", game.step).as_str());
            self.println(format!("Time Elapsed: {:.2}s", game.start_time.elapsed().as_millis() as f32 * 0.001).as_str());
            self.println(format!("Framerate: {:.0?}/s", 1. / self.frame_time_monitor.display_val).as_str());
            self.println(format!("Render Time: {:.2}ms", self.rendering_duration.as_micros() as f32 * 0.001 / (game.step % 60) as f32).as_str());
            self.println(format!("Update Time: {:.2}ms", self.update_duration.as_micros() as f32 * 0.001 / (game.step % 60) as f32).as_str());
            
            self.last_frame_instant = Instant::now();
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
