use sdl2::mixer::{AudioFormat, InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS, Music};

const FREQUENCY: i32 = 44_100;
const FORMAT: AudioFormat = AUDIO_S16LSB; 
const CHANNELS: i32 = DEFAULT_CHANNELS; 
const CHUNK_SIZE: i32 = 1_024;

static mut MIXER_INIT: bool = false;

pub type SdlSound = sdl2::mixer::Chunk;
pub type SdlMusic = Music<'static>;

pub fn init(number_of_channels: i32) {
    sdl2::mixer::open_audio(FREQUENCY, FORMAT, CHANNELS, CHUNK_SIZE).unwrap();
    sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).unwrap();
    sdl2::mixer::allocate_channels(number_of_channels);
    unsafe { MIXER_INIT = true; }
}

pub fn close() {
    unsafe {
        if MIXER_INIT {
            sdl2::mixer::close_audio();    
            MIXER_INIT = false; 
        }
    }
}

pub fn load_music(file: &String) ->  SdlMusic {
    Music::from_file(file).unwrap()
}

pub fn play_sound(sound_fx: SdlSound, channel: i32) {
    let channel = sdl2::mixer::Channel(channel);
    channel.play(&sound_fx, 0).unwrap();
}
