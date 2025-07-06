use barn::math::vector2::Vector2;
use barn::graphics::color::Color;

use rand::Rng; 

pub struct SnowFallLayer {
    pub snow: Vec<Snow>, 
    pub dir: Vector2,
    pub color: Color,
    pub window_width: f32,
    pub window_height: f32
}

pub struct Snow {
    pub pos: Vector2,
    pub speed: f32
}

const SPACING: f32 =  200.0;
const MAX_VARIANCE: i32 = 50;
const MIN_VARIANCE: i32 = 25;

impl SnowFallLayer {

    pub fn new(dir: Vector2, color: Color, speed: f32, offset: Vector2, window_width: f32, window_height: f32) -> SnowFallLayer {
        let mut layer = SnowFallLayer {
            dir:dir,
            snow: Vec::new(),
            color: color,
            window_width,
            window_height
        };
        
        // Calculate grid size based on window dimensions
        let grid_cols = (window_width / SPACING).ceil() as i32;
        let grid_rows = (window_height / SPACING).ceil() as i32;
        
        for x in 0..=grid_cols {
            for y in 0..=grid_rows {
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
            if snow.pos.y > self.window_height + 4.0 {
                snow.pos.y -= self.window_height;
            }
            if snow.pos.x > self.window_width + 4.0 {
                snow.pos.x -= self.window_width;
            }
            if snow.pos.x < -4.0 {
                snow.pos.x += self.window_width;
            }
        }
    }
}