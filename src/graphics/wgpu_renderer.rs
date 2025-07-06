use std::sync::Arc;
use winit::window::Window;
use wgpu::util::DeviceExt;
use ab_glyph::{Font, FontArc, Glyph, point, PxScale, Rect};
use crate::graphics::texture::Texture;
use crate::graphics::wgpu_sprite::WgpuSprite;
use crate::graphics::Rect as BarnRect;

pub enum DrawCommand {
    Sprite(WgpuSprite),
    Rect { x: i32, y: i32, width: u32, height: u32, color: [f32; 4] },
}

pub struct WgpuRenderer {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub sprite_pipeline: wgpu::RenderPipeline,
    pub text_pipeline: wgpu::RenderPipeline,
    pub rectangle_pipeline: wgpu::RenderPipeline,
    pub surface_format: wgpu::TextureFormat,
    pub clear_color: wgpu::Color,
    pub text_vertex_buffer: wgpu::Buffer,
    pub text_index_buffer: wgpu::Buffer,
    pub text_vertices: Vec<TextVertex>,
    pub current_text_vertex_buffer: Option<wgpu::Buffer>,
    pub font_atlas: Option<FontAtlas>,
    pub draw_commands: Vec<DrawCommand>,
    pub sprite_bind_group_layout: wgpu::BindGroupLayout,
    pub textures: std::collections::HashMap<String, Texture>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct RectangleVertex {
    position: [f32; 2],
    color: [f32; 4],
}

impl TextVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 2]>() + std::mem::size_of::<[f32; 4]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

impl RectangleVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RectangleVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct GlyphInfo {
    pub advance: f32,
    pub uv_rect: [f32; 4], // [u0, v0, u1, v1]
    pub size: [f32; 2],
    pub offset: [f32; 2],
}

pub struct FontAtlas {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
    pub glyphs: std::collections::HashMap<char, GlyphInfo>,
    pub width: u32,
    pub height: u32,
}

