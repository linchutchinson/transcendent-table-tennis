mod colors;
mod math;
use colors::*;
use macroquad::{
    prelude::{is_key_down, is_quit_requested, KeyCode},
    shapes::{draw_circle, draw_rectangle},
    text::draw_text,
    window::{clear_background, screen_height, screen_width},
};
use math::{Rect, Vec2};

// TODO: These consts are gonna move.
const PADDLE_HEIGHT: f32 = 200.0;
const PADDLE_WIDTH: f32 = 50.0;
const PLAYER_X: f32 = 100.0;
const BOT_X: f32 = 1820.0;
const PADDLE_SPEED: f32 = 8.0;

const BALL_SPEED: f32 = 10.0;
const BALL_RADIUS: f32 = 24.0;

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

    fn game_scale_to_screen_scale(&self, val: f32) -> f32 {
        let ratio = self.0.size.x / 1920.0;

        val * ratio
    }
}

pub struct Application {
    pub is_running: bool,
    player_y: f32,
    player_dir: f32,
    bot_y: f32,
    ball_pos: Vec2,
    ball_velocity: Vec2,
    player_score: i32,
    bot_score: i32,
}

impl Application {
    pub fn new() -> Self {
        Self {
            is_running: true,
            player_y: 400.0,
            player_dir: 0.0,
            bot_y: 400.0,
            ball_pos: Vec2::new(200.0, 200.0),
            //TODO This needs to be properly normalized.
            ball_velocity: Vec2::new(1.0, 1.0) * BALL_SPEED,
            player_score: 0,
            bot_score: 0,
        }
    }

    pub fn handle_input(&mut self) {
        let pressing_up = is_key_down(KeyCode::W);
        let pressing_down = is_key_down(KeyCode::S);

        let dir = match (pressing_up, pressing_down) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        self.player_dir = dir;
    }

    pub fn tick(&mut self) {
        if is_quit_requested() {
            self.is_running = false;
        }

        let bot_dir = if self.ball_pos.y < self.bot_y {
            -1.0
        } else if self.ball_pos.y > self.bot_y {
            1.0
        } else {
            0.0
        };

        self.bot_y += bot_dir * PADDLE_SPEED * 0.8;

        self.player_y += self.player_dir * PADDLE_SPEED;

        self.ball_pos += self.ball_velocity;

        // Top/Bottom Boundary Collisions
        if self.ball_pos.y >= 1080.0 - BALL_RADIUS || self.ball_pos.y <= BALL_RADIUS {
            self.ball_velocity.y *= -1.0;
        }

        // Paddle Collisions
        if self.ball_velocity.x > 0.0 {
            // Bot Paddle
            if self.ball_pos.x < BOT_X && self.ball_pos.x + BALL_RADIUS >= BOT_X {
                let dist_to_paddle = (self.ball_pos.y - self.bot_y).abs();
                let hits_paddle = dist_to_paddle < (PADDLE_HEIGHT * 0.5) + BALL_RADIUS;

                if hits_paddle {
                    self.ball_velocity.x *= -1.0;
                }
            }
        } else {
            // Player Paddle
            if self.ball_pos.x > PLAYER_X && self.ball_pos.x - BALL_RADIUS <= PLAYER_X {
                let dist_to_paddle = (self.ball_pos.y - self.player_y).abs();
                let hits_paddle = dist_to_paddle < PADDLE_HEIGHT + BALL_RADIUS;

                if hits_paddle {
                    self.ball_velocity.x *= -1.0;
                }
            }
        }

        if self.ball_pos.x >= 1920.0 + BALL_RADIUS || self.ball_pos.x <= -BALL_RADIUS {
            // Ball has scored. Reset Position.
            self.ball_pos = Vec2::new(1920.0, 1080.0) * 0.5;

            if self.ball_pos.x < 0.0 {
                self.bot_score += 1;
            } else {
                self.player_score += 1;
            }
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

        let scaled_paddle_height = play_area.game_scale_to_screen_scale(PADDLE_HEIGHT);
        let scaled_paddle_width = play_area.game_scale_to_screen_scale(PADDLE_WIDTH);
        let scaled_paddle_size = Vec2::new(scaled_paddle_width, scaled_paddle_height);

        fn draw_paddle_from_center(pos: Vec2, size: Vec2) {
            let left = pos.x - (size.x * 0.5);
            let top = pos.y - (size.y * 0.5);

            draw_rectangle(left, top, size.x, size.y, PADDLE_COLOR);
        }

        // NOTE We shift the player and bot paddles so that they line up exactly with the boundary lines used
        // for collision checking.
        let player_screen_pos = play_area
            .game_pos_to_screen_space(Vec2::new(PLAYER_X - PADDLE_WIDTH * 0.5, self.player_y));
        draw_paddle_from_center(player_screen_pos, scaled_paddle_size);

        let bot_screen_pos =
            play_area.game_pos_to_screen_space(Vec2::new(BOT_X + PADDLE_WIDTH * 0.5, self.bot_y));
        draw_paddle_from_center(bot_screen_pos, scaled_paddle_size);

        let ball_screen_pos = play_area.game_pos_to_screen_space(self.ball_pos);
        let scaled_ball_radius = play_area.game_scale_to_screen_scale(BALL_RADIUS);
        draw_circle(
            ball_screen_pos.x,
            ball_screen_pos.y,
            scaled_ball_radius,
            BALL_COLOR,
        );

        draw_text(
            &format!("{} | {}", self.player_score, self.bot_score),
            0.0,
            128.0,
            64.0,
            TEXT_COLOR,
        );
    }
}
