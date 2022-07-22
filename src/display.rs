use colored::ColoredString;

use colored::Colorize;

use crate::color_names::search_color_name;

pub const LEN: usize = 36;

pub fn make_bc(rgb: &[u8]) -> ColoredString {
    "█".repeat(LEN).truecolor(rgb[0], rgb[1], rgb[2])
}

pub fn make_hex(rgb: &[u8], hex: &str) -> ColoredString {
    let hex = format!("#{}", hex);
    match search_color_name(&hex) {
        Some(name) => format!("[{}] {}", hex, name).truecolor(rgb[0], rgb[1], rgb[2]),
        None => hex.truecolor(rgb[0], rgb[1], rgb[2]),
    }
}

pub fn convert_to_hsl(rgb: &[u8]) -> [f64; 3] {
    let hsl = hsl::HSL::from_rgb(rgb);
    [round_f(hsl.h), round_f(hsl.s), round_f(hsl.l)]
}

pub fn print_rgb(rgb: &[u8], hsl: &[f64; 3]) {
    let text = if hsl[2] < 0.5 {
        format!("r: {} g: {} b: {}", rgb[0], rgb[1], rgb[2])
            .white()
            .on_truecolor(rgb[0], rgb[1], rgb[2])
    } else {
        format!("r: {} g: {} b: {}", rgb[0], rgb[1], rgb[2])
            .black()
            .on_truecolor(rgb[0], rgb[1], rgb[2])
    };

    let len = text.len();
    let diff = LEN - len;
    let prefix = "█".repeat(diff).truecolor(rgb[0], rgb[1], rgb[2]);
    print!("{}", prefix);
    println!("{}", text);
}

pub fn print_rgb_ratio(rgb: &[u8], hsl: &[f64; 3]) {
    let r_ratio: f64 = round_n(rgb[0]);
    let g_ratio: f64 = round_n(rgb[1]);
    let b_ratio: f64 = round_n(rgb[2]);
    let text = if hsl[2] < 0.5 {
        format!("({}, {}, {})", r_ratio, g_ratio, b_ratio)
            .white()
            .on_truecolor(rgb[0], rgb[1], rgb[2])
    } else {
        format!("({}, {}, {})", r_ratio, g_ratio, b_ratio)
            .black()
            .on_truecolor(rgb[0], rgb[1], rgb[2])
    };

    let len = text.len();
    let diff = LEN - len;
    let prefix = "█".repeat(diff).truecolor(rgb[0], rgb[1], rgb[2]);
    print!("{}", prefix);
    println!("{}", text);
}
pub fn print_hsl(rgb: &[u8], hsl: &[f64; 3]) {
    let text = if hsl[2] < 0.5 {
        format!("h: {} s: {} l: {}", hsl[0], hsl[1], hsl[2])
            .white()
            .on_truecolor(rgb[0], rgb[1], rgb[2])
    } else {
        format!("h: {} s: {} l: {}", hsl[0], hsl[1], hsl[2])
            .black()
            .on_truecolor(rgb[0], rgb[1], rgb[2])
    };

    let len = text.len();
    let diff = LEN - len;
    let prefix = "█".repeat(diff).truecolor(rgb[0], rgb[1], rgb[2]);
    print!("{}", prefix);
    println!("{}", text);
}

fn round_f(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

fn round_n(n: u8) -> f64 {
    ((n as f64 / 255.0) * 100.0).round() / 100.0
}
