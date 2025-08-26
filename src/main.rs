use std::time::Instant;

// Image
const IMAGE_WIDTH: usize = 1024;
const IMAGE_HEIGHT: usize = 1024;

fn main() {
    let start = Instant::now();

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let red = (x as f64 / (IMAGE_WIDTH - 1) as f64 * 255.999) as u8;
            let green = (y as f64 / (IMAGE_HEIGHT - 1) as f64 * 255.999) as u8;
            let blue = 0;
            println!("{red} {green} {blue}");
        }
    }

    eprintln!("{:?}", start.elapsed());
}
