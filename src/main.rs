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

fn main() -> Result<(), std::io::Error> {
    let mut args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => println!("{}", HELP),
        2 => {
            args.remove(0);
            for arg in args {
                match Color::from_hex(&arg) {
                    Ok(color) => color.print_color(),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
        _ => {
            if &args[1] == "-r" || &args[1] == "--rgb" {
                let v = &args[2..args.len()];
                let times = v.len() % 3;
                if times == 0 {
                    let mut rgb = vec![];
                    for i in 0..(v.len() / 3) {
                        for n in &v[((i * 3)..=2 + (i * 3))] {
                            let parsed = n.parse::<u8>();
                            match parsed {
                                Ok(x) => rgb.push(x),
                                Err(_) => {
                                    eprintln!("Error: Invalid input => {}", n);
                                    return Ok(());
                                }
                            }
                        }
                        let color = Color::from_rgb(rgb[0], rgb[1], rgb[2]);
                        color.print_color();
                        rgb = vec![];
                    }
                } else {
                    eprintln!("Error: Invalid inputs.");
                }
            } else {
                eprintln!("Error: Invalid inputs.");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        let color = Color::from_hex("7CB9E8").unwrap();
        assert_eq!(color.hex, "7cb9e8".to_string());
        assert_eq!(color.name, Some("Aero".to_string()));
        assert_eq!(color.rgb, vec![124, 185, 232]);
    }

    #[test]
    fn test_from_rgb() {
        let color = Color::from_rgb(175, 143, 44);
        assert_eq!(color.hex, "af8f2c".to_string());
        assert_eq!(color.name, Some("Alpine".to_string()));
        assert_eq!(color.rgb, vec![175, 143, 44]);
    }
}
