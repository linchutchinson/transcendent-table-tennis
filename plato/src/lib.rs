mod app;
mod app_state;
mod input;
mod ui;

pub use app::Application;
use macroquad::window::next_frame;

pub async fn run() {
    let mut app = Application::new();
    while app.keep_running {
        app.handle_window_input();
        app.tick();

        next_frame().await
    }
}
