use colors_transform::{Color, Hsl, Rgb};

pub fn rgb_to_hsv(r: u8, g: u8, b:u8) -> (i64, i64, i64) {
    let rgb = Rgb::from(r as f32, g as f32, b as f32);
    let hsv = rgb.to_hsl();
    (hsv.get_hue() as i64, hsv.get_saturation() as i64, hsv.get_lightness() as i64)
}

pub fn hsv_to_rgb(h: i64, s: i64, v: i64) -> (u8, u8, u8) {
    let hsv = Hsl::from(h as f32, s as f32, v as f32);
    let rgb = hsv.to_rgb();
    (rgb.get_red() as u8, rgb.get_green() as u8, rgb.get_blue() as u8)
}