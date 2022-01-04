use crate::audio;
use crate::graphics::sdl_renderer::SDLRenderer;
use crate::graphics::barn_gfx::BarnGFX;
use crate::game::state::State;
use crate::game::context::Context;

use std::time::Duration;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::FullscreenType;

pub struct Game {
    pub bgfx: BarnGFX,
    events: EventPump
}

impl Game {

    pub fn new(window_title: &String, window_width: u32, window_height: u32, fullscreen: bool) -> Self {
        // Initialize window and graphics context.
        let sdl = sdl2::init().unwrap();
        let video_subsys = sdl.video().unwrap();
        let mut window = video_subsys
            .window(&window_title, window_width, window_height)
            .opengl()
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        if fullscreen {
            window.set_fullscreen(FullscreenType::True).unwrap();
        }

        // Create graphics context and input event stream.
        let bgfx = BarnGFX { sdl: SDLRenderer::new(window.into_canvas().build().unwrap()) };
        let events = sdl.event_pump().unwrap();

        Game {
            bgfx: bgfx,
            events: events,
        }
    }

    pub fn run(&mut self, mut context: Context, mut state: Box<dyn State>) ->  Result<(), String> {
        state.on_enter(&mut context);
        audio::init(6);
        // Main game loop.
        'running: loop {
            // Check if the game loop should be exited.
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            // State handling logic.
            let new_state = context.update(&mut *state, &mut self.events);
            context.draw(&mut *state, &mut self.bgfx);
            match new_state {
                Some(x) => {
                    state.on_exit(&mut context);
                    state = x;
                    state.on_enter(&mut context);
                    log::debug!("Switched to state: {}", state.get_name());
                }
                None => {
                    // No state change has occurred.
                }
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        audio::close();
        Ok(())
    }
}