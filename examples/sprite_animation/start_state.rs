
use barn::graphics::sdl_sprite::{SdlSprite, SdlSpriteFrame, SdlSpriteAnimation};
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::color::Color;
use barn::graphics::{SdlRect};
use barn::game::state::State;
use barn::game::barn_context::BarnContext;

pub struct StartState {
    pub sprite: SdlSprite
}

impl State<BarnContext> for StartState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> { 
        self.sprite.get_active_animation().unwrap().tick(dt);
        None 
    }

    fn draw(&mut self, context: &mut BarnContext, bgfx: &mut BarnGFX) {
        bgfx.sdl.set_draw_color(Color::SKY);
        bgfx.sdl.clear();

        bgfx.sdl.draw_sprite(context.load_texture(String::from("examples/resources/images/debug_boy.png")), &mut self.sprite);
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
        let mut sprite = SdlSprite::new(
            SdlRect::new(0, 0, 9, 15),
            SdlRect::new(0, 0, 9, 15)
        );
        let mut walk_down_frames: Vec<SdlSpriteFrame> = Vec::new();
        let mut walk_up_frames: Vec<SdlSpriteFrame> = Vec::new();
        walk_down_frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(18, 0, 9, 15), duration: 0.2});
        walk_down_frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(9, 0, 9, 15), duration: 0.2});
        walk_down_frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(27, 0, 9, 15), duration: 0.2});
        walk_down_frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(9, 0, 9, 15), duration: 0.2});
        let walk_down_anim: SdlSpriteAnimation = SdlSpriteAnimation::new(walk_down_frames, true);
        sprite.add_animation(walk_down_anim, String::from("walk_down"));
        walk_up_frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(18, 0, 9, 15), duration: 0.2});
        walk_up_frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(9, 0, 9, 15), duration: 0.2});
        walk_up_frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(27, 0, 9, 15), duration: 0.2});
        walk_up_frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(9, 0, 9, 15), duration: 0.2});
        let walk_up_anim: SdlSpriteAnimation = SdlSpriteAnimation::new(walk_up_frames, true);
        sprite.add_animation(walk_up_anim, String::from("walk_up"));
        sprite.play_animation(String::from("walk_up"), true);
        StartState {
            sprite: sprite
        }
    }
}