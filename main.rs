use std::fs::File;
use std::io;
use std::io::Write;

fn save_as_ppm(file_path: &str, pixels: &[u32], width: usize, height: usize) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    write!(file, "P6\n{} {} 255\n", width, height)?;
    for y in 0..height {
        for x in 0..width {
            let pixel = pixels[y * width + x];
            let color = [((pixel >> 8 * 2) & 0xFF) as u8, // Red
                         ((pixel >> 8 * 1) & 0xFF) as u8, // Green
                         ((pixel >> 8 * 0) & 0xFF) as u8  // Blue
                        ];
            file.write_all(&color)?;
        }
    }
    Ok(())
}

fn main() {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 64;
    const OUTPUT_PATH: &str = "output.ppm";
    let mut pixels = [0u32; WIDTH*HEIGHT];
    pixels.fill(0xFF0000);

    let _ = save_as_ppm(OUTPUT_PATH, &pixels, WIDTH, HEIGHT);
    println!("The file was saved at {}!", OUTPUT_PATH);
}
