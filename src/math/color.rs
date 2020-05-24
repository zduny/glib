#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[allow(dead_code)]
impl Color {
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

#[allow(dead_code)]
pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r, g, b, a }
}

#[allow(dead_code)]
pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    rgba(r, g, b, 1.0)
}

#[allow(dead_code)]
const SRGB_ALPHA: f32 = 0.055;

#[allow(dead_code)]
fn srgb_to_linear(channel: f32) -> f32 {
    if channel <= 0.04045 {
        channel / 12.92
    } else {
        ((channel + SRGB_ALPHA) / (1.0 + SRGB_ALPHA)).powf(2.4)
    }
}

#[allow(dead_code)]
pub fn srgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color {
        r: srgb_to_linear(r),
        g: srgb_to_linear(g),
        b: srgb_to_linear(b),
        a,
    }
}

#[allow(dead_code)]
pub fn srgb(r: f32, g: f32, b: f32) -> Color {
    srgba(r, g, b, 1.0)
}

pub fn hex(value: &str) -> Color {
    assert!(value.starts_with("#"));
    assert!(value.len() == 7);

    let without_prefix = value.trim_start_matches("#");

    let (r, rest) = without_prefix.split_at(2);
    let (g, b) = rest.split_at(2);

    let r_f32 = u8::from_str_radix(r, 16).unwrap() as f32 / 255.0;
    let g_f32 = u8::from_str_radix(g, 16).unwrap() as f32 / 255.0;
    let b_f32 = u8::from_str_radix(b, 16).unwrap() as f32 / 255.0;

    srgb(r_f32, g_f32, b_f32)
}
