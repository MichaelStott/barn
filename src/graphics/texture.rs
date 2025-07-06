// Texture module - placeholder for future texture loading functionality

use std::sync::Arc;
use wgpu::util::DeviceExt;
use std::fs;
use std::path::Path;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: wgpu::Extent3d,
}

impl Texture {
    pub fn create_default(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let size = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };
        
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Default Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        
        // Create a white pixel
        let pixel_data = [255, 255, 255, 255];
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &pixel_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            size,
        );
        
        Self {
            texture,
            view,
            sampler,
            size,
        }
    }
    
    pub fn from_file(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let abs_path = fs::canonicalize(Path::new(path));
        println!("Attempting to load image from: {:?}", abs_path);
        let img = image::open(path)?;
        println!("Image color type for {}: {:?}", path, img.color());
        let rgba = img.into_rgba8();
        let dimensions = rgba.dimensions();
        println!("Loaded image: {} ({}x{})", path, dimensions.0, dimensions.1);
        let bytes = rgba.as_raw();
        println!("First 16 bytes: {:?}", &bytes[..16.min(bytes.len())]);
        // Debug: count non-zero pixels and print alpha histogram
        let mut non_zero_pixels = 0;
        let mut alpha_hist = [0u32; 256];
        for chunk in bytes.chunks(4) {
            if chunk[0] != 0 || chunk[1] != 0 || chunk[2] != 0 || chunk[3] != 0 {
                non_zero_pixels += 1;
            }
            alpha_hist[chunk[3] as usize] += 1;
        }
        println!("Non-zero RGBA pixels: {} / {}", non_zero_pixels, bytes.len() / 4);
        println!("Alpha histogram (nonzero only):");
        for (i, &count) in alpha_hist.iter().enumerate() {
            if count > 0 {
                println!("  alpha={}: {}", i, count);
            }
        }
        
        // Debug: show some sample pixels from the image
        println!("Sample pixels from {}:", path);
        for i in 0..10.min(bytes.len() / 4) {
            let pixel = &bytes[i * 4..(i + 1) * 4];
            println!("  pixel {}: R={}, G={}, B={}, A={}", i, pixel[0], pixel[1], pixel[2], pixel[3]);
        }
        
        // Also show some pixels from the middle of the image
        let mid_pixel = bytes.len() / 8;
        for i in 0..10 {
            let pixel_idx = mid_pixel + i * 4;
            if pixel_idx + 3 < bytes.len() {
                let pixel = &bytes[pixel_idx..pixel_idx + 4];
                println!("  mid pixel {}: R={}, G={}, B={}, A={}", i, pixel[0], pixel[1], pixel[2], pixel[3]);
            }
        }
        
        // For ground texture, show pixels from the bottom half (where the visible part is)
        if path.contains("snow_ground") {
            println!("=== GROUND TEXTURE DEBUG ===");
            println!("Ground texture bottom half pixels:");
            let bottom_start = bytes.len() / 2; // Start from middle (bottom half)
            for i in 0..20 {
                let pixel_idx = bottom_start + i * 4;
                if pixel_idx + 3 < bytes.len() {
                    let pixel = &bytes[pixel_idx..pixel_idx + 4];
                    println!("  bottom pixel {}: R={}, G={}, B={}, A={}", i, pixel[0], pixel[1], pixel[2], pixel[3]);
                }
            }
            
            // Also show some pixels from the very bottom (last few rows)
            println!("Ground texture very bottom pixels:");
            let very_bottom_start = bytes.len() - (512 * 4 * 4); // Last 4 rows
            for i in 0..20 {
                let pixel_idx = very_bottom_start + i * 4;
                if pixel_idx + 3 < bytes.len() {
                    let pixel = &bytes[pixel_idx..pixel_idx + 4];
                    println!("  very bottom pixel {}: R={}, G={}, B={}, A={}", i, pixel[0], pixel[1], pixel[2], pixel[3]);
                }
            }
            println!("=== END GROUND TEXTURE DEBUG ===");
        }
        
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("Texture: {}", path)),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );
        
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        
        Ok(Self {
            texture,
            view,
            sampler,
            size,
        })
    }
} 