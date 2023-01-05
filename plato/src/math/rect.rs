use super::Vec2;
use macroquad::prelude::Rect as MQRect;

pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }

    /// Translate a position on this rect to a position on another differently sized Rect.
    /// In other words, a point that would be in the center of this rect will be in the
    /// center of the other rect after projection.
    pub fn project_point(&self, other: &Self, point: Vec2) -> Vec2 {
        let local_point = point - self.position;
        let normalized_local_point = local_point / self.size;
        let other_local_point = normalized_local_point * other.size;

        other_local_point + other.position
    }
}

impl From<Rect> for MQRect {
    fn from(item: Rect) -> Self {
        Self::new(item.position.x, item.position.y, item.size.x, item.size.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_projection_on_larger() {
        let base_rect = Rect::new(Vec2::ZERO, Vec2::new(10.0, 10.0));
        let tenx_rect = Rect::new(Vec2::ZERO, Vec2::new(100.0, 100.0));

        let pos = Vec2::new(3.0, 6.0);
        let actual = base_rect.project_point(&tenx_rect, pos);
        let expected = Vec2::new(30.0, 60.0);
        assert!(
            actual.almost_equal(expected),
            "Expect equality between {actual:?} and {expected:?}"
        );
    }

    #[test]
    fn test_point_projection_on_translated() {
        let base_rect = Rect::new(Vec2::ZERO, Vec2::new(100.0, 100.0));
        let translated_rect = Rect::new(Vec2::new(10.0, 10.0), Vec2::new(100.0, 100.0));

        let pos = Vec2::new(3.0, 6.0);
        let actual = base_rect.project_point(&translated_rect, pos);
        let expected = Vec2::new(13.0, 16.0);
        assert!(
            actual.almost_equal(expected),
            "Expect equality between {actual:?} and {expected:?}"
        );
    }
}
