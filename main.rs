use std::fs::File;
use std::io;
use std::io::Write;

fn save_as_ppm(file_path: &str, pixels: &[u32], width: usize, height: usize) -> io::Result<()> {
    let mut buffer = Vec::<u8>::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = pixels[y * width + x];
            buffer.push(((pixel >> 8 * 2) & 0xFF) as u8);
            buffer.push(((pixel >> 8 * 1) & 0xFF) as u8);
            buffer.push(((pixel >> 8 * 0) & 0xFF) as u8);
        }
    }
    let mut file = File::create(file_path)?;
    write!(file, "P6\n{} {} 255\n", width, height)?;
    file.write(&buffer)?;
    println!("The file was saved at {}!", file_path);
    Ok(())
}

fn stripe_pattern(pixels: &mut [u32], width: usize, height: usize, tile_size: usize, foreground: u32, background: u32) {
    for y in 0..height {
        for x in 0..width {
            pixels[y * width + x] = if ((x + y) / tile_size) % 2 == 0 {
                background
            } else {
                foreground
            }
        }
    }
}

fn checker_pattern(pixels: &mut [u32], width: usize, height: usize, tile_size: usize, foreground: u32, background: u32) {
    for y in 0..height {
        for x in 0..width {
            pixels[y * width + x] = if (x / tile_size + y / tile_size) % 2 == 1 {
                background
            } else {
                foreground
            }
        }
    }
}

fn fill_solid_circle(pixels: &mut [u32], width: usize, height: usize, radius: usize, foreground: u32, background: u32) {
    let cx = width as f32 * 0.5;
    let cy = height as f32 * 0.5;
    let r = radius as f32;

    for y in 0..height {
        for x in 0..width {
            let dx = cx - x as f32 - 0.5;
            let dy = cy - y as f32 - 0.5;

            pixels[y * width + x] = if dx * dx + dy * dy <= r * r {
                foreground
            } else {
                background
            };
        }
    }
}


fn draw_hollow_circle(pixels: &mut [u32], width: usize, height: usize, radius: usize, foreground: u32, background: u32) {
    let cx = width as f32 * 0.5;
    let cy = height as f32 * 0.5;
    let r = radius as f32;

    let thickness = 1.0;
    let _r_squared = r * r;
    let r_inner = (r - thickness).max(0.0);
    let r_outer = r + thickness;
    let r_inner_squared = r_inner * r_inner;
    let r_outer_squared = r_outer * r_outer;

    for y in 0..height {
        for x in 0..width {
            let dx = cx - x as f32 - 0.5;
            let dy = cy - y as f32 - 0.5;
            let dist_squared = dx * dx + dy * dy;

            pixels[y * width + x] = if dist_squared >= r_inner_squared && dist_squared <= r_outer_squared {
                foreground
            } else {
                background
            };
        }
    }
}

fn draw_circle_with_mid_point_algorithm(pixels: &mut [u32], width: usize, height: usize, radius: usize, foreground: u32, background: u32) {
    pixels.fill(background);

    let w = width as i32;
    let h = height as i32;
    let r = radius as i32;
    let cx = w / 2;
    let cy = w / 2;
    let mut x = 0;
    let mut y = r;

    while x <= y {
        let px = x + cx;
        let py = y + cy;
        if (0..w).contains(&px) && (0..h).contains(&py) {
            assert!(height == width);
            let dx = px as usize;
            let dy = py as usize;
            // Right half circle
            pixels[dy * width + dx] = foreground;
            pixels[dx * width + dy] = foreground;
            pixels[(height - dy) * width + dx] = foreground;
            pixels[(width - dx) * width + dy] = foreground;

            // Left half circle
            pixels[dy * width - dx] = foreground;
            pixels[dx * width - dy] = foreground;
            pixels[(height - dy) * width - dx] = foreground;
            pixels[(width - dx) * width - dy] = foreground;

        }
        x += 1;
        if x*x + y*y > r*r {
            y -= 1;
        }
    }
}

fn main() {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;
    const FOREGROUND: u32 = 0xFF00FF;
    const BACKGROUND: u32 = 0x181818;
    let mut pixels = [0u32; WIDTH*HEIGHT];

    pixels.fill(0xFF0000);
    stripe_pattern(&mut pixels, WIDTH, HEIGHT, 32, FOREGROUND, BACKGROUND);
    let _ = save_as_ppm("stride_pattern.ppm", &pixels, WIDTH, HEIGHT);

    checker_pattern(&mut pixels, WIDTH, HEIGHT, 32, FOREGROUND, BACKGROUND);
    let _ = save_as_ppm("checker_pattern.ppm", &pixels, WIDTH, HEIGHT);

    fill_solid_circle(&mut pixels, WIDTH, HEIGHT, WIDTH/2, FOREGROUND, BACKGROUND);
    let _ = save_as_ppm("solid_circle.ppm", &pixels, WIDTH, HEIGHT);

    draw_hollow_circle(&mut pixels, WIDTH, HEIGHT, WIDTH/2, FOREGROUND, BACKGROUND);
    let _ = save_as_ppm("hollow_circle.ppm", &pixels, WIDTH, HEIGHT);

    draw_circle_with_mid_point_algorithm(&mut pixels, WIDTH, HEIGHT, WIDTH/2, FOREGROUND, BACKGROUND);
    let _ = save_as_ppm("mid_point_circle_algo.ppm", &pixels, WIDTH, HEIGHT);
}
