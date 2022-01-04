use std::collections::HashMap;

use crate::graphics::SdlRect;
use crate::graphics::SdlTexture;

pub struct SdlSprite {
    pub src_rect: SdlRect,
    pub dst_rect: SdlRect,
    pub angle: f32,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    texture: SdlTexture,
    animations: HashMap<String, SdlSpriteAnimation>,
    active_animation_name: String, 
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
    pub src_rect: SdlRect,
    pub dst_rect: SdlRect,
    pub duration: f32
}

impl SdlSprite {

    pub fn new(texture: SdlTexture, src_rect: SdlRect, dst_rect: SdlRect) -> SdlSprite {
        SdlSprite {
            texture: texture,
            src_rect: src_rect, 
            dst_rect: dst_rect,
            angle: 0.0, 
            flip_horizontal: false,
            flip_vertical: false,
            play_animation: false,
            animations: HashMap::new(),
            active_animation_name: String::from(""),
        }
    }

    pub fn play_animation(&mut self, name: String, repeat: bool) {
        if self.animations.contains_key(&name) {
            self.animations.get_mut(&name).unwrap().repeat = repeat;
            self.play_animation = true;
        }
    }

    pub fn get_src_rect(&mut self) -> SdlRect {
        let mut result = self.src_rect;
        if self.play_animation {
            let frame = self.animations.get_mut(&self.active_animation_name).unwrap().current_frame();
            result = frame.src_rect;
        }
        result
    }

    pub fn get_dst_rect(&mut self) -> SdlRect {
        let mut result = self.dst_rect;
        if self.play_animation {
            let frame = self.animations.get_mut(&self.active_animation_name).unwrap().current_frame();
            result = frame.dst_rect;
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
        if self.animation_timer > self.total_duration {
            self.animation_timer %= self.total_duration;
        }
    }

    pub fn reset(&mut self) {
        self.animation_timer = 0.0;
    }

    pub fn current_frame(&mut self) -> SdlSpriteFrame {
        // TODO: Lazy implementation. Optimize this.
        let mut index = 0;
        let mut current_duration = 0.0;
        for frame in self.frames.iter_mut() {
            if current_duration + frame.duration  >= self.animation_timer {
                break;
            } else {
                current_duration += frame.duration;
                index += 1;
            }
        }
        *self.frames.get(index).unwrap()
    }
}