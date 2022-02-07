use crate::graphics::SdlTexture;
use crate::audio::SdlSound;
use crate::fonts::{SdlFont, TTF_CONTEXT};
use crate::fonts::font_details::FontDetails;
use crate::game::game::Game;
use crate::input::keyboard_handler::KeyboardHandler;

use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;

use std::path::Path;
use std::collections::HashMap;

use super::context::Context;

pub struct BarnContext {
    texture_creator: TextureCreator<WindowContext>,
    ttf_context: &'static Sdl2TtfContext,
    fonts: HashMap<FontDetails, SdlFont>,
    textures: HashMap<String, SdlTexture>,
    sounds: HashMap<String, SdlSound>,
    pub input: KeyboardHandler,
}

impl Context for BarnContext {
    type T = BarnContext;

    fn update(&mut self, state: &mut dyn super::state::State<BarnContext>, event: &mut sdl2::EventPump, dt: f32) -> Option<Box<dyn super::state::State<BarnContext>>>
    where
        Self: std::marker::Sized,
        BarnContext: Context {
        self.get_input_handler().update(event);
        state.update(self, dt)
    }

    fn draw(&mut self, state: &mut dyn super::state::State<BarnContext>, bgfx: &mut crate::graphics::barn_gfx::BarnGFX)
    where
        Self: std::marker::Sized,
        BarnContext: Context {
        state.draw(self, bgfx);
        self.get_input_handler().refresh_prev()
    }
}

impl BarnContext {

    pub fn new(
        game: &mut Game
    ) -> Self {
        let texture_creator = game.bgfx.sdl.generate_texture_creator();
        BarnContext {
            input: KeyboardHandler::new(),
            texture_creator: texture_creator,
            ttf_context: &TTF_CONTEXT,
            textures: HashMap::new(),
            sounds: HashMap::new(),
            fonts: HashMap::new(),
        }
    }

    pub fn load_sound(&mut self, path: String) -> &mut SdlSound {
        if !self.sounds.contains_key(&path) {
            let sound = sdl2::mixer::Chunk::from_file(Path::new(&path)).unwrap();
            self.sounds.insert(path.clone(), sound);
        }
        self.sounds.get_mut(&path).unwrap()
    }

    pub fn load_texture(&mut self, path: String) -> &mut SdlTexture {
        if !self.textures.contains_key(&path) {
            let texture = self.texture_creator.load_texture(Path::new(&path)).unwrap();
            self.textures.insert(path.clone(), texture);
        }
        self.textures.get_mut(&path).unwrap()
    }

    pub fn load_font(&mut self, details: FontDetails) -> &SdlFont {
        if !self.fonts.contains_key(&details) {
            self.fonts.insert(details.clone(), 
                self.ttf_context.load_font(details.clone().path, details.clone().size).unwrap());
        }
        self.fonts.get_mut(&details).unwrap()
    }

    pub fn get_input_handler(&mut self) -> &mut KeyboardHandler {
        &mut self.input
    }
}
