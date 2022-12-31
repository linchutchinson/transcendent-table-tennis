use legion::{Schedule, system};
use macroquad::{window::clear_background, prelude::BLACK};

use crate::ui::add_ui_systems_to_schedule;


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
