use legion::{system, systems::CommandBuffer, Resources, Schedule, World};
use macroquad::{
    prelude::{Rect, GREEN, BLACK, GRAY, WHITE, Vec2},
    window::{screen_height, screen_width, clear_background}, shapes::draw_rectangle, time::get_time,
};

pub fn build_game_schedule() -> Schedule {
    let mut builder = Schedule::builder();

    builder.add_system(recalculate_play_area_system()).flush();
    builder.add_thread_local(draw_bg_and_play_area_system()).add_thread_local(draw_paddles_system());

    builder.build()
}

/// The game is simulated at 1920x1080, but on the client side we dynamically adjust the
/// play area based on the resolution of the window. The PlayArea rect is referenced by drawing
/// functions to know what space they can use and what ratios they should use to scale draw calls.
struct PlayArea(Rect);

impl PlayArea {
    fn simulated_size_to_displayed_size(&self, sim_size: f32) -> f32 {
        //TODO: Get the 1920 here from the common lib as a const in case
        // we change the simulated dimensions.
        let ratio = self.0.w / 1920.0;

        sim_size * ratio
    }

    fn simulated_pos_to_displayed_pos(&self, sim_pos: Vec2) -> Vec2 {
        //TODO: Get the 1920 here from the common lib as a const in case
        // we change the simulated dimensions.
        let ratio = self.0.w / 1920.0;

        let scaled = sim_pos * ratio;
        let adjusted_x = scaled.x + self.0.x;
        let adjusted_y = scaled.y + self.0.y;

        Vec2::new(adjusted_x, adjusted_y)
    }
}

#[system]
fn draw_bg_and_play_area(#[resource] play_area: &PlayArea) {
    clear_background(GRAY);

    let play_rect = play_area.0;
    draw_rectangle(play_rect.x, play_rect.y, play_rect.w, play_rect.h, BLACK);
}

pub fn initialize_game_state(world: &mut World, resources: &mut Resources) {
    let play_area = PlayArea(calculate_play_area());
    resources.insert(play_area);
}

#[system]
fn recalculate_play_area(#[resource] play_area: &mut PlayArea) {
    play_area.0 = calculate_play_area();
}

fn calculate_play_area() -> Rect {
    let screen_height = screen_height();
    let screen_width = screen_width();

    // 16:9 ratio for the play area
    let ratio = (9.0 / 16.0);
    let play_area_height = screen_width * ratio;

    //FIXME: This code doesn't take into account resolutions where the screen is wider than a 16:9.
    // As a result, wider monitors will have the top and bottom parts of the game area cut off.

    let play_area_y = (screen_height * 0.5) - (play_area_height * 0.5);

    Rect::new(0.0, play_area_y, screen_width, play_area_height)
}

#[system]
fn draw_paddles(#[resource] play_area: &PlayArea) {
    //FIXME: These numbers are all made up until we actually have a game.
    const PADDLE_SIM_WIDTH: f32 = 64.0;
    const PADDLE_SIM_HEIGHT: f32 = 128.0;
    let paddle_sim_x = 64.0;
    let paddle_sim_y = (1080.0 * 0.5) + (get_time().sin() * (1080.0 * 0.5));

    let paddle_display_width = play_area.simulated_size_to_displayed_size(PADDLE_SIM_WIDTH);
    let paddle_display_height = play_area.simulated_size_to_displayed_size(PADDLE_SIM_HEIGHT);

    let paddle_display_pos = play_area.simulated_pos_to_displayed_pos(Vec2::new(paddle_sim_x, paddle_sim_y as f32));

    let paddle_top = paddle_display_pos.x - paddle_display_width * 0.5;
    let paddle_left = paddle_display_pos.y - paddle_display_height * 0.5;

    draw_rectangle(paddle_top, paddle_left, paddle_display_width, paddle_display_height, WHITE);
}
