#[derive(Eq, Copy, Clone, Hash)]
pub struct FontDetails {
    pub path: &'static str,
    pub size: u16,
}

impl PartialEq for FontDetails {
    fn eq(&self, _details: &Self) -> bool {
        self.path == _details.path && self.size == _details.size
    }
}

