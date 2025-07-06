use crate::player::PLAYER_SPEED;
use crate::tile::{Tile, TileType};
use crate::player::Player;

use barn::graphics::fill_type::FillType;
use barn::input::Keycode;
use barn::math::bounding_box_2d::BoundingBox2D;
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::color::Color;
use barn::game::state::State;
use barn::game::barn_context::BarnContext;
use barn::game::context::Context;
use barn::math::vector2::Vector2;
use winit::keyboard::NamedKey;

pub struct StartState {
    player: Player,
    tile1: Tile,
    tile2: Tile
}

impl State<BarnContext> for StartState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> {
        println!("[DEBUG] dt: {}", dt);
        let mut vel = Vector2::ZERO;
        let delta = dt * PLAYER_SPEED;
        let mut input_handler = Context::get_input_handler(context);
        let down = input_handler.is_key_pressed(NamedKey::ArrowDown);
        let up = input_handler.is_key_pressed(NamedKey::ArrowUp);
        let right = input_handler.is_key_pressed(NamedKey::ArrowRight);
        let left = input_handler.is_key_pressed(NamedKey::ArrowLeft);
        println!("[DEBUG] Keys: Down={}, Up={}, Right={}, Left={}", down, up, right, left);
        if down { vel.y += delta; }
        if up { vel.y -= delta; }
        if right { vel.x += delta; }
        if left { vel.x -= delta; }
        println!("[DEBUG] Velocity before collision: x={}, y={}", vel.x, vel.y);
        println!("[DEBUG] Player pos before: x={}, y={}", self.player.bb.origin.x, self.player.bb.origin.y);
        let tiles = &mut [self.tile1.bb.clone(), self.tile2.bb.clone()].to_vec();
        self.player.bb.resolve_bb_intersect(tiles, &mut vel);
        println!("[DEBUG] Player pos after: x={}, y={}", self.player.bb.origin.x, self.player.bb.origin.y);
        None
    }

    fn render(&mut self, context: &mut BarnContext, renderer: &mut barn::graphics::wgpu_renderer::WgpuRenderer) {
        let bb = &self.player.bb;
        renderer.draw_rect(bb.origin.x as i32, bb.origin.y as i32, bb.width, bb.height, [0.0, 0.0, 1.0, 1.0]);
        let t1 = &self.tile1.bb;
        renderer.draw_rect(t1.origin.x as i32, t1.origin.y as i32, t1.width, t1.height, [0.0, 1.0, 0.0, 1.0]);
        let t2 = &self.tile2.bb;
        renderer.draw_rect(t2.origin.x as i32, t2.origin.y as i32, t2.width, t2.height, [0.0, 1.0, 0.0, 1.0]);
    }

    fn on_enter(&mut self, context: &mut BarnContext) {
        // If you want to load a texture, you must now provide name, path, and renderer.
        // context.load_texture("debug_boy", "examples/resources/images/debug_boy.png", renderer);
    }

    fn on_exit(&mut self, _context: &mut BarnContext) {}

    fn get_name(&mut self) -> String { String::from("StartState") }
}

impl StartState {
    pub fn new() -> StartState {
        StartState {
            player: Player { bb: BoundingBox2D::new(Vector2::ZERO, 50, 50) },
            tile1: Tile::new(TileType::Ground, 200.0, 200.0, 100, 100),
            tile2: Tile::new(TileType::Ground, 300.0, 300.0, 100, 100)
        }
    }
}
