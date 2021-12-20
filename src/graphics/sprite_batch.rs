extern crate gl;

struct SpriteBatch {}

impl SpriteBatch {
    pub fn set_clear_color(&mut self) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}