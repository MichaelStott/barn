use rodio::{Decoder, OutputStream, Sink, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub struct AudioManager {
    sinks: HashMap<String, Sink>,
    output_stream: OutputStream,
    output_handle: rodio::OutputStreamHandle,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (output_stream, output_handle) = OutputStream::try_default()?;
        
        Ok(AudioManager {
            sinks: HashMap::new(),
            output_stream,
            output_handle,
        })
    }
    
    pub fn load_audio(&mut self, name: &str, path: &str, repeat: bool) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)?;
        let source: Box<dyn Source<Item = _> + Send> = if repeat {
            Box::new(source.repeat_infinite())
        } else {
            Box::new(source)
        };
        let sink = Sink::try_new(&self.output_handle)?;
        sink.append(source);
        sink.pause();
        self.sinks.insert(name.to_string(), sink);
        Ok(())
    }
    
    pub fn play_sound(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(sink) = self.sinks.get_mut(name) {
            sink.play();
        }
        Ok(())
    }
    
    pub fn play_music(&mut self, name: &str, repeat: bool) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(sink) = self.sinks.get_mut(name) {
            sink.play();
            if repeat {
                // Note: For true looping, you'd need to reload the audio
                // This is a simplified implementation
            }
        }
        Ok(())
    }
    
    pub fn stop_audio(&mut self, name: &str) {
        if let Some(sink) = self.sinks.get_mut(name) {
            sink.stop();
        }
    }
    
    pub fn pause_audio(&mut self, name: &str) {
        if let Some(sink) = self.sinks.get_mut(name) {
            sink.pause();
        }
    }
    
    pub fn resume_audio(&mut self, name: &str) {
        if let Some(sink) = self.sinks.get_mut(name) {
            sink.play();
        }
    }
    
    pub fn set_volume(&mut self, name: &str, volume: f32) {
        if let Some(sink) = self.sinks.get_mut(name) {
            sink.set_volume(volume);
        }
    }
}

// Legacy type aliases for compatibility
pub type Sound = Vec<u8>;
pub type Music = Vec<u8>;

// Legacy functions for compatibility
pub fn init(_number_of_channels: i32) {
    // No initialization needed for rodio
}

pub fn close() {
    // No cleanup needed for rodio
}

pub fn load_music(_file: &String) -> Music {
    // This function is deprecated, use AudioManager instead
    Vec::new()
}

pub fn play_sound(_sound_fx: Sound, _channel: i32) {
    // This function is deprecated, use AudioManager instead
}
