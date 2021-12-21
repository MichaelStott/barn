use crate::fonts::font_details::FontDetails;
use crate::game::game::Game;
use crate::game::state::State;
use crate::input::keyboard_handler::KeyboardHandler;

use sdl2::mixer::Chunk;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use sdl2::EventPump;
use std::path::Path;
use std::collections::HashMap;

use lazy_static::lazy_static;
use leak::Leak;

pub type Font = sdl2::ttf::Font<'static, 'static>;
lazy_static! {
    static ref TTF_CONTEXT: &'static sdl2::ttf::Sdl2TtfContext = Box::from(sdl2::ttf::init().unwrap()).leak();
}

pub struct Context {
    texture_creator: TextureCreator<WindowContext>,
    ttf_context: &'static Sdl2TtfContext,
    fonts: HashMap<FontDetails, Font>,
    textures: HashMap<String, Texture>,
    sounds: HashMap<String, Chunk>,
    pub input: KeyboardHandler,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl Context {
    pub fn new(
        game: &Game
    ) -> Self {
        let texture_creator = game._canvas.texture_creator();
        Context {
            input: KeyboardHandler::new(),
            texture_creator: texture_creator,
            ttf_context: &TTF_CONTEXT,
            textures: HashMap::new(),
            sounds: HashMap::new(),
            fonts: HashMap::new(),
            screen_width: game._canvas.window().size().0,
            screen_height: game._canvas.window().size().1
        }
    }

    pub fn load_sound(&mut self, path: String) -> &mut Chunk {
        if !self.sounds.contains_key(&path) {
            let sound = sdl2::mixer::Chunk::from_file(Path::new(&path)).unwrap();
            self.sounds.insert(path.clone(), sound);
        }
        self.sounds.get_mut(&path).unwrap()
    }

    pub fn load_texture(&mut self, path: String) -> &mut Texture {
        if !self.textures.contains_key(&path) {
            let texture = self.texture_creator.load_texture(Path::new(&path)).unwrap();
            self.textures.insert(path.clone(), texture);
        }
        self.textures.get_mut(&path).unwrap()
    }

    pub fn load_font(&mut self, details: FontDetails) -> &Font {
        if !self.fonts.contains_key(&details) {

            self.fonts.insert(details.clone(), 
                self.ttf_context.load_font(details.clone().path, details.clone().size).unwrap());
        }
        self.fonts.get_mut(&details).unwrap()
    }


    pub fn get_input_handler(&mut self) -> &mut KeyboardHandler {
        &mut self.input
    }

    pub fn update(&mut self, state: &mut dyn State, event: &mut EventPump) -> Option<Box<dyn State>>
    where
        Self: std::marker::Sized,
    {
        self.get_input_handler().update(event);
        state.update(self)
    }

    pub fn draw(&mut self, state: &mut dyn State, canvas: &mut WindowCanvas)
    where
        Self: std::marker::Sized,
    {
        state.draw(self, canvas);
        self.get_input_handler().refresh_prev();
    }
}
