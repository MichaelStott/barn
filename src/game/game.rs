use crate::audio;
use crate::graphics::sdl_renderer::SDLRenderer;
use crate::graphics::barn_gfx::BarnGFX;
use crate::game::state::State;
use crate::game::context::Context;
use crate::game::barn_context::BarnContext;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::FullscreenType;

pub struct Game {
    pub bgfx: BarnGFX,
    events: EventPump,
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
        // Handle fullscreen option.
        if fullscreen {
            window.set_fullscreen(FullscreenType::True).unwrap();
        } else {
            window.set_fullscreen(FullscreenType::Off).unwrap();
        }

        // Create graphics context and input event stream.
        let bgfx = BarnGFX { sdl: SDLRenderer::new(window.into_canvas().build().unwrap()) };
        let events = sdl.event_pump().unwrap();

        Game {
            bgfx: bgfx,
            events: events

        }
    }

    pub fn run(&mut self, mut context: BarnContext, mut state: Box<dyn State<BarnContext>>) ->  Result<(), String>{
        state.on_enter(&mut context);
        // NOTE: This probably shouldn't happen here...
        audio::init(8);
        // Initialize timestep marker... (first timestep is always zero)
        let mut prev: Option<Duration> = None;
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
            // Determine timestep.
            let dt = Game::calc_time_step(SystemTime::now().duration_since(UNIX_EPOCH).unwrap(), &mut prev);
            // State handling logic.
            let new_state = context.update(&mut *state, &mut self.events, dt);
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
        }
        audio::close();
        Ok(())
    }

    fn calc_time_step(now: Duration, prev: &mut Option<Duration>) -> f32 {
        let mut dt = 0.0;
        if *prev != None {
            dt = (now - prev.unwrap()).as_secs_f32();
        }
        *prev = Some(now);
        return dt;
    }
}