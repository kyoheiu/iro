use colorconv::*;
use colored::Colorize;

pub const LEN: usize = 42;

fn print_background(color: &Color) {
    println!(
        "{}",
        "█"
            .repeat(LEN)
            .truecolor(color.rgb[0], color.rgb[1], color.rgb[2])
    );
}

fn print_hex(color: &Color) {
    let text = match &color.name {
        Some(name) => {
            format!("[#{}] {}", color.hex, name)
        }
        None => format!("[#{}]", color.hex),
    };
    if color.hsl[2] < 0.5 {
        let len = text.len();
        let diff = "█".repeat(LEN - len);
        let text = text
            .on_white()
            .truecolor(color.rgb[0], color.rgb[1], color.rgb[2]);
        let diff = diff.white();

        print!("{}", text);
        println!("{}", diff);
    } else {
        let text = text
            .on_black()
            .truecolor(color.rgb[0], color.rgb[1], color.rgb[2]);
        println!("{}", text);
    }
}

fn print_rgb(color: &Color) {
    let text = if color.hsl[2] < 0.5 {
        format!(
            "r: {} g: {} b: {}",
            color.rgb[0], color.rgb[1], color.rgb[2]
        )
        .white()
        .on_truecolor(color.rgb[0], color.rgb[1], color.rgb[2])
    } else {
        format!(
            "r: {} g: {} b: {}",
            color.rgb[0], color.rgb[1], color.rgb[2]
        )
        .black()
        .on_truecolor(color.rgb[0], color.rgb[1], color.rgb[2])
    };

    let len = text.len();
    let diff = LEN - len;
    let prefix = "█"
        .repeat(diff)
        .truecolor(color.rgb[0], color.rgb[1], color.rgb[2]);
    print!("{}", prefix);
    println!("{}", text);
}

fn print_rgb_ratio(color: &Color) {
    let r_ratio: f64 = round_n(color.rgb[0]);
    let g_ratio: f64 = round_n(color.rgb[1]);
    let b_ratio: f64 = round_n(color.rgb[2]);
    let text = if color.hsl[2] < 0.5 {
        format!("({}, {}, {})", r_ratio, g_ratio, b_ratio)
            .white()
            .on_truecolor(color.rgb[0], color.rgb[1], color.rgb[2])
    } else {
        format!("({}, {}, {})", r_ratio, g_ratio, b_ratio)
            .black()
            .on_truecolor(color.rgb[0], color.rgb[1], color.rgb[2])
    };

    let len = text.len();
    let diff = LEN - len;
    let prefix = "█"
        .repeat(diff)
        .truecolor(color.rgb[0], color.rgb[1], color.rgb[2]);
    print!("{}", prefix);
    println!("{}", text);
}

fn print_hsl(color: &Color) {
    let text = if color.hsl[2] < 0.5 {
        format!(
            "h: {} s: {} l: {}",
            color.hsl[0], color.hsl[1], color.hsl[2]
        )
        .white()
        .on_truecolor(color.rgb[0], color.rgb[1], color.rgb[2])
    } else {
        format!(
            "h: {} s: {} l: {}",
            color.hsl[0], color.hsl[1], color.hsl[2]
        )
        .black()
        .on_truecolor(color.rgb[0], color.rgb[1], color.rgb[2])
    };

    let len = text.len();
    let diff = LEN - len;
    let prefix = "█"
        .repeat(diff)
        .truecolor(color.rgb[0], color.rgb[1], color.rgb[2]);
    print!("{}", prefix);
    println!("{}", text);
}

pub fn print_color(color: &Color) {
    print_background(color);
    print_hex(color);
    print_rgb(color);
    print_rgb_ratio(color);
    print_hsl(color);
    print_background(color);
    println!();
}

fn round_f(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

fn round_n(n: u8) -> f64 {
    ((n as f64 / 255.0) * 100.0).round() / 100.0
}
