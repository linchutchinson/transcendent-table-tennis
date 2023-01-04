mod colors;
mod math;
use colors::*;
use macroquad::{
    prelude::is_quit_requested,
    shapes::{draw_circle, draw_rectangle},
    window::clear_background,
};
use math::Vec2;

pub struct Application {
    pub is_running: bool,
    player_y: f32,
    bot_y: f32,
    ball_pos: Vec2,
}

impl Application {
    pub fn new() -> Self {
        Self {
            is_running: true,
            player_y: 400.0,
            bot_y: 400.0,
            ball_pos: Vec2::new(200.0, 200.0),
        }
    }

    pub fn handle_input(&mut self) {}

    pub fn tick(&mut self) {
        if is_quit_requested() {
            self.is_running = false;
        }
    }

    pub fn render(&mut self) {
        clear_background(BACKGROUND_COLOR);

        //TODO: These consts are gonna move.
        const PADDLE_HEIGHT: f32 = 100.0;
        const PADDLE_WIDTH: f32 = 50.0;
        const PLAYER_X: f32 = 100.0;
        const BOT_X: f32 = 600.0;

        fn draw_paddle_from_center(center_x: f32, center_y: f32) {
            let left = center_x - (PADDLE_WIDTH * 0.5);
            let top = center_y - (PADDLE_HEIGHT * 0.5);

            draw_rectangle(left, top, PADDLE_WIDTH, PADDLE_HEIGHT, PADDLE_COLOR);
        }

        draw_paddle_from_center(PLAYER_X, self.player_y);
        draw_paddle_from_center(BOT_X, self.bot_y);

        const BALL_RADIUS: f32 = 24.0;
        draw_circle(self.ball_pos.x, self.ball_pos.y, BALL_RADIUS, BALL_COLOR);
    }
}
