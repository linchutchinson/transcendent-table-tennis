use legion::{system, Schedule, World};
use macroquad::{
    prelude::{Rect, BLACK},
    window::clear_background,
};

use crate::{
    app_state::AppState,
    ui::{
        add_ui_systems_to_schedule, spawn_button, Label, StateChangeButton, UIContainer, UIRoot,
        UISize,
    },
};

pub fn initialize_main_menu_entities(world: &mut World) {
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

    let multiplayer_button = spawn_button(world, "Play Online");
    let quit_button = spawn_button(world, "Quit");

    {
        let mut quit_btn_entry = world.entry(quit_button).unwrap();
        quit_btn_entry.add_component(StateChangeButton(AppState::Quit));
    }

    let spacer_2 = world.push((UISize::Grow(1), ()));

    button_container.add_child(spacer_1);

    #[cfg(feature = "singleplayer")]
    {
        let singleplayer_button = spawn_button(world, "Play Solo");
        button_container.add_child(singleplayer_button);

        {
            let mut splayer_btn_entry = world.entry(singleplayer_button).unwrap();
            splayer_btn_entry.add_component(StateChangeButton(AppState::InGame));
        }
    }

    button_container.add_child(multiplayer_button);
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
}

pub fn build_title_menu_schedule() -> Schedule {
    let mut builder = Schedule::builder();
    builder.add_thread_local(clear_screen_system());
    builder.flush();
    add_ui_systems_to_schedule(&mut builder);
    builder.add_thread_local(crate::app_state::state_transition_system(0.0));
    builder.build()
}

#[system]
fn clear_screen() {
    clear_background(BLACK);
}
