use sdl2::mixer::{AudioFormat, InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

const FREQUENCY: i32 = 44_100;
const FORMAT: AudioFormat = AUDIO_S16LSB; 
const CHANNELS: i32 = DEFAULT_CHANNELS; 
const CHUNK_SIZE: i32 = 1_024;

static mut MIXER_INIT: bool = false;

pub type SdlSound = sdl2::mixer::Chunk;

pub fn init(number_of_channels: i32) {
    sdl2::mixer::open_audio(FREQUENCY, FORMAT, CHANNELS, CHUNK_SIZE).unwrap();
    sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).unwrap();
    sdl2::mixer::allocate_channels(number_of_channels);
    unsafe { MIXER_INIT = true; }
}

pub fn close() {
    sdl2::mixer::close_audio();
    unsafe { MIXER_INIT = false; }
}

pub fn play_music(file: String) {
    sdl2::mixer::Music::from_file(file).unwrap();
}

pub fn play_sound(sound_fx: SdlSound, channel: i32) {
    let channel = sdl2::mixer::Channel(channel);
    channel.play(&sound_fx, 0).unwrap();
}
