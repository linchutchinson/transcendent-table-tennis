use legion::{system, IntoQuery, Resources, Schedule, World};
pub use macroquad;
use macroquad::{
    prelude::{
        is_mouse_button_down, mouse_position, MouseButton as MquadMouseButton, Rect, Vec2, BLACK,
    },
    window::clear_background,
};

use crate::{
    app_state::{AppState, NextState},
    input::{Input, MouseButton, MousePosition},
    title_menu::{build_title_menu_schedule, initialize_main_menu_entities},
    ui::{
        add_ui_systems_to_schedule, spawn_button, Label, StateChangeButton, Text, UIContainer,
        UIRoot, UISize,
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
        initialize_main_menu_entities(&mut base.world);

        base
    }

    pub fn empty() -> Self {
        let world = World::default();
        let mut resources = Resources::default();
        let schedule = build_title_menu_schedule();

        resources.insert(AppState::MainMenu);
        resources.insert(NextState(None));
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
            AppState::MainMenu => {
                drop(app_state);
                self.schedule.execute(&mut self.world, &mut self.resources);
            }
            AppState::InGame => {
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
