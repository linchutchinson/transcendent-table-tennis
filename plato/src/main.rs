use macroquad::window::next_frame;

#[macroquad::main("Transcendent Table Tennis")]
async fn main() {
    let mut app = plato::Application::with_starting_state(plato::AppState::InGame);
    while app.keep_running {
        app.handle_window_input();
        app.tick();

        next_frame().await
    }
}
