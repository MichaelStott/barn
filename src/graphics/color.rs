#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {

    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub const LIGHT_GRAY: Color = Color::new(0.75, 0.75, 0.75, 1.0);
    pub const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
    pub const DARK_GRAY: Color = Color::new(0.25, 0.25, 0.25, 1.0);
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);

    pub const CLEAR: Color = Color::new(0.0, 0.0, 0.0, 0.0);

    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
    pub const NAVY: Color = Color::new(0.0, 0.0, 0.5, 1.0);
    pub const ROYAL: Color = Color::new(0.254, 0.411, 0.882, 1.0);
    pub const SLATE: Color = Color::new(0.439, 0.501, 0.564, 1.0);
    pub const SKY: Color = Color::new(0.529, 0.807, 0.921, 1.0);
    pub const CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);
    pub const TEAL: Color = Color::new(0.0, 0.5, 0.5, 1.0);

    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    pub const CHARTREUSE: Color = Color::new(0.498, 1.0, 0.0, 1.0);
    pub const LIME: Color = Color::new(0.196, 0.803, 0.196, 1.0);
    pub const FOREST: Color = Color::new(0.133, 0.545, 0.133, 1.0);
    pub const OLIVE: Color = Color::new(0.419, 0.556, 0.137, 1.0);

    pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
    pub const GOLD: Color = Color::new(1.0, 0.843, 0.0, 1.0);
    pub const GOLDENROD: Color = Color::new(0.854, 0.647, 0.125, 1.0);
    pub const ORANGE: Color = Color::new(1.0, 0.647, 0.0, 1.0);

    pub const BROWN: Color = Color::new(0.545, 0.270, 0.074, 1.0);
    pub const TAN: Color = Color::new(0.823, 0.705, 0.549, 1.0);
    pub const FIREBRICK: Color = Color::new(0.698, 0.133, 0.133, 1.0);

    pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    pub const SCARLET: Color = Color::new(1.0, 0.203, 0.109, 1.0);
    pub const CORAL: Color = Color::new(1.0, 0.498, 0.313, 1.0);
    pub const SALMON: Color = Color::new(0.980, 0.501, 0.447, 1.0);
    pub const PINK: Color = Color::new(0.964, 0.607, 0.015, 1.0);
    pub const MAGENTA: Color = Color::new(1.0, 0.0, 1.0, 1.0);

    pub const PURPLE: Color = Color::new(0.627, 0.125, 0.941, 1.0);
    pub const VIOLET: Color = Color::new(0.933, 0.509, 0.933, 1.0);
    pub const MAROON: Color = Color::new(0.690, 0.188, 0.376, 1.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r: r, 
            g: g, 
            b: b, 
            a: a
        }
    }

    pub fn from_hex(rgba: u32) -> Color {
        Color {
            r: ((rgba & 0xff000000) >> 24) as f32/ 255.0,
            g: ((rgba & 0x00ff0000) >> 16) as f32/ 255.0,
            b: ((rgba & 0x0000ff00) >> 8) as f32 / 255.0,
            a: ((rgba & 0x000000ff)) as f32 / 255.0
        }
    }

    pub fn from_rgb(r: u32, g: u32, b: u32) -> Color {
        Color::clamp(&mut Color {
            r: r as f32/ 255.0,
            g: g as f32/ 255.0,
            b: b as f32 / 255.0,
            a: 1.0
        })
    }

    pub fn from_rgba(r: u32, g: u32, b: u32, a: u32) -> Color {
        Color::clamp(&mut Color {
            r: r as f32/ 255.0,
            g: g as f32/ 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0
        })
    }

    pub fn clamp (color: &mut Color) -> Color {
		if color.r < 0.0 {
            color.r = 0.0;
        } else if color.r > 1.0 {
            color.r = 1.0;
        }

		if color.g < 0.0 {
            color.g = 0.0;
        } else if color.g > 1.0 {
            color.g = 1.0;
        }

		if color.b < 0.0 {
            color.b = 0.0;
        } else if color.b > 1.0 {
            color.b = 1.0;
        }

		if color.a < 0.0 {
            color.a = 0.0;
        } else if color.a > 1.0 {
            color.a = 1.0;
        }

		*color
	}
}

impl PartialEq for Color {
    fn eq(&self, _color: &Self) -> bool {
        self.r == _color.r && 
        self.g == _color.g &&
        self.b == _color.b &&
        self.a == _color.a
    }
}

