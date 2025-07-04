use barn::game::state::State;
use barn::game::barn_context::BarnContext;
use barn::graphics::wgpu_renderer::WgpuRenderer;

pub struct StartState {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub font_size: f32,
    pub color: [f32; 4],
}

impl State<BarnContext> for StartState {
    fn update(&mut self, _context: &mut BarnContext, _dt: f32) -> Option<Box<dyn State<BarnContext>>> { 
        None 
    }
    
    fn render(&mut self, _context: &mut BarnContext, renderer: &mut WgpuRenderer) {
        // Set background to cornflower blue
        renderer.set_clear_color(0.392, 0.584, 0.929, 1.0);
        
        // Draw "Hello World!" text
        renderer.draw_text(&self.text, self.x, self.y, self.font_size, self.color);
    }
    
    fn on_enter(&mut self, _context: &mut BarnContext) {
        // Initialize any resources needed for this state
    }
    
    fn on_exit(&mut self, _context: &mut BarnContext) {
        // Clean up any resources
    }
    
    fn get_name(&mut self) -> String {
        String::from("StartState")
    }
}

impl StartState {
    pub fn new() -> StartState {
        StartState {
            text: String::from("Hello World!"),
            x: 100.0,
            y: 200.0,
            font_size: 48.0,
            color: [1.0, 0.0, 0.0, 1.0], // Bright red
        }
    }
}