use barn::game::state::State;
use barn::game::barn_context::BarnContext;
use barn::graphics::wgpu_renderer::WgpuRenderer;
use barn::fonts::font_details::FontDetails;

pub struct StartState {
    pub font_details: FontDetails,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub font_size: f32,
    pub color: [f32; 4],
}

impl State<BarnContext> for StartState {
    fn update(&mut self, _context: &mut BarnContext, _dt: f32) -> Option<Box<dyn State<BarnContext>>> { None }

    fn render(&mut self, _context: &mut BarnContext, renderer: &mut WgpuRenderer) {
        // Set background color
        renderer.set_clear_color(0.2, 0.2, 0.5, 1.0);
        // Draw text
        renderer.draw_text(&self.text, self.x, self.y, self.font_size, self.color);
    }

    fn on_enter(&mut self, context: &mut BarnContext) {
        let result = context.load_sound("music", "examples/resources/audio/laidback.mp3", true);
        let result = context.play_music("music", true);
    }

    fn on_exit(&mut self, _context: &mut BarnContext) {}

    fn get_name(&mut self) -> String { String::from("StartState") }
}

impl StartState {
    pub fn new() -> Self {
        Self {
            font_details: FontDetails {
                size: 32,
                path: "examples/resources/fonts/press-start/PressStart2P-vaV7.ttf"
            },
            text: String::from("Hello World!"),
            x: 100.0,
            y: 200.0,
            font_size: 48.0,
            color: [1.0, 1.0, 0.0, 1.0], // Yellow
        }
    }
}