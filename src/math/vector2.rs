use std::ops;

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32, 
    pub y: f32
}

impl Vector2 {

    pub const UP: Vector2 = Vector2{x: 0.0, y: 1.0};
    pub const RIGHT: Vector2 = Vector2{x: 1.0, y: 0.0};
    pub const ZERO: Vector2 = Vector2{x: 0.0, y: 0.0};

    pub const fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x: x, 
            y: y
        }
    }

    pub fn length(&mut self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) -> Vector2 {
        let length = self.length();
        Vector2 {
            x: self.x / length,
            y: self.y / length
        }
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
    fn mul_assign(&mut self, _rhs: f32) {
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
    fn eq(&self, _rhs: &Self) -> bool {
        self.x == _rhs.x && self.y == _rhs.y
    }
}