use crate::input::KeyboardHandler;
use crate::graphics::wgpu_renderer::WgpuRenderer;
use super::state::State;

pub trait Context {
    /// Get a mutable reference to the keyboard handler
    fn get_input_handler(&mut self) -> std::cell::RefMut<KeyboardHandler>;
    
    /// Update the context with the given state and delta time
    fn update(&mut self, state: &mut Box<dyn State<Self>>, dt: f32) -> Option<Box<dyn State<Self>>> 
    where Self: std::marker::Sized;
    
    /// Render the state using the provided renderer
    fn render_state(&mut self, state: &mut Box<dyn State<Self>>, renderer: &mut WgpuRenderer) 
    where Self: std::marker::Sized;
}