use crossterm::style::{Color, Stylize};

struct Col {
    r: i32,
    g: i32,
    b: i32,
}

impl Col {
    fn new(sha: &str) -> Self {
        Self {
            r: i32::from_str_radix(&sha[0..2], 16).unwrap(),
            g: i32::from_str_radix(&sha[2..4], 16).unwrap(),
            b: i32::from_str_radix(&sha[4..6], 16).unwrap(),
        }
    }

    fn rgb(&self) -> Color {
        Color::Rgb {
            r: self.r as u8,
            g: self.g as u8,
            b: self.b as u8,
        }
    }
}

fn main() {
    let sha1 = "5a98d8";
    fill(sha1, "Unknown");
}

fn fill(sha: &str, name: &str) {
    let num = 7;
    let color = Col::new(sha);
    let block = " ".repeat(num).on(color.rgb());
    println!("{}", block);
    println!("{} Your commit colour is {}", block, name);
    println!("{}", block);
    println!("#{}", sha);
}
