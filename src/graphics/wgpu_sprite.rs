use std::collections::HashMap;
use wgpu::util::DeviceExt;
use std::sync::Arc;

use crate::graphics::Rect;
use crate::graphics::wgpu_renderer::TextVertex;



pub struct WgpuSprite {
    pub src: Rect,
    pub dst: Rect,
    pub color: [f32; 4],
    pub texture_path: Option<String>,
    pub texture_size: Option<(u32, u32)>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub vertex_count: usize,
}

#[derive(Clone)]
pub struct WgpuSpriteAnimation {
    frames: Vec<WgpuSpriteFrame>,
    total_duration: f32,
    repeat: bool,
    animation_timer: f32,
    play: bool,
}

#[derive(Clone, Copy)]
pub struct WgpuSpriteFrame {
    pub src: Rect,
    pub dst: Rect,
    pub duration: f32,
}

impl WgpuSprite {
    pub fn new(
        src: Rect,
        dst: Rect,
        color: [f32; 4],
        device: &wgpu::Device,
        texture_path: String,
        texture_size: (u32, u32),
        screen_width: f32,
        screen_height: f32,
    ) -> WgpuSprite {
        let sprite = WgpuSprite {
            src,
            dst,
            color,
            texture_path: Some(texture_path),
            texture_size: Some(texture_size),
            vertex_buffer: None,
            vertex_count: 6,
        };
        let buffer = sprite.create_vertex_buffer(device, screen_width, screen_height);
        WgpuSprite {
            vertex_buffer: Some(buffer),
            ..sprite
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, device: &wgpu::Device, _texture_size: (u32, u32), screen_width: f32, screen_height: f32) {
        self.dst.0 = x;
        self.dst.1 = y;
        let buffer = self.create_vertex_buffer(device, screen_width, screen_height);
        self.vertex_buffer = Some(buffer);
        self.vertex_count = 6;
    }

    pub fn set_size(&mut self, width: f32, height: f32, device: &wgpu::Device, _texture_size: (u32, u32), screen_width: f32, screen_height: f32) {
        self.dst.2 = width;
        self.dst.3 = height;
        let buffer = self.create_vertex_buffer(device, screen_width, screen_height);
        self.vertex_buffer = Some(buffer);
        self.vertex_count = 6;
    }

    pub fn get_texture_path(&self) -> Option<&String> {
        self.texture_path.as_ref()
    }

    pub fn create_vertex_buffer(&self, device: &wgpu::Device, screen_width: f32, screen_height: f32) -> wgpu::Buffer {
        let (tex_w, tex_h) = self.texture_size.unwrap_or((1280, 1280));
        let u0 = self.src.0 / tex_w as f32;
        let v0 = self.src.1 / tex_h as f32;
        let u1 = (self.src.0 + self.src.2) / tex_w as f32;
        let v1 = (self.src.1 + self.src.3) / tex_h as f32;
        println!("[DEBUG] Sprite UVs: u0={}, v0={}, u1={}, v1={}", u0, v0, u1, v1);
        println!("[DEBUG] Sprite texture_path: {:?}", self.texture_path);
        let x = self.dst.0;
        let y = self.dst.1;
        let w = self.dst.2;
        let h = self.dst.3;
        
        println!("[DEBUG] create_vertex_buffer: src={:?}, dst={:?}, texture_size={:?}", &self.src, &self.dst, (tex_w, tex_h));
        println!("[DEBUG] UV coordinates: u0={}, v0={}, u1={}, v1={}", u0, v0, u1, v1);
        println!("[DEBUG] Position coordinates: x={}, y={}, w={}, h={}", x, y, w, h);
        
        let x1 = (x / screen_width) * 2.0 - 1.0;
        let y1 = 1.0 - (y / screen_height) * 2.0;
        let x2 = ((x + w) / screen_width) * 2.0 - 1.0;
        let y2 = 1.0 - ((y + h) / screen_height) * 2.0;
        
        println!("[DEBUG] NDC coordinates: x1={}, y1={}, x2={}, y2={}", x1, y1, x2, y2);
        
        // Create sprite vertices (two triangles)
        let vertices = vec![
            TextVertex { position: [x1, y1], color: self.color, tex_coords: [u0, v0] },
            TextVertex { position: [x2, y1], color: self.color, tex_coords: [u1, v0] },
            TextVertex { position: [x1, y2], color: self.color, tex_coords: [u0, v1] },
            TextVertex { position: [x2, y1], color: self.color, tex_coords: [u1, v0] },
            TextVertex { position: [x2, y2], color: self.color, tex_coords: [u1, v1] },
            TextVertex { position: [x1, y2], color: self.color, tex_coords: [u0, v1] },
        ];
        
        println!("[DEBUG] Vertex positions: TL={:?}, TR={:?}, BL={:?}, BR={:?}", 
                 vertices[0].position, vertices[1].position, vertices[2].position, vertices[4].position);
        
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
}

impl WgpuSpriteAnimation {
    pub fn new(frames: Vec<WgpuSpriteFrame>, repeat: bool) -> WgpuSpriteAnimation {
        let mut duration = 0.0;
        for frame in frames.iter() {
            duration += frame.duration;
        }
        WgpuSpriteAnimation {
            frames,
            total_duration: duration,
            repeat,
            animation_timer: 0.0,
            play: true,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        if self.play {
            self.animation_timer += dt;
            if self.animation_timer > self.total_duration && self.repeat {
                self.animation_timer %= self.total_duration;
            }
        }
    }

    pub fn current_frame(&mut self) -> WgpuSpriteFrame {
        let mut index = 0;
        let mut current_duration = 0.0;
        let length = self.frames.len();
        
        for frame in self.frames.iter() {
            if current_duration + frame.duration >= self.animation_timer {
                break;
            } else {
                current_duration += frame.duration;
                if index + 1 < length {
                    index += 1;
                }
            }
        }
        self.frames[index]
    }

    pub fn play(&mut self) {
        self.play = true;
    }

    pub fn pause(&mut self) {
        self.play = false;
    }

    pub fn reset(&mut self) {
        self.animation_timer = 0.0;
    }
}

impl Clone for WgpuSprite {
    fn clone(&self) -> Self {
        WgpuSprite {
            src: self.src,
            dst: self.dst,
            color: self.color,
            texture_path: self.texture_path.clone(),
            texture_size: self.texture_size,
            vertex_buffer: None,
            vertex_count: self.vertex_count,
        }
    }
} 