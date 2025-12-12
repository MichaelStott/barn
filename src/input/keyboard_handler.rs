use winit::event::{ElementState, KeyEvent, DeviceEvent};
use winit::keyboard::NamedKey;
use std::collections::HashMap;

pub struct KeyboardHandler {
    pub keys: HashMap<NamedKey, bool>,
    pub prev_keys: HashMap<NamedKey, bool>,
}

impl KeyboardHandler {
    pub fn new() -> Self {
        KeyboardHandler {
            keys: HashMap::new(),
            prev_keys: HashMap::new(),
        }
    }

    pub fn handle_event(&mut self, event: &KeyEvent) {
        if let Some(keycode) = match &event.logical_key {
            winit::keyboard::Key::Character(_) => None,
            winit::keyboard::Key::Named(named) => Some(*named),
            winit::keyboard::Key::Unidentified(_) => None,
            winit::keyboard::Key::Dead(_) => None,
        } {
            self.keys.insert(keycode, event.state == ElementState::Pressed);
        }
    }

    pub fn handle_device_event(&mut self, _event: &DeviceEvent) {
        // Handle device events if needed
    }

    pub fn is_key_pressed(&self, key: NamedKey) -> bool {
        *self.keys.get(&key).unwrap_or(&false)
    }

    pub fn is_key_just_pressed(&self, key: NamedKey) -> bool {
        *self.keys.get(&key).unwrap_or(&false) && !*self.prev_keys.get(&key).unwrap_or(&false)
    }

    pub fn is_key_just_released(&self, key: NamedKey) -> bool {
        !*self.keys.get(&key).unwrap_or(&false) && *self.prev_keys.get(&key).unwrap_or(&false)
    }

    pub fn update(&mut self) {
        self.prev_keys = self.keys.clone();
    }
}
