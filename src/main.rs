use std::{
    io::{BufWriter, Write as _, stdout},
    time::Instant,
};

use crate::color::{Color, write_color};

pub mod color;
pub mod vec3;

// Image
const IMAGE_WIDTH: usize = 4096;
const IMAGE_HEIGHT: usize = 4096;

fn main() {
    let start = Instant::now();

    // Render
    {
        let mut out = BufWriter::new(stdout().lock());
        writeln!(out, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").unwrap();
        for y in 0..IMAGE_HEIGHT {
            eprint!("{:02}%\r", y * 100 / IMAGE_HEIGHT);
            for x in 0..IMAGE_WIDTH {
                let pixel_color = Color::new([
                    x as f64 / (IMAGE_WIDTH - 1) as f64,
                    y as f64 / (IMAGE_HEIGHT - 1) as f64,
                    0.0,
                ]);
                write_color(&mut out, &pixel_color);
            }
        }
    }

    eprintln!("{:?}", start.elapsed());
}
