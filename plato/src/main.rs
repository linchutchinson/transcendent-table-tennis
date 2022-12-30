mod ui;

use legion::*;
use macroquad::prelude::*;
use ui::{add_ui_systems_to_schedule, spawn_button, Label, UIContainer, UIRoot, UISize};

#[macroquad::main("Transcendent Table Tennis")]
async fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = build_schedule();

    let mut root_container = UIContainer::empty();

    let c1 = world.push((
        Rect::new(10.0, 10.0, 20.0, 20.0),
        UISize::Grow(1),
        Label::new("Transcendent Table Tennis".to_string(), 32.0),
    ));

    let mut child_container = UIContainer::empty();

    let cc1 = world.push((Rect::new(200.0, 20.0, 10.0, 100.0), UISize::Grow(1)));

    let mut button_container = UIContainer::empty();

    let spacer_1 = world.push((UISize::Grow(1), ()));

    let play_button = spawn_button(&mut world, "Play");
    let quit_button = spawn_button(&mut world, "Quit");
    let spacer_2 = world.push((UISize::Grow(1), ()));

    button_container.add_child(spacer_1);
    button_container.add_child(play_button);
    button_container.add_child(quit_button);
    button_container.add_child(spacer_2);

    let button_section = world.push((
        Rect::new(200.0, 20.0, 10.0, 100.0),
        UISize::Grow(2),
        button_container,
    ));
    let cc3 = world.push((Rect::new(200.0, 20.0, 10.0, 100.0), UISize::Grow(2)));

    child_container.add_child(cc1);
    child_container.add_child(button_section);
    child_container.add_child(cc3);

    let c2 = world.push((
        Rect::new(200.0, 20.0, 10.0, 100.0),
        UISize::Grow(9),
        child_container,
    ));

    root_container.add_child(c1);
    root_container.add_child(c2);

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

