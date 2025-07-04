pub trait State<T> {
    // Update game logic.
    fn update(&mut self, context: &mut T, dt: f32) -> Option<Box<dyn State<T>>>;

    // Render the game entities.
    fn render(&mut self, context: &mut T, renderer: &mut crate::graphics::wgpu_renderer::WgpuRenderer);

    // Perform any initialization here.
    fn on_enter(&mut self, context: &mut T);

    // Perform any cleanup before transitioning to the next state.
    fn on_exit(&mut self, context: &mut T);

    // Get the state name.
    fn get_name(&mut self) -> String;
}
