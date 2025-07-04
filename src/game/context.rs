use crate::graphics::barn_gfx::BarnGFX;
use crate::input::keyboard_handler::KeyboardHandler;
use super::state::State;

pub trait Context { 
    type T: Context;

    fn update(&mut self, state: &mut dyn State<Self::T>, keyboard: &mut KeyboardHandler, dt: f32) -> Option<Box<dyn State<Self::T>>> where Self: std::marker::Sized;

    fn draw(&mut self, state: &mut dyn State<Self::T>, bgfx: &mut BarnGFX) where Self: std::marker::Sized;
}