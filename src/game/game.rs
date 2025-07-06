use crate::audio::AudioManager;
use crate::graphics::wgpu_renderer::WgpuRenderer;
use crate::input::KeyboardHandler;
use crate::game::state::State;
use crate::game::context::Context;
use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::sync::Arc;

pub struct Game<C: Context> {
    pub renderer: Option<WgpuRenderer>,
    pub keyboard: Rc<RefCell<KeyboardHandler>>,
    pub audio_manager: AudioManager,
    pub context: Option<C>,
    pub current_state: Option<Box<dyn State<C>>>,
    pub running: bool,
    pub last_frame_time: Instant,
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
}

impl<C: Context> Game<C> {
    pub fn new(window_title: &str, window_width: u32, window_height: u32) -> Result<Self, Box<dyn std::error::Error>> {
        env_logger::init();
        
        let keyboard = Rc::new(RefCell::new(KeyboardHandler::new()));
        let audio_manager = AudioManager::new()?;
        
        Ok(Game {
            renderer: None,
            keyboard,
            audio_manager,
            context: None, // Will be initialized in run() when we have a mutable reference to self
            current_state: None,
            running: true,
            last_frame_time: Instant::now(),
            window_title: window_title.to_string(),
            window_width,
            window_height,
        })
    }
    
    pub fn get_keyboard(&self) -> &Rc<RefCell<KeyboardHandler>> {
        &self.keyboard
    }
    
    pub fn run(
        mut self,
        mut initial_state: Box<dyn State<C>>,
        mut context: C,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        let window = Arc::new(WindowBuilder::new()
            .with_title(&self.window_title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.window_width as f64, self.window_height as f64))
            .build(&event_loop)?);
        let window_for_renderer = Arc::clone(&window);
        let (renderer, mut surface, mut config) = WgpuRenderer::new(&window_for_renderer)?;
        self.renderer = Some(renderer);
        
        // Initialize context
        self.context = Some(context);
        
        // Call on_enter for the initial state
        if let Some(context) = &mut self.context {
            initial_state.on_enter(context);
        }
        self.current_state = Some(initial_state);
        
        event_loop.run(move |event, elwt| {
            let window = Arc::clone(&window);
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            self.running = false;
                            elwt.exit();
                        }
                        WindowEvent::KeyboardInput { event, .. } => {
                            if let Some(winit::keyboard::NamedKey::Escape) = match &event.logical_key {
                                winit::keyboard::Key::Character(_) => None,
                                winit::keyboard::Key::Named(named) => Some(*named),
                                winit::keyboard::Key::Unidentified(_) => None,
                                winit::keyboard::Key::Dead(_) => None,
                            } {
                                self.running = false;
                                elwt.exit();
                            }
                            self.keyboard.borrow_mut().handle_event(&event);
                        }
                        WindowEvent::Resized(new_size) => {
                            if let Some(renderer) = &mut self.renderer {
                                renderer.resize(new_size, &mut surface, &mut config);
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            let now = Instant::now();
                            let dt = now.duration_since(self.last_frame_time).as_secs_f32();
                            self.last_frame_time = now;
                            // Handle state update and rendering
                            if let Some(mut state) = self.current_state.take() {
                                if let Some(context) = &mut self.context {
                                    // Update state
                                    if let Some(new_state) = context.update(&mut state, dt) {
                                        // Call on_exit for current state
                                        state.on_exit(context);
                                        // Set new state and call on_enter
                                        let mut next_state = new_state;
                                        next_state.on_enter(context);
                                        state = next_state;
                                    }
                                    // Render state
                                    if let Some(renderer) = &mut self.renderer {
                                        context.render_state(&mut state, renderer);
                                    }
                                }
                                // Put state back
                                self.current_state = Some(state);
                            }
                            // Present the renderer
                            if let Some(renderer) = &mut self.renderer {
                                renderer.present(&mut surface);
                            }
                        }
                        _ => {}
                    }
                }
                Event::DeviceEvent { event, .. } => {
                    self.keyboard.borrow_mut().handle_device_event(&event);
                }
                Event::AboutToWait => {
                    window.request_redraw();
                    self.keyboard.borrow_mut().update();
                }
                _ => {}
            }
        })?;
        
        Ok(())
    }
} 