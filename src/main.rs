use termion::color;

fn main() {
    let sample = "#7cb0a1";
    let sample = sample.strip_prefix('#').unwrap();
    let mut temp = "".to_string();
    let mut v = vec![];
    for (i, c) in sample.chars().enumerate() {
        if i % 2 != 0 {
            temp.push(c);
            v.push(temp);
            temp = "".to_string();
        } else {
            temp.push(c);
        }
    }
    println!("{:?}", v);

    let rgb: Vec<u8> = v
        .iter()
        .map(|value| u8::from_str_radix(value, 16).unwrap())
        .collect();
    println!(
        "{}█████████{:?}█{}",
        color::Bg(color::Rgb(rgb[0], rgb[1], rgb[2])),
        rgb,
        color::Bg(color::Reset),
    );
}
