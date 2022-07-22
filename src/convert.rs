use super::display::*;

pub fn convert_hex(arg: String) {
    let mut temp = "".to_string();
    let mut v = vec![];
    for (i, c) in arg.chars().enumerate() {
        if i % 2 != 0 {
            temp.push(c);
            v.push(temp);
            temp = "".to_string();
        } else {
            temp.push(c);
        }
    }

    let rgb = v
        .iter()
        .map(|value| u8::from_str_radix(value, 16).unwrap())
        .collect::<Vec<u8>>();

    let hsl = convert_to_hsl(&rgb);

    let bc = make_bc(&rgb);
    let hex = make_hex(&rgb, &arg);

    println!("{}", bc);
    println!("{}", hex);
    print_rgb(&rgb, &hsl);
    print_rgb_ratio(&rgb, &hsl);
    print_hsl(&rgb, &hsl);
    println!("{}", bc);
}
