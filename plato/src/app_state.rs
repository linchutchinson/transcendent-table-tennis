use legion::system;
use macroquad::{
    prelude::Color,
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};

#[derive(PartialEq, Copy, Clone)]
pub enum AppState {
    MainMenu,
    InGame,
    Quit,
}

pub struct NextState(pub Option<AppState>);

#[system]
pub fn state_transition(
    #[resource] current_state: &mut AppState,
    #[resource] next_state: &mut NextState,
    #[state] progress: &mut f32,
) {
    const TRANSITION_RATE: f32 = 0.1;
    if let Some(next) = next_state.0 {
        if next != *current_state {
            // Transition out
            *progress = (*progress + TRANSITION_RATE).min(1.0);

            let screen_width = screen_width();
            let screen_height = screen_height();
            let color = Color::new(0.0, 0.0, 0.0, *progress);
            draw_rectangle(0.0, 0.0, screen_width, screen_height, color);

            if *progress == 1.0 {
                // Blacked out screen. Swap to next state.
                *current_state = next;
            }
        } else {
            // Transition in
            *progress = (*progress - TRANSITION_RATE).max(0.0);

            if *progress > 0.0 {
                // Draw transition graphic
                let screen_width = screen_width();
                let screen_height = screen_height();
                let color = Color::new(0.0, 0.0, 0.0, *progress);
                draw_rectangle(0.0, 0.0, screen_width, screen_height, color);
            } else {
                // Transition over
                next_state.0 = None;
            }
        }
    }
}