impl WgpuRenderer {
    pub fn new(window: &Window) -> Result<(Self, wgpu::Surface, wgpu::SurfaceConfiguration), Box<dyn std::error::Error>> {
        let size = window.inner_size();
        
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });
        
        let surface = unsafe { instance.create_surface(window) }?;
        
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })).ok_or("Failed to find an appropriate adapter")?;
        
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ))?;
        
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sprite Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/sprite.wgsl").into()),
        });
        
        let sprite_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("Sprite Bind Group Layout"),
        });
        
        let sprite_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sprite Pipeline"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&sprite_bind_group_layout],
                push_constant_ranges: &[],
                label: Some("Sprite Pipeline Layout"),
            })),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[TextVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        // Create text rendering shader
        let text_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Text Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/text.wgsl").into()),
        });
        
        // Create text bind group layout for the font atlas
        let text_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("Text Bind Group Layout"),
        });
        
        let text_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&text_bind_group_layout],
            push_constant_ranges: &[],
            label: Some("Text Pipeline Layout"),
        });
        
        let text_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Pipeline"),
            layout: Some(&text_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &text_shader,
                entry_point: "vs_main",
                buffers: &[TextVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &text_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        // Create rectangle shader and pipeline
        let rectangle_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Rectangle Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/rectangle.wgsl").into()),
        });
        
        let rectangle_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rectangle Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &rectangle_shader,
                entry_point: "vs_main",
                buffers: &[RectangleVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &rectangle_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        // Create text rendering resources
        let text_vertices = vec![
            TextVertex { position: [-0.5, -0.5], color: [1.0, 1.0, 1.0, 1.0], tex_coords: [0.0, 0.0] },
            TextVertex { position: [ 0.5, -0.5], color: [1.0, 1.0, 1.0, 1.0], tex_coords: [1.0, 0.0] },
            TextVertex { position: [ 0.5,  0.5], color: [1.0, 1.0, 1.0, 1.0], tex_coords: [1.0, 1.0] },
            TextVertex { position: [-0.5,  0.5], color: [1.0, 1.0, 1.0, 1.0], tex_coords: [0.0, 1.0] },
        ];
        
        let text_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Text Vertex Buffer"),
            contents: bytemuck::cast_slice(&text_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        
        let text_indices = vec![0, 1, 2, 0, 2, 3];
        let text_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Text Index Buffer"),
            contents: bytemuck::cast_slice(&text_indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        
        // Create font atlas
        let font_atlas = Self::create_font_atlas(&device, &queue, &text_bind_group_layout, "examples/resources/fonts/press-start/PressStart2P-vaV7.ttf", 48.0);

        Ok((WgpuRenderer {
            device: Arc::new(device),
            queue: Arc::new(queue),
            size,
            sprite_pipeline,
            text_pipeline,
            rectangle_pipeline,
            surface_format,
            clear_color: wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
            text_vertex_buffer,
            text_index_buffer,
            text_vertices: Vec::new(),
            current_text_vertex_buffer: None,
            font_atlas: Some(font_atlas),
            draw_commands: Vec::new(),
            sprite_bind_group_layout: sprite_bind_group_layout,
            textures: std::collections::HashMap::new(),
        }, surface, config))
    }
    
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, surface: &mut wgpu::Surface, config: &mut wgpu::SurfaceConfiguration) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            config.width = new_size.width;
            config.height = new_size.height;
            surface.configure(&self.device, config);
        }
    }
    
    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.clear_color = wgpu::Color { r, g, b, a };
    }
    
    pub fn present(&mut self, surface: &mut wgpu::Surface) {
        let frame = surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Pre-allocate sprite bind groups and rectangle vertex buffers
        let mut sprite_bind_groups = Vec::new();
        let mut rect_vertex_buffers = Vec::new();
        for cmd in &self.draw_commands {
            match cmd {
                DrawCommand::Sprite(sprite) => {
                    if let Some(texture_path) = sprite.get_texture_path() {
                        if let Some(texture) = self.textures.get(texture_path) {
                            let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                                layout: &self.sprite_bind_group_layout,
                                entries: &[
                                    wgpu::BindGroupEntry {
                                        binding: 0,
                                        resource: wgpu::BindingResource::TextureView(&texture.view),
                                    },
                                    wgpu::BindGroupEntry {
                                        binding: 1,
                                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                                    },
                                ],
                                label: Some("Sprite Bind Group"),
                            });
                            sprite_bind_groups.push(Some(bind_group));
                            rect_vertex_buffers.push(None);
                        } else {
                            sprite_bind_groups.push(None);
                            rect_vertex_buffers.push(None);
                        }
                    } else {
                        sprite_bind_groups.push(None);
                        rect_vertex_buffers.push(None);
                    }
                },
                DrawCommand::Rect { x, y, width, height, color } => {
                    let x = *x;
                    let y = *y;
                    let width = *width;
                    let height = *height;
                    let color = *color;
                    let screen_width = self.size.width as f32;
                    let screen_height = self.size.height as f32;
                    let x1 = (x as f32 / screen_width) * 2.0 - 1.0;
                    let y1 = 1.0 - (y as f32 / screen_height) * 2.0;
                    let x2 = ((x + width as i32) as f32 / screen_width) * 2.0 - 1.0;
                    let y2 = 1.0 - ((y + height as i32) as f32 / screen_height) * 2.0;
                    let vertices = [
                        RectangleVertex { position: [x1, y1], color },
                        RectangleVertex { position: [x2, y1], color },
                        RectangleVertex { position: [x2, y2], color },
                        RectangleVertex { position: [x1, y1], color },
                        RectangleVertex { position: [x2, y2], color },
                        RectangleVertex { position: [x1, y2], color },
                    ];
                    let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Rectangle Vertex Buffer (temp)"),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                    sprite_bind_groups.push(None);
                    rect_vertex_buffers.push(Some(vertex_buffer));
                }
            }
        }
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            for (i, cmd) in self.draw_commands.iter().enumerate() {
                match *cmd {
                    DrawCommand::Sprite(ref sprite) => {
                        if let Some(ref bind_group) = sprite_bind_groups[i] {
                            render_pass.set_pipeline(&self.sprite_pipeline);
                            render_pass.set_bind_group(0, bind_group, &[]);
                            render_pass.set_vertex_buffer(0, sprite.vertex_buffer.as_ref().unwrap().slice(..));
                            render_pass.draw(0..sprite.vertex_count as u32, 0..1);
                        }
                    },
                    DrawCommand::Rect { x, y, width, height, color } => {
                        if let Some(ref vertex_buffer) = rect_vertex_buffers[i] {
                            render_pass.set_pipeline(&self.rectangle_pipeline);
                            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                            render_pass.draw(0..6, 0..1);
                        }
                    },
                }
            }
            
            // Render text on top of everything
            if let Some(ref text_vertex_buffer) = self.current_text_vertex_buffer {
                if let Some(ref font_atlas) = self.font_atlas {
                    render_pass.set_pipeline(&self.text_pipeline);
                    render_pass.set_bind_group(0, &font_atlas.bind_group, &[]);
                    render_pass.set_vertex_buffer(0, text_vertex_buffer.slice(..));
                    render_pass.draw(0..self.text_vertices.len() as u32, 0..1);
                }
            }
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        self.clear_draw_commands();
    }
    
    pub fn render(&mut self, surface: &mut wgpu::Surface) {
        self.present(surface);
    }
    
    pub fn load_texture(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let texture = Texture::from_file(&self.device, &self.queue, path)?;
        self.textures.insert(path.to_string(), texture);
        Ok(())
    }
    
    pub fn draw_text(&mut self, text: &str, x: f32, y: f32, font_size: f32, color: [f32; 4]) {
        self.text_vertices.clear();
        if let Some(ref font_atlas) = self.font_atlas {
            let screen_width = self.size.width as f32;
            let screen_height = self.size.height as f32;
            
            let mut current_x = x; // accumulate in pixels
            let mut first = true;
            let mut char_count = 0;
            for c in text.chars() {
                char_count += 1;
                if let Some(glyph_info) = font_atlas.glyphs.get(&c) {
                    let char_width = glyph_info.size[0];
                    let char_height = glyph_info.size[1];
                    let offset_x = glyph_info.offset[0];
                    let offset_y = glyph_info.offset[1];
                    let px1 = current_x + offset_x;
                    let py1 = y + offset_y;
                    let px2 = px1 + char_width;
                    let py2 = py1 + char_height;
                    // Convert pixel coordinates to NDC
                    let ndc_x1 = (px1 / screen_width) * 2.0 - 1.0;
                    let ndc_y1 = 1.0 - (py1 / screen_height) * 2.0;
                    let ndc_x2 = (px2 / screen_width) * 2.0 - 1.0;
                    let ndc_y2 = 1.0 - (py2 / screen_height) * 2.0;
                    let [u0, v0, u1, v1] = glyph_info.uv_rect;
                    
                    let verts = [
                        TextVertex { position: [ndc_x1, ndc_y1], color, tex_coords: [u0, v0] },
                        TextVertex { position: [ndc_x1, ndc_y2], color, tex_coords: [u0, v1] },
                        TextVertex { position: [ndc_x2, ndc_y1], color, tex_coords: [u1, v0] },
                        TextVertex { position: [ndc_x2, ndc_y1], color, tex_coords: [u1, v0] },
                        TextVertex { position: [ndc_x1, ndc_y2], color, tex_coords: [u0, v1] },
                        TextVertex { position: [ndc_x2, ndc_y2], color, tex_coords: [u1, v1] },
                    ];
                    self.text_vertices.extend_from_slice(&verts);
                    let advance = if c == ' ' {
                        16.0 // Fixed width for space
                    } else {
                        glyph_info.advance
                    };
                    current_x += advance;
                } else {
                    println!("Character '{}' not found in atlas!", c);
                }
            }
        } else {
            println!("No font atlas available!");
        }
        if !self.text_vertices.is_empty() {
            self.current_text_vertex_buffer = Some(
                self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Text Vertex Buffer"),
                    contents: bytemuck::cast_slice(&self.text_vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                })
            );
        }
    }

    pub fn draw_sprite(&mut self, mut sprite: WgpuSprite) {
        // Always (re)create the vertex buffer if texture_size is set and vertex_buffer is None
        if sprite.vertex_buffer.is_none() && sprite.texture_size.is_some() {
            let buffer = sprite.create_vertex_buffer(
                &self.device,
                self.size.width as f32,
                self.size.height as f32
            );
            sprite.vertex_buffer = Some(buffer);
            sprite.vertex_count = 6;
        }
        self.draw_commands.push(DrawCommand::Sprite(sprite));
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: [f32; 4]) {
        self.draw_commands.push(DrawCommand::Rect { x, y, width, height, color });
    }

    pub fn clear_draw_commands(&mut self) {
        self.draw_commands.clear();
    }

    fn create_font_atlas(device: &wgpu::Device, queue: &wgpu::Queue, layout: &wgpu::BindGroupLayout, font_path: &str, font_size: f32) -> FontAtlas {
        // Check if file exists
        if !std::path::Path::new(font_path).exists() {
            panic!("Font file does not exist: {}", font_path);
        }
        
        let font_data = match std::fs::read(font_path) {
            Ok(data) => {
                data
            },
            Err(e) => panic!("Failed to read font file: {}", e),
        };
        
        let font = match FontArc::try_from_vec(font_data) {
            Ok(font) => {
                font
            },
            Err(e) => panic!("Failed to load font: {}", e),
        };
        
        let scale = PxScale::from(font_size);
        let mut charset: Vec<char> = (32u8..127u8).map(|c| c as char).collect();
        // Ensure space character is included
        if !charset.contains(&' ') { 
            charset.push(' '); 
        }
        
        // Calculate atlas size
        let mut max_width = 0;
        let mut max_height = 0;
        let mut glyphs = std::collections::HashMap::new();
        let max_glyph_width = 64;
        let max_glyph_height = 64;
        let space_glyph_width = 16;
        for &c in &charset {
            let glyph_id = font.glyph_id(c);
            let glyph = glyph_id.with_scale(scale);
            if let Some(outlined) = font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let width = bounds.width() as u32;
                let height = bounds.height() as u32;
                max_width = max_width.max(width.min(max_glyph_width));
                max_height = max_height.max(height.min(max_glyph_height));
            } else if c == ' ' {
                // Handle space: set a default width/height
                let width = (font.h_advance_unscaled(glyph_id) * scale.x).ceil() as u32;
                let width = width.min(space_glyph_width);
                let height = max_height.max(1);
                max_width = max_width.max(width);
                max_height = max_height.max(height);
            }
        }
        
        // Create atlas texture
        let atlas_width = max_width.max(1) * 16; // 16 characters per row
        let atlas_height = max_height.max(1) * 8; // 8 rows for ASCII
        
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Font Atlas"),
            size: wgpu::Extent3d {
                width: atlas_width,
                height: atlas_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        
        // Rasterize glyphs and populate atlas
        let mut atlas_data = vec![0u8; (atlas_width * atlas_height * 4) as usize]; // 4 bytes per pixel for RGBA
        let mut x_offset = 0;
        let mut y_offset = 0;
        
        for &c in &charset {
            let glyph_id = font.glyph_id(c);
            let glyph = glyph_id.with_scale(scale);
            if let Some(outlined) = font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let width = bounds.width() as u32;
                let height = bounds.height() as u32;
                
                if x_offset + width > atlas_width {
                    x_offset = 0;
                    y_offset += max_height;
                }
                
                // Rasterize glyph
                let mut glyph_data = vec![0u8; (width * height) as usize];
                outlined.draw(|x, y, coverage| {
                    let index = (y * width + x) as usize;
                    if index < glyph_data.len() {
                        glyph_data[index] = (coverage * 255.0) as u8;
                    }
                });
                
                // Copy to atlas (convert to RGBA)
                for y in 0..height {
                    for x in 0..width {
                        let atlas_index = ((y_offset + y) * atlas_width + x_offset + x) as usize * 4;
                        let glyph_index = (y * width + x) as usize;
                        if atlas_index + 3 < atlas_data.len() && glyph_index < glyph_data.len() {
                            let alpha = glyph_data[glyph_index];
                            atlas_data[atlas_index] = 255;     // R
                            atlas_data[atlas_index + 1] = 255; // G
                            atlas_data[atlas_index + 2] = 255; // B
                            atlas_data[atlas_index + 3] = alpha; // A
                        }
                    }
                }
                
                // Store glyph info
                let uv_rect = [
                    x_offset as f32 / atlas_width as f32,
                    y_offset as f32 / atlas_height as f32,
                    (x_offset + width) as f32 / atlas_width as f32,
                    (y_offset + height) as f32 / atlas_height as f32,
                ];
                
                glyphs.insert(c, GlyphInfo {
                    advance: width as f32 + 2.0, // Use width plus small padding
                    uv_rect,
                    size: [width as f32, height as f32],
                    offset: [bounds.min.x, bounds.min.y],
                });
                
                x_offset += width;
            } else if c == ' ' {
                // Add a dummy glyph for space
                let width = 16.0; // Use a reasonable space width
                let uv_rect = [0.0, 0.0, 0.0, 0.0];
                glyphs.insert(c, GlyphInfo {
                    advance: width,
                    uv_rect,
                    size: [width, 0.0],
                    offset: [0.0, 0.0],
                });
                x_offset += width as u32;
            }
        }
        
        // Upload atlas data to GPU
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &atlas_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(atlas_width * 4), // 4 bytes per pixel
                rows_per_image: Some(atlas_height),
            },
            wgpu::Extent3d {
                width: atlas_width,
                height: atlas_height,
                depth_or_array_layers: 1,
            },
        );
        
        // Create bind group for the font atlas
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Font Atlas Bind Group"),
        });
        
        FontAtlas {
            texture,
            view,
            sampler,
            bind_group,
            glyphs,
            width: atlas_width,
            height: atlas_height,
        }
    }
} 