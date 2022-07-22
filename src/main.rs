mod color_names;
mod convert;
mod display;

use convert::convert_hex;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => println!("show help."),
        _ => {
            args.remove(0);
            for arg in args {
                convert_hex(arg);
            }
        }
    }
}
