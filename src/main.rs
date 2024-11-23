use termion::color;

fn main() {
    let size = 7;
    let sha1 = "5a98d8";
    fill(size, &sha1);
}

fn fill(num: usize, sha: &str) {
    let col = sha2col(sha);
    let fill = " ".repeat(num);
    println!("{}{}{}", color::Bg(col), fill, color::Bg(color::Reset));
    print!("{}{}{}", color::Bg(col), fill, color::Bg(color::Reset));
    println!(" Your commit colour is {}", "unknown");
    println!("{}{}{}", color::Bg(col), fill, color::Bg(color::Reset));
    println!("#{}", sha);
}

fn sha2col(sha: &str) -> color::Rgb {
    let r = u8::from_str_radix(&sha[0..2], 16).unwrap();
    let g = u8::from_str_radix(&sha[2..4], 16).unwrap();
    let b = u8::from_str_radix(&sha[4..6], 16).unwrap();
    color::Rgb(r, g, b)
}