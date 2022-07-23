mod color;
mod color_names;

use color::*;

const HELP: &str = "
Usage:
    `iro <hex color code(s)>`
    `iro -r(-rgb) <r g b>`
The color code do not need '#' at the beginning.

ex:
    `iro ffffff` or `iro 123456 333333`
    `iro -r 25 34 187`
";

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => println!("{}", HELP),
        _ => {
            if &args[1] == "-r" || &args[1] == "--rgb" {
                let mut rgb = vec![];
                for n in &args[2..=args.len() - 1] {
                    let n = n.parse::<u8>().unwrap();
                    rgb.push(n);
                }
                let color = Color::from_rgb(rgb[0], rgb[1], rgb[2]);
                color.print_color();
                println!();
            } else {
                args.remove(0);
                for arg in args {
                    let color = Color::from_hex(&arg);
                    color.print_color();
                    println!();
                }
            }
        }
    }
}
