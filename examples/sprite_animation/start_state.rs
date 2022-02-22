
use barn::graphics::sdl_sprite::{SdlSprite, SdlSpriteFrame, SdlSpriteAnimation};
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::color::Color;
use barn::graphics::{SdlRect, SdlTexture};
use barn::game::state::State;
use barn::game::barn_context::BarnContext;

pub struct StartState {
    pub sprite: SdlSprite
}

impl State<BarnContext> for StartState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> { 
        self.sprite.animations.get_mut(&String::from("walk_down")).unwrap().tick(dt);
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
        let mut frames: Vec<SdlSpriteFrame> = Vec::new();
        frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(18, 0, 9, 15), duration: 0.2});
        frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(9, 0, 9, 15), duration: 0.2});
        frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(27, 0, 9, 15), duration: 0.2});
        frames.push(SdlSpriteFrame{dst: SdlRect::new(0, 0, 36, 60), src: SdlRect::new(9, 0, 9, 15), duration: 0.2});
        let mut anim: SdlSpriteAnimation = SdlSpriteAnimation::new(frames, true);
        sprite.animations.insert(String::from("walk_down"), anim);
        sprite.play_animation(String::from("walk_down"), true);
        StartState {
            sprite: sprite
        }
    }
}