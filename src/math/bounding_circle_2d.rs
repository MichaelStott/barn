use crate::barn::math::vector2::Vector2;

#[derive(Clone)]
pub struct BoundingCircle2D {
    pub center: Vector2,
    pub radius: i32
}

impl BoundingCircle2D {

    pub fn intersects_point(&mut self, point: Vector2) -> bool {
        (point - self.center).length() < self.radius as f32
    }

    pub fn intersects_circle(&mut self, circle: BoundingCircle2D) -> bool {
        (circle.center - self.center).length() < (circle.radius + self.radius) as f32
    }
}
