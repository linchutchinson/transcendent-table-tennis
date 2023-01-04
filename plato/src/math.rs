use macroquad::prelude::Vec2 as MQVec2;

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Vec2> for MQVec2 {
    fn from(item: Vec2) -> Self {
        MQVec2::new(item.x, item.y)
    }
}
