mod colors;
mod math;
use colors::*;
use macroquad::{
    prelude::is_quit_requested,
    shapes::{draw_circle, draw_rectangle},
    window::{clear_background, screen_height, screen_width},
};
use math::{Rect, Vec2};

struct PlayArea(Rect);

impl PlayArea {
    fn calculate() -> Self {
        let max_width = screen_width();
        let max_height = screen_height();
        const RATIO: f32 = 9.0 / 16.0;

        let desired_height = max_width * RATIO;

        fn calculate_pos(size: &Vec2) -> Vec2 {
            let max_width = screen_width();
            let max_height = screen_height();
            let x = if size.x == max_width {
                0.0
            } else {
                max_width * 0.5 - size.x * 0.5
            };

            let y = if size.y == max_height {
                0.0
            } else {
                max_height * 0.5 - size.y * 0.5
            };
            Vec2::new(x, y)
        }

        if desired_height > max_height {
            let width = max_height / RATIO;
            let size = Vec2::new(width, max_height);
            let position = calculate_pos(&size);
            Self(Rect::new(position, size))
        } else {
            let size = Vec2::new(max_width, desired_height);
            let position = calculate_pos(&size);
            Self(Rect::new(position, size))
        }
    }

    fn game_pos_to_screen_space(&self, pos: Vec2) -> Vec2 {
        let game_rect = Rect::new(Vec2::ZERO, Vec2::new(1920.0, 1080.0));

        game_rect.project_point(&self.0, pos)
    }
}

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

        let play_area = PlayArea::calculate();

        draw_rectangle(
            play_area.0.position.x,
            play_area.0.position.y,
            play_area.0.size.x,
            play_area.0.size.y,
            PLAY_AREA_COLOR,
        );

        //TODO: These consts are gonna move.
        const PADDLE_HEIGHT: f32 = 100.0;
        const PADDLE_WIDTH: f32 = 50.0;
        const PLAYER_X: f32 = 100.0;
        const BOT_X: f32 = 600.0;

        fn draw_paddle_from_center(pos: Vec2) {
            let left = pos.x - (PADDLE_WIDTH * 0.5);
            let top = pos.y - (PADDLE_HEIGHT * 0.5);

            draw_rectangle(left, top, PADDLE_WIDTH, PADDLE_HEIGHT, PADDLE_COLOR);
        }

        let player_screen_pos =
            play_area.game_pos_to_screen_space(Vec2::new(PLAYER_X, self.player_y));
        draw_paddle_from_center(player_screen_pos);

        let bot_screen_pos = play_area.game_pos_to_screen_space(Vec2::new(BOT_X, self.bot_y));
        draw_paddle_from_center(bot_screen_pos);

        const BALL_RADIUS: f32 = 24.0;
        draw_circle(self.ball_pos.x, self.ball_pos.y, BALL_RADIUS, BALL_COLOR);
    }
}
