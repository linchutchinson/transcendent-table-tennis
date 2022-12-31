use macroquad::{prelude::Vec2, window::next_frame};

#[macroquad::test]
async fn test_smoke() {
    let mut app = plato::Application::new();

    assert!(app.keep_running);
    for _ in 0..100 {
        app.tick();
    }
    assert!(app.keep_running);
}

#[macroquad::test]
async fn test_quit_game() {
    let mut app = plato::Application::new();
    app.tick();
    let quit_rect_opt = app.find_text_rect("Quit");
    if let Some(quit_rect) = quit_rect_opt {
        for _ in 0..10 {
            app.tick();
            next_frame().await
        }
        let running_at_start = app.keep_running;
        app.click_pos(Vec2::new(quit_rect.x, quit_rect.y));

        for _ in 0..10 {
            app.tick();
            next_frame().await
        }
        let running_at_end = app.keep_running;

        drop(app);
        assert!(running_at_start);
        assert!(!running_at_end);
    } else {
        drop(app);
        assert!(false, "Expect to find a button labeled quit on main menu.");
    }
}
