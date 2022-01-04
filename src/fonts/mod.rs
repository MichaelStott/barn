
use lazy_static::lazy_static;
use leak::Leak;

pub mod font_details;

pub type SdlFont = sdl2::ttf::Font<'static, 'static>;
lazy_static! {
    pub static ref TTF_CONTEXT: &'static sdl2::ttf::Sdl2TtfContext = Box::from(sdl2::ttf::init().unwrap()).leak();
}