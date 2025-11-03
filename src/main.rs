// ==========================================================================
//
//  Copyright (c) 2024, 2025 Marco Colombo
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
    fn rgb(&self) -> Color {
        Color::Rgb {
            r: self.r as u8,
            g: self.g as u8,
            b: self.b as u8,
        }
    }
}

fn parse_sha(sha: &str) -> Option<(i32, i32, i32)> {
    let r = i32::from_str_radix(sha.get(0..2)?, 16).ok()?;
    let g = i32::from_str_radix(sha.get(2..4)?, 16).ok()?;
    let b = i32::from_str_radix(sha.get(4..6)?, 16).ok()?;
    Some((r, g, b))
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
    let sha1 = &args[argc - 1];
    let (rgbs, names): (Vec<_>, Vec<_>) = include_str!("../all-colors.csv")
        .lines()
        .filter_map(|line| {
            let (code, name) = line.split_at(7);
            parse_sha(code).map(|(r, g, b)| (Col { r, g, b }, name.to_string()))
        })
        .unzip();
    if let Some((r, g, b)) = parse_sha(sha1) {
        let idx_closest = closest(&Col { r, g, b }, &rgbs);
        fill(sha1, &rgbs[idx_closest], &names[idx_closest], argc == 3);
    } else {
        println!("{sha1} is not a valid sha1")
    }
}

fn fill(sha: &str, color: &Col, name: &str, oneline: bool) {
    let num = 7;
    let block = " ".repeat(num).on(color.rgb());
    let msg = "Your commit colour is";
    if oneline {
        println!("{} {} {} (#{})", block, msg, name, sha);
    } else {
        println!(
            "\n{}\n{} {} {}\n{}\n#{}\n",
            block, block, msg, name, block, sha
        )
    };
}

fn closest(rgb: &Col, rgbs: &[Col]) -> usize {
    rgbs.iter()
        .enumerate()
        .map(|(idx, item)| (idx, distance(rgb, item)))
        .min_by(|(_, dist_a), (_, dist_b)| dist_a.partial_cmp(dist_b).unwrap())
        .map(|(idx, _)| idx)
        .unwrap()
}

fn distance(c1: &Col, c2: &Col) -> f32 {
    f32::sqrt(((c1.r - c2.r).pow(2) + (c1.g - c2.g).pow(2) + (c1.b - c2.b).pow(2)) as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parse_sha1() {
        assert_eq!(parse_sha(&"123123123123"), Some((18, 49, 35)));
        assert_eq!(parse_sha(&"123123"), Some((18, 49, 35)));
        assert_eq!(parse_sha(&"000000"), Some((0, 0, 0)));
    }

    #[test]
    fn test_invalid_parse_sha1() {
        assert_eq!(parse_sha(&"123"), None);
        assert_eq!(parse_sha(&"123ijk"), None);
    }

    #[test]
    fn test_distance() {
        let col1 = Col { r: 0, g: 0, b: 0 };
        let col2 = Col {
            r: 255,
            g: 255,
            b: 255,
        };
        assert_eq!(distance(&col1, &col1), 0f32);
        assert_eq!(distance(&col1, &col2), 441.67294f32);
    }
}
