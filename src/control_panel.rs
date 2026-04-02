use macroquad::{
    color::{BLACK, DARKGRAY, LIGHTGRAY, WHITE},
    shapes::draw_rectangle,
    text::{draw_text, measure_text},
    window::{screen_height, screen_width},
};

use crate::{game::GameState, observer::Observer};

const BAR_HEIGHT: f32 = 28.0;
const FONT_SIZE: f32 = 16.0;
const KEY_PADDING: f32 = 10.0;
const ITEM_GAP: f32 = 20.0;

const SHORTCUTS: &[(&str, &str)] = &[
    ("Space", "Play/Pause"),
    ("R", "Reset"),
    ("T", "Reset (vec)"),
    ("N", "Next init"),
    (".", "Step"),
    ("Esc", "Quit"),
];

pub struct ControlPanel {}

impl Observer for ControlPanel {
    fn render(&self, game: &crate::game::Game) {
        let bar_y = screen_height() - BAR_HEIGHT;
        draw_rectangle(0.0, bar_y, screen_width(), BAR_HEIGHT, LIGHTGRAY);

        let text_y = bar_y + BAR_HEIGHT / 2.0 + FONT_SIZE / 2.0 - 2.0;

        // Shortcuts on the left
        let mut x = KEY_PADDING;
        for (key, label) in SHORTCUTS {
            let key_dims = measure_text(key, None, FONT_SIZE as u16, 1.0);
            let label_dims = measure_text(label, None, FONT_SIZE as u16, 1.0);

            let badge_w = key_dims.width + 8.0;
            let badge_h = FONT_SIZE + 4.0;
            let badge_y = bar_y + (BAR_HEIGHT - badge_h) / 2.0;
            draw_rectangle(x, badge_y, badge_w, badge_h, DARKGRAY);
            draw_text(key, x + 4.0, text_y, FONT_SIZE, WHITE);
            x += badge_w + 5.0;

            draw_text(label, x, text_y, FONT_SIZE, BLACK);
            x += label_dims.width + ITEM_GAP;
        }

        // Status on the right
        let status = format!(
            "{} | init: {}",
            match game.state {
                GameState::Running => "Running",
                GameState::Paused => "Paused",
            },
            game.init_variant.label()
        );
        let status_dims = measure_text(&status, None, FONT_SIZE as u16, 1.0);
        draw_text(&status, screen_width() - status_dims.width - KEY_PADDING, text_y, FONT_SIZE, BLACK);
    }

    fn update(&mut self, _game: &crate::game::Game) {}
}

impl ControlPanel {
    pub fn new() -> Self {
        Self {}
    }
}
