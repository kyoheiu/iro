use super::color_names::search_color_name;
use colored::Colorize;

pub const LEN: usize = 36;

pub struct Color {
    hex: String,
    name: Option<String>,
    rgb: Vec<u8>,
    hsl: [f64; 3],
}

impl Color {
    pub fn from_hex(hex: &str) -> Self {
        let mut temp = "".to_string();
        let mut v = vec![];
        for (i, c) in hex.chars().enumerate() {
            if i % 2 != 0 {
                temp.push(c);
                v.push(temp);
                temp = "".to_string();
            } else {
                temp.push(c);
            }
        }

        let name = search_color_name(hex);

        let rgb = v
            .iter()
            .map(|value| u8::from_str_radix(value, 16).unwrap())
            .collect::<Vec<u8>>();

        let hsl = convert_to_hsl(&rgb);
        Color {
            hex: hex.to_string(),
            name,
            rgb,
            hsl,
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let rgb = vec![r, g, b];
        let hex = format!("{}{}{}", to_hex(r), to_hex(g), to_hex(b));
        let name = search_color_name(&hex);
        let hsl = convert_to_hsl(&rgb);
        Color {
            hex,
            name,
            rgb,
            hsl,
        }
    }

    fn print_background(&self) {
        println!(
            "{}",
            "█"
                .repeat(LEN)
                .truecolor(self.rgb[0], self.rgb[1], self.rgb[2])
        );
    }

    fn print_hex(&self) {
        let text = match &self.name {
            Some(name) => {
                format!("[#{}] {}", self.hex, name)
            }
            None => format!("[#{}]", self.hex),
        };
        if self.hsl[2] < 0.5 {
            let len = text.len();
            let diff = "█".repeat(LEN - len);
            let text = text
                .on_white()
                .truecolor(self.rgb[0], self.rgb[1], self.rgb[2]);
            let diff = diff.white();

            print!("{}", text);
            println!("{}", diff);
        } else {
            let text = text
                .on_black()
                .truecolor(self.rgb[0], self.rgb[1], self.rgb[2]);
            println!("{}", text);
        }
    }

    fn print_rgb(&self) {
        let text = if self.hsl[2] < 0.5 {
            format!("r: {} g: {} b: {}", self.rgb[0], self.rgb[1], self.rgb[2])
                .white()
                .on_truecolor(self.rgb[0], self.rgb[1], self.rgb[2])
        } else {
            format!("r: {} g: {} b: {}", self.rgb[0], self.rgb[1], self.rgb[2])
                .black()
                .on_truecolor(self.rgb[0], self.rgb[1], self.rgb[2])
        };

        let len = text.len();
        let diff = LEN - len;
        let prefix = "█"
            .repeat(diff)
            .truecolor(self.rgb[0], self.rgb[1], self.rgb[2]);
        print!("{}", prefix);
        println!("{}", text);
    }

    fn print_rgb_ratio(&self) {
        let r_ratio: f64 = round_n(self.rgb[0]);
        let g_ratio: f64 = round_n(self.rgb[1]);
        let b_ratio: f64 = round_n(self.rgb[2]);
        let text = if self.hsl[2] < 0.5 {
            format!("({}, {}, {})", r_ratio, g_ratio, b_ratio)
                .white()
                .on_truecolor(self.rgb[0], self.rgb[1], self.rgb[2])
        } else {
            format!("({}, {}, {})", r_ratio, g_ratio, b_ratio)
                .black()
                .on_truecolor(self.rgb[0], self.rgb[1], self.rgb[2])
        };

        let len = text.len();
        let diff = LEN - len;
        let prefix = "█"
            .repeat(diff)
            .truecolor(self.rgb[0], self.rgb[1], self.rgb[2]);
        print!("{}", prefix);
        println!("{}", text);
    }

    fn print_hsl(&self) {
        let text = if self.hsl[2] < 0.5 {
            format!("h: {} s: {} l: {}", self.hsl[0], self.hsl[1], self.hsl[2])
                .white()
                .on_truecolor(self.rgb[0], self.rgb[1], self.rgb[2])
        } else {
            format!("h: {} s: {} l: {}", self.hsl[0], self.hsl[1], self.hsl[2])
                .black()
                .on_truecolor(self.rgb[0], self.rgb[1], self.rgb[2])
        };

        let len = text.len();
        let diff = LEN - len;
        let prefix = "█"
            .repeat(diff)
            .truecolor(self.rgb[0], self.rgb[1], self.rgb[2]);
        print!("{}", prefix);
        println!("{}", text);
    }

    pub fn print_color(&self) {
        self.print_background();
        self.print_hex();
        self.print_rgb();
        self.print_rgb_ratio();
        self.print_hsl();
        self.print_background();
    }
}

fn round_f(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

fn round_n(n: u8) -> f64 {
    ((n as f64 / 255.0) * 100.0).round() / 100.0
}

fn convert_to_hsl(rgb: &[u8]) -> [f64; 3] {
    let hsl = hsl::HSL::from_rgb(rgb);
    [round_f(hsl.h), round_f(hsl.s), round_f(hsl.l)]
}

fn to_hex(n: u8) -> String {
    let mut n = n;
    let mut i = 0;
    while n >= 16 {
        n -= 16;
        i += 1;
    }

    let mut result = "".to_string();
    result.push(map_16(i));
    result.push(map_16(n));
    result
}

fn map_16(n: u8) -> char {
    match n {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        _ => 'f',
    }
}
