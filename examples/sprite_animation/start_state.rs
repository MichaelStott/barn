use barn::graphics::wgpu_renderer::WgpuRenderer;
use barn::graphics::wgpu_sprite::WgpuSprite;
use barn::game::state::State;
use barn::game::barn_context::BarnContext;

pub struct StartState {
    pub animation_time: f32,
    pub current_frame: usize,
    pub frame_duration: f32,
    pub frames: Vec<WgpuSprite>,
    pub frames_initialized: bool,
}

impl<'a> State<BarnContext> for StartState {
    fn update(&mut self, _context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> {
        self.animation_time += dt;
        if self.animation_time >= self.frame_duration && !self.frames.is_empty() {
            self.animation_time = 0.0;
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
        None
    }

    fn render(&mut self, _context: &mut BarnContext, renderer: &mut WgpuRenderer) {
        if !self.frames_initialized {
            // Create frames here, since we have access to renderer
            let texture_name = "examples/resources/images/debug_boy.png";
            // Ensure the texture is loaded
            if renderer.textures.get(texture_name).is_none() {
                let _ = renderer.load_texture(texture_name);
            }
            let frame_width = 9.0;
            let frame_height = 15.0;
            let num_frames = 4;
            let dst_width = 36.0 * 2.0; // scale up
            let dst_height = 60.0 * 2.0;
            let x = 200.0;
            let y = 200.0;
            let mut frames = Vec::new();
            if let Some(tex) = renderer.textures.get(texture_name) {
                let tex_size = (tex.size.width, tex.size.height);
                for i in 0..num_frames {
                    let src = (i as f32 * frame_width, 0.0, frame_width, frame_height);
                    let dst = (x, y, dst_width, dst_height);
                    let sprite = WgpuSprite::new(src, dst, [1.0, 1.0, 1.0, 1.0], &renderer.device, texture_name.to_string(), tex_size, renderer.size.width as f32, renderer.size.height as f32);
                    frames.push(sprite);
                }
            }
            self.frames = frames;
            self.frames_initialized = true;
        }
        if !self.frames.is_empty() {
            renderer.draw_sprite(self.frames[self.current_frame].clone());
        }
    }

    fn on_enter(&mut self, _context: &mut BarnContext) {}
    fn on_exit(&mut self, _context: &mut BarnContext) {}
    fn get_name(&mut self) -> String { String::from("StartState") }
}

impl StartState {
    pub fn new() -> StartState {
        StartState {
            animation_time: 0.0,
            current_frame: 0,
            frame_duration: 0.2, // 200ms per frame
            frames: Vec::new(),
            frames_initialized: false,
        }
    }
}