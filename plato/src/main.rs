use legion::*;
use macroquad::prelude::*;

#[macroquad::main("Transcendent Table Tennis")]
async fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = build_schedule();
    loop {
        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}

fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(draw_main_menu_system())
        .build()
}

#[system]
fn draw_main_menu() {
    clear_background(BLACK);

    draw_title();
    draw_control_buttons();
}

fn draw_title() {
    let screen_width = screen_width();
    const TITLE_FONT_SIZE: u16 = 48;
    const TITLE_TEXT: &str = "Transcendent Table Tennis";
    const TOP_MARGIN: f32 = 32.0;
    let text_dims = measure_text(TITLE_TEXT, None, TITLE_FONT_SIZE, 1.0);
    let text_left = (screen_width / 2.0) - text_dims.width * 0.5;
    draw_text(
        "Transcendent Table Tennis",
        text_left,
        text_dims.height + TOP_MARGIN,
        TITLE_FONT_SIZE as f32,
        WHITE,
    );
}

fn draw_control_buttons() {
    let container_rect = {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let container_height = screen_height * 0.6;
        let container_width = screen_width * 0.5;
        Rect::new(
            screen_width * 0.5 - (container_width * 0.5),
            screen_height * 0.5 - container_height * 0.5,
            container_width,
            container_height,
        )
    };

    draw_rectangle_lines(
        container_rect.x,
        container_rect.y,
        container_rect.w,
        container_rect.h,
        2.0,
        WHITE,
    );

    draw_button(&container_rect, "Play");
}

enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}

fn draw_button(rect: &Rect, text: &str) {
    let x = rect.x;
    let y = rect.y;
    let w = rect.w;
    let h = rect.h;

    let state = {
        let mouse_pos = mouse_position();

        if rect.contains(Vec2::from(mouse_pos)) {
            if is_mouse_button_down(MouseButton::Left) {
                ButtonState::Pressed
            } else {
                ButtonState::Hovered
            }
        } else {
            ButtonState::Normal
        }
    };
    let (btn_color, text_color) = match state {
        ButtonState::Normal => (BLACK, WHITE),
        ButtonState::Hovered => (LIGHTGRAY, BLACK),
        ButtonState::Pressed => (BLACK, GREEN),
    };

    draw_rectangle(x, y, w, h, btn_color);
    draw_centered_text(rect, text, text_color);
}

fn draw_centered_text(rect: &Rect, text: &str, color: Color) {
    const FONT_SIZE: u16 = 32;
    let (x, y) = {
        let text_dims = measure_text(text, None, FONT_SIZE, 1.0);
        let center = rect.center();
        let x = center.x - text_dims.width * 0.5;
        let y = center.y - text_dims.height * 0.5;
        (x, y)
    };

    draw_text(text, x, y, FONT_SIZE as f32, color);
}
