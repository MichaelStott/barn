use crate::audio::AudioManager;
use crate::graphics::wgpu_renderer::WgpuRenderer;
use crate::input::KeyboardHandler;
use crate::game::state::State;
use crate::game::barn_context::BarnContext;
use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::sync::Arc;

pub struct Game {
    pub renderer: Option<WgpuRenderer>,
    pub keyboard: Rc<RefCell<KeyboardHandler>>,
    pub audio_manager: AudioManager,
    pub context: Option<BarnContext>,
    pub current_state: Option<Box<dyn State<BarnContext>>>,
    pub running: bool,
    pub last_frame_time: Instant,
}

impl Game {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
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
        })
    }
    
    pub fn run(
        mut self,
        mut initial_state: Box<dyn State<BarnContext>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        let window = Arc::new(WindowBuilder::new()
            .with_title("Barn Game")
            .with_inner_size(winit::dpi::LogicalSize::new(512.0, 512.0))
            .build(&event_loop)?);
        let window_for_renderer = Arc::clone(&window);
        let (renderer, mut surface, mut config) = WgpuRenderer::new(&window_for_renderer)?;
        self.renderer = Some(renderer);
        
        // Initialize context with shared keyboard
        self.context = Some(BarnContext::new(Rc::clone(&self.keyboard)));
        
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
                                    if let Some(new_state) = state.update(context, dt) {
                                        // Call on_exit for current state
                                        state.on_exit(context);
                                        // Set new state and call on_enter
                                        let mut next_state = new_state;
                                        next_state.on_enter(context);
                                        state = next_state;
                                    }
                                    // Render state
                                    if let Some(renderer) = &mut self.renderer {
                                        state.render(context, renderer);
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