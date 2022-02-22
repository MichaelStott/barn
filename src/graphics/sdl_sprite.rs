use std::collections::HashMap;

use crate::graphics::SdlRect;

pub struct SdlSprite {
    pub animations: HashMap<String, SdlSpriteAnimation>,
    active_animation_name: String, 
    src: SdlRect,
    dst: SdlRect,
    play_animation: bool
}

#[derive(Clone)]
pub struct SdlSpriteAnimation {
    frames: Vec<SdlSpriteFrame>,
    total_duration: f32,
    repeat: bool,
    animation_timer: f32
}

#[derive(Clone, Copy)]
pub struct SdlSpriteFrame {
    pub src: SdlRect,
    pub dst: SdlRect,
    pub duration: f32
}

impl SdlSprite {

    pub fn new(src: SdlRect, dst: SdlRect) -> SdlSprite {
        SdlSprite {
            src: src, 
            dst: dst,
            play_animation: false,
            animations: HashMap::new(),
            active_animation_name: String::from(""),
        }
    }

    pub fn play_animation(&mut self, name: String, repeat: bool) {
        if self.animations.contains_key(&name) {
            self.active_animation_name = name.clone();
            self.animations.get_mut(&name).unwrap().repeat = repeat; 
            self.play_animation = true;
        }
    }

    pub fn get_src_rect(&mut self) -> SdlRect {
        let mut result = self.src;
        if self.play_animation {
            let frame = self.animations.get_mut(&self.active_animation_name).unwrap().current_frame();
            result = frame.src;
        }
        result
    }

    pub fn get_dst_rect(&mut self) -> SdlRect {
        let mut result = self.dst;
        if self.play_animation {
            let frame = self.animations.get_mut(&self.active_animation_name).unwrap().current_frame();
            result = frame.dst;
        }
        result
    }
}

impl SdlSpriteAnimation {

    pub fn new(frames: Vec<SdlSpriteFrame>, repeat: bool) -> SdlSpriteAnimation {
        let mut duration = 0.0;
        for frame in frames.iter() { duration += frame.duration; }
        SdlSpriteAnimation {
            frames: frames,
            total_duration: duration,
            repeat: repeat,
            animation_timer: 0.0,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.animation_timer += dt;
        if self.animation_timer > self.total_duration && self.repeat {
            self.animation_timer %= self.total_duration;
        }
    }

    pub fn reset(&mut self) {
        self.animation_timer = 0.0;
    }

    pub fn current_frame(&mut self) -> SdlSpriteFrame {
        let mut index = 0;
        let mut current_duration = 0.0;
        let length = self.frames.len();
        for frame in self.frames.iter_mut() {
            if current_duration + frame.duration  >= self.animation_timer {
                break;
            } else {
                current_duration += frame.duration;
                if index + 1 < length {
                    index += 1;
                }
                
            }
        }
        *self.frames.get(index).unwrap()
    }
}