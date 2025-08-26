use std::{
    io::{BufWriter, Write as _, stdout},
    time::Instant,
};

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
            for x in 0..IMAGE_WIDTH {
                let red = (x as f64 / (IMAGE_WIDTH - 1) as f64 * 255.999) as u8;
                let green = (y as f64 / (IMAGE_HEIGHT - 1) as f64 * 255.999) as u8;
                let blue = 0;
                writeln!(out, "{red} {green} {blue}").unwrap();
            }
        }
    }

    eprintln!("{:?}", start.elapsed());
}
