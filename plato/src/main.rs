use std::time::Instant;

use macroquad::window::next_frame;
use plato::Application;

const TICKS_PER_SECOND: usize = 60;
const SECS_PER_TICK: f32 = 1.0 / TICKS_PER_SECOND as f32;

#[macroquad::main("Transcendent Table Tennis")]
async fn main() {
    let mut app = Application::new();

    let mut elapsed = 0.0;
    while app.is_running {
        let frame_start = Instant::now();

        app.handle_input();

        while elapsed >= SECS_PER_TICK {
            app.tick();
            elapsed -= SECS_PER_TICK;
        }

        app.render();
        next_frame().await;

        elapsed += frame_start.elapsed().as_secs_f32();
    }
}
