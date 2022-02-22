use std::any::Any;

use sdl2::EventPump;

use crate::graphics::barn_gfx::BarnGFX;
use super::state::State;

pub trait Context { 
    type T: Context;

    fn update(&mut self, state: &mut dyn State<Self::T>, event: &mut EventPump, dt: f32) -> Option<Box<dyn State<Self::T>>> where Self: std::marker::Sized;

    fn draw(&mut self, state: &mut dyn State<Self::T>, bgfx: &mut BarnGFX) where Self: std::marker::Sized;
}