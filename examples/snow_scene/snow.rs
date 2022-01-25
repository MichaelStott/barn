use std::char::MAX;

use barn::math::vector2::Vector2;
use barn::graphics::color::Color;

use rand::Rng; 

pub struct SnowFallLayer {
    pub snow: Vec<Snow>, 
    pub dir: Vector2,
    pub color: Color
}

pub struct Snow {
    pub pos: Vector2,
    pub speed: f32
}

const SPACING: f32 =  100.0;
const MAX_VARIANCE: i32 = 50;
const MIN_VARIANCE: i32 = 25;

impl SnowFallLayer {

    pub fn new(dir: Vector2, color: Color, speed: f32, offset: Vector2) -> SnowFallLayer {
        let mut layer = SnowFallLayer {
            dir:dir,
            snow: Vec::new(),
            color: color
        };
        for x in 0..=4 {
            for y in 0..=4 {
                let x_var = rand::thread_rng().gen_range(MIN_VARIANCE..MAX_VARIANCE) as f32 + offset.x;
                let y_var = rand::thread_rng().gen_range(MIN_VARIANCE..MAX_VARIANCE) as f32 + offset.y;
                layer.snow.push(Snow {
                    pos: Vector2::new(x as f32 * SPACING + x_var, y as f32 * SPACING + y_var),
                    speed: speed * rand::thread_rng().gen_range(1..5) as f32
                });
            }
        }
        layer
    }

    pub fn update(&mut self, dt: f32) {
        for snow in self.snow.iter_mut() {
            snow.pos += self.dir * dt * snow.speed;
            if snow.pos.y > 516.0 {
                snow.pos.y -= 512.0;
            }
            if snow.pos.x > 516.0 {
                snow.pos.x -= 512.0;
            }
            if snow.pos.x < -4.0 {
                snow.pos.x += 512.0;
            }
        }
    }
}