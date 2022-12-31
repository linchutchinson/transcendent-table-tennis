use legion::{system, IntoQuery, Resources, Schedule, World};
pub use macroquad;
use macroquad::{
    prelude::{
        is_mouse_button_down, mouse_position, MouseButton as MquadMouseButton, Rect, Vec2, BLACK,
    },
    window::clear_background,
};

use crate::{
    app_state::AppState,
    input::{Input, MouseButton, MousePosition},
    ui::{
        add_ui_systems_to_schedule, spawn_button, Label, QuitButton, Text, UIContainer, UIRoot,
        UISize,
    },
};

pub struct Application {
    pub world: World,
    pub resources: Resources,
    pub schedule: Schedule,
    pub keep_running: bool,
}

impl Application {
    pub fn new() -> Self {
        let mut base = Application::empty();

        // Build Main Menu UI
        let mut root_container = UIContainer::empty();

        let c1 = base.world.push((
            Rect::new(10.0, 10.0, 20.0, 20.0),
            UISize::Grow(1),
            Label::new("Transcendent Table Tennis".to_string(), 32.0),
        ));

        let mut child_container = UIContainer::empty();

        let cc1 = base
            .world
            .push((Rect::new(200.0, 20.0, 10.0, 100.0), UISize::Grow(1)));

        let mut button_container = UIContainer::empty();

        let spacer_1 = base.world.push((UISize::Grow(1), ()));

        let play_button = spawn_button(&mut base.world, "Play");
        let quit_button = spawn_button(&mut base.world, "Quit");

        {
            let mut quit_btn_entry = base.world.entry(quit_button).unwrap();
            quit_btn_entry.add_component(QuitButton);
        }

        let spacer_2 = base.world.push((UISize::Grow(1), ()));

        button_container.add_child(spacer_1);
        button_container.add_child(play_button);
        button_container.add_child(quit_button);
        button_container.add_child(spacer_2);

        let button_section = base.world.push((
            Rect::new(200.0, 20.0, 10.0, 100.0),
            UISize::Grow(2),
            button_container,
        ));
        let cc3 = base
            .world
            .push((Rect::new(200.0, 20.0, 10.0, 100.0), UISize::Grow(2)));

        child_container.add_child(cc1);
        child_container.add_child(button_section);
        child_container.add_child(cc3);

        let c2 = base.world.push((
            Rect::new(200.0, 20.0, 10.0, 100.0),
            UISize::Grow(9),
            child_container,
        ));

        root_container.add_child(c1);
        root_container.add_child(c2);

        base.world
            .push((UIRoot, root_container, Rect::new(0.0, 0.0, 0.0, 0.0)));

        base
    }

    pub fn empty() -> Self {
        let world = World::default();
        let mut resources = Resources::default();
        let schedule = build_schedule();

        resources.insert(AppState::Run);
        resources.insert(MousePosition(Vec2::new(0.0, 0.0)));
        resources.insert::<Input<MouseButton>>(Input::default());

        Self {
            world,
            resources,
            schedule,
            keep_running: true,
        }
    }

    pub fn tick(&mut self) {
        let app_state = self.resources.get::<AppState>().unwrap();
        match *app_state {
            AppState::Run => {
                drop(app_state);
                self.schedule.execute(&mut self.world, &mut self.resources);
            }
            AppState::Quit => {
                self.keep_running = false;
            }
        }

        // Clear any just_pressed mouse buttons
        let mut mouse_btns = self.resources.get_mut::<Input<MouseButton>>().unwrap();
        mouse_btns.tick_frame();
        drop(mouse_btns);
    }

    pub fn handle_window_input(&mut self) {
        let mouse_pos = mouse_position();
        self.resources
            .insert(MousePosition(Vec2::new(mouse_pos.0, mouse_pos.1)));

        let mut mouse_btns = self.resources.get_mut::<Input<MouseButton>>().unwrap();

        if is_mouse_button_down(MquadMouseButton::Left) {
            mouse_btns.press(MouseButton::Left);
        } else {
            mouse_btns.release(MouseButton::Left);
        }

        if is_mouse_button_down(MquadMouseButton::Right) {
            mouse_btns.press(MouseButton::Right);
        } else {
            mouse_btns.release(MouseButton::Right);
        }

        if is_mouse_button_down(MquadMouseButton::Middle) {
            mouse_btns.press(MouseButton::Middle);
        } else {
            mouse_btns.release(MouseButton::Middle);
        }
    }

    pub fn find_text_rect(&self, text: &str) -> Option<Rect> {
        let mut query = <(&Text, &Rect)>::query();
        if let Some((_, rect)) = query
            .iter(&self.world)
            .filter(|(label, _)| label.0 == text)
            .next()
        {
            Some(*rect)
        } else {
            None
        }
    }

    pub fn click_pos(&mut self, pos: Vec2) {
        self.resources.insert(MousePosition(pos));

        let mut mouse_btns = self.resources.get_mut::<Input<MouseButton>>().unwrap();
        mouse_btns.press(MouseButton::Left);
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
