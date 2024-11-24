use crossterm::style::{Color, Stylize};

fn main() {
    let sha1 = "5a98d8";
    fill(sha1, "Unknown");
}

fn fill(sha: &str, name: &str) {
    let num = 7;
    let col = sha2col(sha);
    let block = " ".repeat(num).on(col);
    println!("{}", block);
    println!("{} Your commit colour is {}", block, name);
    println!("{}", block);
    println!("#{}", sha);
}

fn sha2col(sha: &str) -> Color {
    let r = u8::from_str_radix(&sha[0..2], 16).unwrap();
    let g = u8::from_str_radix(&sha[2..4], 16).unwrap();
    let b = u8::from_str_radix(&sha[4..6], 16).unwrap();
    Color::Rgb { r, g, b }
}
