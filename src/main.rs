mod color;

use colorconv::Color;
use log::debug;

use crate::color::print_color;

const HELP: &str = "
Hex color code or RGB => color code, RGB, HSL, color name(if exists).

Usage:
    `iro <hex color code>`
    `iro -r(--rgb) <r g b>`
    The color code do not need '#' at the beginning.

ex:
    `iro ffffff` or `iro 123456 333333`
    `iro -r 25 34 187` or `iro --rgb 0 0 255 120 120 240`
";

fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let mut args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => println!("{}", HELP),
        2 => {
            if args[1].len() < 6 {
                eprintln!("Error: Too short code.");
                return Ok(());
            }
            match Color::try_from(args[1].as_ref()) {
                Ok(color) => print_color(&color),
                Err(e) => eprintln!("{:#?}", e),
            }
        }
        _ => {
            if &args[1] == "-s" || &args[1] == "--search" {
                let query = &args[2..args.len()];
                let query: Vec<&str> = query.iter().map(|x| x.as_str()).collect();
                let names = colorconv::find_all_by_name(query.as_slice());
                debug!("{:#?}", names);
                if names.is_none() {
                    println!("No such color name.");
                } else {
                    for name in names.unwrap() {
                        if let Ok(color) = Color::try_from(name.as_str()) {
                            print_color(&color);
                        }
                    }
                }
            } else if &args[1] == "-r" || &args[1] == "--rgb" {
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
                                    eprintln!("Error: {} => Invalid input.", n);
                                    return Ok(());
                                }
                            }
                        }
                        if rgb.len() == 3 {
                            let color = Color::from([rgb[0], rgb[1], rgb[2]]);
                            print_color(&color);
                        } else {
                            eprintln!("Error: Invalid input.");
                        }
                        rgb = vec![];
                    }
                } else {
                    eprintln!("Error: Invalid inputs.");
                }
            } else {
                args.remove(0);
                for arg in args {
                    if arg.len() < 6 {
                        eprintln!("Error: {} => Too short.", arg);
                        continue;
                    }
                    match Color::try_from(arg.as_str()) {
                        Ok(color) => print_color(&color),
                        Err(e) => eprintln!("{:#?}", e),
                    }
                }
            }
        }
    }
    Ok(())
}
