use crossterm::style::{Color, Stylize};
use std::fs;

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
    let file = "all-colors.csv";
    let file = fs::read_to_string(file).expect("Error reading file");
    let mut names: Vec<String> = Vec::new();
    let mut rgbs: Vec<Col> = Vec::new();
    for line in file.lines() {
        let code = &line[0..6];
        let name = &line[7..];
        names.push(name.to_string());
        rgbs.push(Col::new(code));
    }
    let idx_closest = closest(&Col::new(sha1), &rgbs);
    fill(sha1, &rgbs[idx_closest], &names[idx_closest]);
}

fn fill(sha: &str, color: &Col, name: &str) {
    let num = 7;
    let block = " ".repeat(num).on(color.rgb());
    println!("{}", block);
    println!("{} Your commit colour is {}", block, name);
    println!("{}", block);
    println!("#{}", sha);
}

fn closest(rgb: &Col, rgbs: &[Col]) -> usize {
    let mut max = f32::MAX;
    let mut idx = 0;
    for (i, item) in rgbs.iter().enumerate() {
        let dist = distance(rgb, item);
        if dist < max {
            max = dist;
            idx = i;
        }
    }
    idx
}

fn distance(c1: &Col, c2: &Col) -> f32 {
    f32::sqrt(((c1.r - c2.r).pow(2) + (c1.g - c2.g).pow(2) + (c1.b - c2.b).pow(2)) as f32)
}
