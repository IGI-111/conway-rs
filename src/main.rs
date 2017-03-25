extern crate rand;
extern crate termion;
extern crate drawille;
extern crate rayon;
mod grid;

use grid::Grid;
use std::io::{stdout, Write, Read};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{terminal_size, cursor, clear, async_stdin};
use drawille::Canvas;

pub fn main() {
    let mut stdin = async_stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(stdout, "{}{}", cursor::Hide, clear::All).unwrap();

    let (term_width, term_height) = terminal_size().unwrap();
    let width = (2 * term_width - 1) as usize;
    let height = (4 * term_height - 1) as usize;
    let mut grid = Grid::random(width, height);
    let mut canvas = Canvas::new(width as u32, height as u32);

    while no_key_pressed(&mut stdin) {
        grid.tick();
        display(&mut stdout, &grid, &mut canvas);
    }
    write!(stdout, "{}", cursor::Show).unwrap();
}

fn no_key_pressed(stdin: &mut Read) -> bool {
    let mut buf: [u8; 1] = [0];
    stdin.read(&mut buf).unwrap();
    buf[0] == 0
}

fn display(out: &mut Write, grid: &Grid, canvas: &mut Canvas) {
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if grid.is_alive(x, y) {
                canvas.set(x as u32, y as u32);
            } else {
                canvas.unset(x as u32, y as u32);
            }
        }
    }
    let mut line = 1;
    write!(out, "{}", cursor::Goto(1, line)).unwrap();
    for c in canvas.frame().chars() {
        if c == '\n' {
            line += 1;
            write!(out, "{}", cursor::Goto(1, line)).unwrap();
        } else {
            write!(out, "{}", c).unwrap();
        }
    }
}
