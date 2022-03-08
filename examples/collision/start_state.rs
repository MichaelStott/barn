
use crate::player::PLAYER_SPEED;
use crate::tile::{Tile, TileType};
use crate::player::Player;
use barn::input::SdlKeycode;
use barn::math::bounding_box_2d::BoundingBox2D;
use barn::graphics::sdl_sprite::{SdlSprite, SdlSpriteFrame, SdlSpriteAnimation};
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::color::Color;
use barn::graphics::{SdlRect};
use barn::game::state::State;
use barn::game::barn_context::BarnContext;
use barn::math::vector2::Vector2;

pub struct StartState {
    player: Player,
    tile1: Tile,
    tile2: Tile
}

impl State<BarnContext> for StartState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> { 
        // Determine player direction from user input.
        let mut vel = Vector2::ZERO;
        let delta = dt * PLAYER_SPEED;
        if context.get_input_handler().key_pressed(&SdlKeycode::Down) {
            vel.y += delta;
        }
        if context.get_input_handler().key_pressed(&SdlKeycode::Up) {
            vel.y -= delta;
        }
        if context.get_input_handler().key_pressed(&SdlKeycode::Right) {
            vel.x += delta;
        }
        if context.get_input_handler().key_pressed(&SdlKeycode::Left) {
            vel.x -= delta;
        }
        // Resolve any collisions with bounding boxes.
        self.player.bb.resolve_bb_intersect(&mut self.tile1.bb, &mut vel);
        None 
    }

    fn draw(&mut self, context: &mut BarnContext, bgfx: &mut BarnGFX) {
        bgfx.sdl.set_draw_color(Color::BLACK);
        bgfx.sdl.clear();

        // Draw player to the screen
        bgfx.sdl.set_draw_color(Color::BLUE);
        bgfx.sdl.draw_bounding_box(&mut self.player.bb, false);

        // Draw tile to the screen
        bgfx.sdl.set_draw_color(Color::GREEN);
        bgfx.sdl.draw_bounding_box(&mut self.tile1.bb, false);

        bgfx.sdl.present();
    }

    fn on_enter(&mut self, context: &mut BarnContext) {
        context.load_texture(String::from("examples/resources/images/debug_boy.png"));
    }

    fn on_exit(&mut self, context: &mut BarnContext) {}

    fn get_name(&mut self) -> String { String::from("StartState") }
}

impl StartState {
    pub fn new() -> StartState {
        StartState {
            player: Player { bb: BoundingBox2D::new(Vector2::ZERO, 50, 50) },
            tile1: Tile::new(TileType::Ground, 200.0, 200.0, 100, 100),
            tile2: Tile::new(TileType::Ground, 200.0, 200.0, 100, 100)
        }
    }
}
