use std::{collections::HashSet, hash::Hash};

use macroquad::prelude::Vec2;

pub struct MousePosition(pub Vec2);

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub struct Input<T> {
    pressed: HashSet<T>,
    just_pressed: HashSet<T>,
}

impl<T> Default for Input<T> {
    fn default() -> Self {
        Self {
            pressed: Default::default(),
            just_pressed: Default::default(),
        }
    }
}

impl<T> Input<T>
where
    T: Eq + Hash + Copy + 'static,
{
    pub fn press(&mut self, input: T) {
        let newly_pressed = self.pressed.insert(input);
        if newly_pressed {
            self.just_pressed.insert(input);
        }
    }

    pub fn release(&mut self, input: T) {
        self.pressed.remove(&input);
    }

    /// Clear the just_pressed array.
    pub fn tick_frame(&mut self) {
        self.just_pressed.clear();
    }

    pub fn is_pressed(&self, input: T) -> bool {
        self.pressed.contains(&input)
    }

    pub fn is_just_pressed(&self, input: T) -> bool {
        self.just_pressed.contains(&input)
    }
}
