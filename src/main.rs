use termion::color;
use termion::color::Color;

fn main() {
    let size = 8;
    fill(size, color::Red);
    fill(size, color::Blue);
    fill(size, color::Green);
}

fn fill<T: Color>(num: usize, col: T) {
    let fill = " ".repeat(num);
    println!("{}{}{}", color::Bg(col), fill, color::Bg(color::Reset));
}
