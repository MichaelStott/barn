use std::ops;

/// Represents a two-dimensional vector.
#[derive(Clone, Copy)]
pub struct Vector2 {
    /// f32 value representing the x-component of the vector.
    pub x: f32, 
    /// f32 value representing the y-component of the vector.
    pub y: f32
}

impl Vector2 {

    /// Length one vector in the positive x direction.
    pub const RIGHT: Vector2 = Vector2{x: 1.0, y: 0.0};
    /// Length one vector in the positive y direction.
    pub const UP: Vector2 = Vector2{x: 0.0, y: 1.0};
    /// Vector where all componenets are of value zero.
    pub const ZERO: Vector2 = Vector2{x: 0.0, y: 0.0};

    /// Return a new two dimensional vector class.
    /// 
    /// # Arguments
    ///
    /// * `x` - f32 value representing the x-component of the vector 
    /// * `y` - f32 value representing the y-component of the vector 
    /// 
    /// # Examples
    /// ```
    /// use barn::math::vector2::Vector2;
    /// let diagonal = Vector2::new(1.0, 1.0);
    /// ```
    /// ```
    /// // Common vector values are also provided.
    /// use barn::math::vector2::Vector2;
    /// let jump_vector = Vector2::UP;
    /// ```
    pub const fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x: x, 
            y: y
        }
    }

    /// Returns the length of the vector as f32 value.
    /// 
    /// # Examples
    /// ```
    /// use barn::math::vector2::Vector2;
    /// // Returns a length of "1"
    /// let length = Vector2::RIGHT.length();
    /// ```
    /// ```
    /// use barn::math::vector2::Vector2;
    /// // Returns a length equal to sqrt(2)
    /// let vector = Vector2::new(1.0, -1.0);
    /// let length = vector.length();
    /// ```
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Caclulate a new vector with the same direction but of length 1.
    /// 
    /// # Examples
    /// ```
    /// use barn::math::vector2::Vector2;
    /// let diagonal = Vector2::new(-1.0, 1.0).normalize();
    /// ```
    /// ```
    /// use barn::math::vector2::Vector2;
    /// // Returns a zero vector.
    /// let diagonal = Vector2::ZERO.normalize();
    /// ```
    pub fn normalize(&mut self) -> Vector2 {
        let mut result = Vector2::ZERO;
        if self != &Vector2::ZERO {
            let length = self.length();
            result = *self / length;
        }
        result
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, _rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, _rhs: Vector2) {
        *self = Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        };
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, _rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        }
    }
}

impl ops::SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, _rhs: Vector2) {
        *self = Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        };
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, _rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * _rhs,
            y: self.y * _rhs
        }
    }
}

impl ops::MulAssign<f32> for Vector2 {
    fn mul_assign(self: &mut Vector2, _rhs: f32) {
        *self = Self {
            x: self.x * _rhs,
            y: self.y * _rhs
        };
    }
}

impl ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, _rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x / _rhs,
            y: self.y / _rhs
        }
    }
}

impl ops::DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, _rhs: f32) {
        *self = Self {
            x: self.x / _rhs,
            y: self.y / _rhs
        };
    }
}

impl PartialEq for Vector2 {
    fn eq(self: &Vector2, _rhs: &Vector2) -> bool {
        self.x == _rhs.x && self.y == _rhs.y
    }
}