use crate::audio::AudioManager;
use crate::input::KeyboardHandler;
use crate::game::state::State;
use crate::game::game::Game;
use std::rc::Rc;
use std::cell::RefCell;

pub struct BarnContext {
    pub audio_manager: AudioManager,
    pub keyboard: Rc<RefCell<KeyboardHandler>>,
}

impl BarnContext {
    pub fn update(&mut self, state: &mut Box<dyn State<BarnContext>>, dt: f32) -> Option<Box<dyn State<BarnContext>>> {
        state.update(self, dt)
    }

    pub fn draw(&mut self, state: &mut Box<dyn State<BarnContext>>, renderer: &mut crate::graphics::wgpu_renderer::WgpuRenderer, surface: &mut wgpu::Surface) {
        // For now, just render the background
        renderer.render(surface);
    }

    pub fn new(keyboard: Rc<RefCell<KeyboardHandler>>) -> Self {
        BarnContext {
            audio_manager: AudioManager::new().unwrap(),
            keyboard,
        }
    }

    pub fn load_sound(&mut self, name: &str, path: &str, repeat: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.audio_manager.load_audio(name, path, repeat)
    }

    pub fn play_sound(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.audio_manager.play_sound(name)
    }

    pub fn play_music(&mut self, name: &str, repeat: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.audio_manager.play_music(name, repeat)
    }

    pub fn stop_audio(&mut self, name: &str) {
        self.audio_manager.stop_audio(name);
    }

    pub fn pause_audio(&mut self, name: &str) {
        self.audio_manager.pause_audio(name);
    }

    pub fn resume_audio(&mut self, name: &str) {
        self.audio_manager.resume_audio(name);
    }

    pub fn set_volume(&mut self, name: &str, volume: f32) {
        self.audio_manager.set_volume(name, volume);
    }

    pub fn load_texture(&mut self, name: &str, path: &str, renderer: &mut crate::graphics::wgpu_renderer::WgpuRenderer) -> Result<(), Box<dyn std::error::Error>> {
        renderer.load_texture(path)
    }

    pub fn get_input_handler(&mut self) -> std::cell::RefMut<KeyboardHandler> {
        self.keyboard.borrow_mut()
    }

    pub fn render_state(&mut self, state: &mut Box<dyn State<BarnContext>>, renderer: &mut crate::graphics::wgpu_renderer::WgpuRenderer, surface: &mut wgpu::Surface) {
        state.render(self, renderer);
        renderer.render(surface);
    }
}
