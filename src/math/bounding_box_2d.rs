use crate::math::vector2::Vector2;

/// A two dimensional bounding box used for detecting and resolving collisions.
#[derive(Clone)]
pub struct BoundingBox2D {
    /// Position of top-left corner of bounding box.
    pub origin: Vector2,
    /// The width of the bounding box.
    pub width: u32,
    /// The height of the bounding box.
    pub height: u32
}

impl BoundingBox2D {

    /// Returns a two-dimensional bounding box.
    ///
    /// # Arguments
    ///
    /// * `origin` - Position of top-left corner of bounding box.
    /// * `width` - Width of bounding box.
    /// * `height` - Height of bounding box.
    ///
    /// # Examples
    ///
    /// ```
    /// use barn::math::vector2::Vector2;
    /// use barn::math::bounding_box_2d::BoundingBox2D;
    /// let bb = BoundingBox2D::new(Vector2::ZERO, 50, 50);
    /// ```
    pub fn new(origin: Vector2, width: u32, height: u32) -> Self {
        Self {
            origin: origin,
            width: width,
            height: height
        }
    }

    pub fn intersects_point(&mut self, point: &mut Vector2) -> bool {
        point.x < self.origin.x + self.width as f32 &&
        self.origin.x < point.x && 
        point.y < self.origin.y + self.height as f32 &&
        self.origin.y < point.y 
    }

    pub fn intersects_box(&mut self, bb: &mut BoundingBox2D) -> bool {
        (self.origin.x < (bb.origin.x + bb.width as f32) as f32)
        && (self.origin.x + self.width as f32) > (bb.origin.x as f32)
        && (self.origin.y) < ((bb.origin.y + bb.height as f32) as f32)
        && (self.origin.y + self.height as f32 > bb.origin.y as f32)
    }

    pub fn center(&mut self) -> Vector2 {
        Vector2 {
            x: self.origin.x.clone() + (self.width.clone() / 2) as f32,
            y: self.origin.y.clone() + (self.height.clone() / 2) as f32,
        }
    }

    pub fn resolve_bb_intersect(&mut self, bb: &mut BoundingBox2D, vel: &mut Vector2) {
        // Handle x-axis of collision.
        if vel.x != 0.0 {
            self.origin.x += vel.x;
            if self.intersects_box(bb) {
                let dir: f32 = if vel.x > 0.0 { -1.0 } else { 1.0 };
                self.origin.x = if dir == 1.0 {
                    (bb.origin.x + bb.width as f32) as f32
                } else {
                    (bb.origin.x - self.width as f32) as f32
                };
            }
        }
        // Handle y-axis of collision
        if vel.y != 0.0 {
            self.origin.y += vel.y;
            if self.intersects_box(bb) {
                let dir: f32 = if vel.y > 0.0 { -1.0 } else { 1.0 };
                self.origin.y = if dir == 1.0 {
                    (bb.origin.y + (bb.height) as f32) as f32
                } else {
                    (bb.origin.y - (self.height) as f32) as f32
                };
            }
        }
    }
}