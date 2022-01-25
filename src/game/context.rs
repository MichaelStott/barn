use crate::graphics::SdlTexture;
use crate::audio::SdlSound;
use crate::fonts::{SdlFont, TTF_CONTEXT};
use crate::graphics::barn_gfx::BarnGFX;
use crate::fonts::font_details::FontDetails;
use crate::game::game::Game;
use crate::game::state::State;
use crate::input::keyboard_handler::KeyboardHandler;

use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use sdl2::EventPump;

use std::path::Path;
use std::collections::HashMap;

pub struct Context {
    texture_creator: TextureCreator<WindowContext>,
    ttf_context: &'static Sdl2TtfContext,
    fonts: HashMap<FontDetails, SdlFont>,
    textures: HashMap<String, SdlTexture>,
    sounds: HashMap<String, SdlSound>,
    pub input: KeyboardHandler,
}

impl Context {
    /**
     * Due to ownership issues of concurrent hashmap access, this needs to be rewritten 
     * as an interface with default implementations for update and draw. State objects 
     * can then downcast to get specific resources.
     */

    pub fn new(
        game: &mut Game
    ) -> Self {
        let texture_creator = game.bgfx.sdl.generate_texture_creator();
        Context {
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

    pub fn update(&mut self, state: &mut dyn State, event: &mut EventPump, dt: f32) -> Option<Box<dyn State>>
    where
        Self: std::marker::Sized,
    {
        self.get_input_handler().update(event);
        state.update(self, dt)
    }

    pub fn draw(&mut self, state: &mut dyn State, bgfx: &mut BarnGFX)
    where
        Self: std::marker::Sized,
    {
        state.draw(self, bgfx);
        self.get_input_handler().refresh_prev();
    }
}
