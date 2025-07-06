pub mod font_details;

// Simple font types for compatibility
pub type Font = String; // Placeholder for now

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // No initialization needed for now
    Ok(())
}

pub fn close() {
    // No cleanup needed for now
}