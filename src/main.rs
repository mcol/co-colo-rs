// ==========================================================================
//
//  Copyright (c) 2024 Marco Colombo
//
//  Inspired by commit-colours by Bryan Braun:
//     https://github.com/sparkbox/commit-colors
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// ==========================================================================

use crossterm::style::{Color, Stylize};
use std::env;

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
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    let usage = "Usage: co-colo-rs [--oneline] <sha1>";
    if argc == 1 || argc > 3 {
        println!("{usage}");
        return;
    }
    if argc == 3 && &args[1] != "--oneline" {
        println!("Unrecognised option '{}'", &args[1]);
        println!("{usage}");
        return;
    }
    let sha1 = &args[argc - 1][0..6];
    let file = include_str!("../all-colors.csv");
    let mut names: Vec<String> = Vec::new();
    let mut rgbs: Vec<Col> = Vec::new();
    for line in file.lines() {
        let code = &line[0..6];
        let name = &line[7..];
        names.push(name.to_string());
        rgbs.push(Col::new(code));
    }
    let idx_closest = closest(&Col::new(sha1), &rgbs);
    fill(sha1, &rgbs[idx_closest], &names[idx_closest], argc == 3);
}

fn fill(sha: &str, color: &Col, name: &str, oneline: bool) {
    let num = 7;
    let block = " ".repeat(num).on(color.rgb());
    let msg = "Your commit colour is";
    if oneline {
        println!("{} {} {} (#{})", block, msg, name, sha);
    } else {
        println!("\n{}", block);
        println!("{} {} {}", block, msg, name);
        println!("{}", block);
        println!("#{}\n", sha);
    }
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
