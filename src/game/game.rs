use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use crate::game::state::State;
use crate::game::context::Context;
use sdl2::video::GLContext;
use std::time::Duration;
use sdl2::EventPump;
use sdl2::video::Window;
use sdl2::VideoSubsystem;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::FullscreenType;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

pub struct Game {
    events: EventPump,
    _gl_context: GLContext,
    _video_subsys: VideoSubsystem,
    pub _canvas: Canvas<Window>,
    _sdl: Sdl,
}

impl Game {

    pub fn new(window_title: &String, window_width: u32, window_height: u32, fullscreen: bool) -> Self {
        // Initialize window and graphics context.
        let _sdl = sdl2::init().unwrap();
        let _video_subsys =  _sdl.video().unwrap();
        let mut window = _video_subsys
            .window(&window_title, window_width, window_height)
            .opengl()
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        
        
        if fullscreen {
            window.set_fullscreen(FullscreenType::True).unwrap();
        }
        let _gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| _video_subsys.gl_get_proc_address(s) as *const std::os::raw::c_void);
        let mut canvas = window.into_canvas().build().unwrap();

        // Initialize input events.
        let events = _sdl.event_pump().unwrap();

        // Initialize sound.
        let frequency = 44_100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1_024;
        sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
        let _mixer_context =
            sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).unwrap();
        sdl2::mixer::allocate_channels(6);

        Game {
            _sdl:  _sdl,
            _gl_context: _gl_context,
            _video_subsys: _video_subsys,
            _canvas: canvas,
            events: events,
        }
    }

    pub fn run(&mut self, mut context: Context, mut state: Box<dyn State>) ->  Result<(), String> {
        let _gl = gl::load_with(|s| self._video_subsys.gl_get_proc_address(s) as *const std::os::raw::c_void);
        state.on_enter(&mut context);
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
            context.draw(&mut *state, &mut self._canvas);
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
        Ok(())
    }
}