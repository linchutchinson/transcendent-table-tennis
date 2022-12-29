mod ui;

use legion::*;
use macroquad::prelude::*;
use ui::{add_ui_systems_to_schedule, UIContainer, UIRoot, UISize};

#[macroquad::main("Transcendent Table Tennis")]
async fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = build_schedule();

    let mut root_container = UIContainer::empty();

    let c1 = world.push((Rect::new(10.0, 10.0, 20.0, 20.0), UISize::Constant(128.0)));

    let mut child_container = UIContainer::empty();

    let cc1 = world.push((Rect::new(200.0, 20.0, 10.0, 100.0), UISize::Constant(32.0)));
    let cc2 = world.push((Rect::new(200.0, 20.0, 10.0, 100.0), UISize::Grow(1)));
    let cc3 = world.push((Rect::new(200.0, 20.0, 10.0, 100.0), UISize::Constant(16.0)));

    child_container.add_child(cc1);
    child_container.add_child(cc2);
    child_container.add_child(cc3);

    let c2 = world.push((
        Rect::new(200.0, 20.0, 10.0, 100.0),
        UISize::Grow(3),
        child_container,
    ));

    let c3 = world.push((Rect::new(200.0, 20.0, 10.0, 100.0), UISize::Grow(1)));
    let c4 = world.push((Rect::new(10.0, 300.0, 400.0, 18.0), UISize::Constant(64.0)));

    root_container.add_child(c1);
    root_container.add_child(c2);
    root_container.add_child(c3);
    root_container.add_child(c4);

    world.push((UIRoot, root_container, Rect::new(0.0, 0.0, 0.0, 0.0)));
    loop {
        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}

fn build_schedule() -> Schedule {
    let mut builder = Schedule::builder();
    builder.add_thread_local(clear_screen_system());
    builder.flush();
    add_ui_systems_to_schedule(&mut builder);
    builder.build()
}

#[system]
fn clear_screen() {
    clear_background(BLACK);
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

    let inner_rect = {
        const MARGIN_SIZE: f32 = 4.0;
        let x = container_rect.x + MARGIN_SIZE;
        let y = container_rect.y + MARGIN_SIZE;
        let h = container_rect.h - (MARGIN_SIZE * 2.0);
        let w = container_rect.w - (MARGIN_SIZE * 2.0);

        Rect::new(x, y, w, h)
    };

    let (first_rect, second_rect, third_rect) = {
        const GAP: f32 = 4.0;
        let w = inner_rect.w;
        let h = (inner_rect.h - GAP * 2.0) / 3.0;
        let x = inner_rect.x;
        let (y1, y2, y3) = {
            let y1 = inner_rect.y;
            let y2 = y1 + h + GAP;
            let y3 = y2 + h + GAP;
            (y1, y2, y3)
        };

        let first = Rect::new(x, y1, w, h);
        let second = Rect::new(x, y2, w, h);
        let third = Rect::new(x, y3, w, h);
        (first, second, third)
    };

    draw_button(&first_rect, "Play", ButtonState::Normal);
    draw_button(&second_rect, "Options", ButtonState::Normal);
    draw_button(&third_rect, "Quit", ButtonState::Normal);
}

enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}

fn draw_button(rect: &Rect, text: &str, state: ButtonState) {
    let x = rect.x;
    let y = rect.y;
    let w = rect.w;
    let h = rect.h;

    let (btn_color, text_color) = match state {
        ButtonState::Normal => (Color::new(0.2, 0.2, 0.2, 1.0), WHITE),
        ButtonState::Hovered => (Color::new(0.3, 0.3, 0.3, 1.0), BLACK),
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
        let y = center.y;
        (x, y)
    };

    draw_text(text, x, y, FONT_SIZE as f32, color);
}
