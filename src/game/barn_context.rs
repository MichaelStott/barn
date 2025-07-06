use crate::audio::AudioManager;
use crate::input::KeyboardHandler;
use crate::game::state::State;
use crate::game::context::Context;
use crate::graphics::wgpu_renderer::WgpuRenderer;
use std::rc::Rc;
use std::cell::RefCell;

pub struct BarnContext {
    pub audio_manager: AudioManager,
    pub keyboard: Rc<RefCell<KeyboardHandler>>,
}

impl Context for BarnContext {
    fn get_input_handler(&mut self) -> std::cell::RefMut<KeyboardHandler> {
        self.keyboard.borrow_mut()
    }

    fn update(&mut self, state: &mut Box<dyn State<Self>>, dt: f32) -> Option<Box<dyn State<Self>>> {
        state.update(self, dt)
    }

    fn render_state(&mut self, state: &mut Box<dyn State<Self>>, renderer: &mut WgpuRenderer) {
        state.render(self, renderer);
    }
}

impl BarnContext {
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

    pub fn load_texture(&mut self, name: &str, path: &str, renderer: &mut WgpuRenderer) -> Result<(), Box<dyn std::error::Error>> {
        renderer.load_texture(path)
    }
}
