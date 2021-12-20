use crate::barn::math::vector2::Vector2;

#[derive(Clone)]
pub struct BoundingBox2D {
    pub origin: Vector2,
    pub width: i32,
    pub height: i32
}

impl BoundingBox2D {

    pub fn intersects_point(&mut self, point: Vector2) -> bool {
        point.x < self.origin.x + self.width as f32 &&
        self.origin.x < point.x && 
        point.y < self.origin.y + self.height as f32 &&
        self.origin.y < point.y 
    }

    pub fn intersects_box(&mut self, bb: BoundingBox2D) -> bool {
        (self.origin.x < (bb.origin.x + bb.width as f32) as f32)
        && (self.origin.x + self.width as f32) > (bb.origin.x as f32)
        && (self.origin.y + self.height as f32) < ((bb.origin.y + bb.height as f32) as f32)
        && (self.origin.y + self.height as f32 > bb.origin.y as f32)
    }

    pub fn center(&mut self) -> Vector2 {
        Vector2 {
            x: self.origin.x + (self.width / 2) as f32,
            y: self.origin.y + (self.height / 2) as f32,
        }
    }
}