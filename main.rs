use std::fs::File;
use std::io;
use std::f32::consts::PI;
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
            };
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

fn diagonal_gradient(pixels: &mut [u32], width: usize, height: usize, foreground: u32, background: u32) {
    let color_start = foreground;
    let color_end = background;

    for y in 0..height {
        for x in 0..width {
            let t = (x + y) as f32 / (width + height) as f32;

            let r_start = ((color_start >> 8*2) & 0xFF) as f32;
            let g_start = ((color_start >> 8*1) & 0xFF) as f32;
            let b_start = ((color_start >> 8*0) & 0xFF) as f32;

            let r_end = ((color_end >> 8*2) & 0xFF) as f32;
            let g_end = ((color_end >> 8*1) & 0xFF) as f32;
            let b_end = ((color_end >> 8*0) & 0xFF) as f32;

            let r = (r_start + (r_end - r_start) * t).round() as u32;
            let g = (g_start + (g_end - g_start) * t).round() as u32;
            let b = (b_start + (b_end - b_start) * t).round() as u32;

            pixels[y * width + x] = (r << 16) | (g << 8) | b;
        }
    }
}

fn sine_wave_pattern(pixels: &mut [u32], width: usize, height: usize, wavelength: f32, amplitude: f32, foreground: u32, background: u32) {
    for y in 0..height {
        for x in 0..width {
            let wave = (y as f32 + amplitude * (2.0 * PI * x as f32 / wavelength).sin()) as usize;
            pixels[y * width + x] = if wave % 20 < 10 {
                foreground + y as u32
            } else {
                background
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

fn lerp(a: f32, b: f32, p: f32) -> f32 {
    a + (b - a) * p
}

fn blend_pixels_gamma_corrected(background: u32, foreground: u32, p: f32) -> u32 {
    let br = (background >> (8 * 2)) & 0xFF;
    let fr = (foreground >> (8 * 2)) & 0xFF;
    let r = lerp((br * br) as f32, (fr * fr) as f32, p).sqrt() as u32;

    let bg = (background >> (8 * 1)) & 0xFF;
    let fg = (foreground >> (8 * 1)) & 0xFF;
    let g = lerp((bg * bg) as f32, (fg * fg) as f32, p).sqrt() as u32;

    let bb = (background >> (8 * 0)) & 0xFF;
    let fb = (foreground >> (8 * 0)) & 0xFF;
    let b = lerp((bb * bb) as f32, (fb * fb) as f32, p).sqrt() as u32;

    (r << (8 * 2)) | (g << (8 * 1)) | (b << (8 * 0))
}

fn blend_pixels_naively(background: u32, foreground: u32, p: f32) -> u32 {
    let br = (background >> (8 * 2)) & 0xFF;
    let fr = (foreground >> (8 * 2)) & 0xFF;
    let r = lerp(br as f32, fr as f32, p) as u32;

    let bg = (background >> (8 * 1)) & 0xFF;
    let fg = (foreground >> (8 * 1)) & 0xFF;
    let g = lerp(bg as f32, fg as f32, p) as u32;

    let bb = (background >> (8 * 0)) & 0xFF;
    let fb = (foreground >> (8 * 0)) & 0xFF;
    let b = lerp(bb as f32, fb as f32, p) as u32;

    (r << (8 * 2)) | (g << (8 * 1)) | (b << (8 * 0))
}

fn fill_solid_aa_circle<Blender>(pixels: &mut [u32], width: usize, height: usize, radius: usize, foreground: u32, background: u32, blend_pixels: Blender) where Blender: Fn(u32, u32, f32) -> u32 {
    let cx = width as f32 * 0.5;
    let cy = height as f32 * 0.5;
    let r = radius as f32;

    const AA_RES: usize = 3;
    const AA_STEP: f32 = 1.0 / (AA_RES + 1) as f32;

    for y in 0..height {
        for x in 0..width {
            let mut aa_count = 0;
            for aay in 0..AA_RES {
                for aax in 0..AA_RES {
                    let px = x as f32 + AA_STEP + aax as f32 * AA_STEP;
                    let py = y as f32 + AA_STEP + aay as f32 * AA_STEP;
                    let dx = cx - px;
                    let dy = cy - py;
                    if dx*dx + dy*dy <= r*r {
                        aa_count += 1;
                    }
                }
            }
            let p = aa_count as f32 / (AA_RES * AA_RES) as f32;

            pixels[y*width + x] = blend_pixels(background, foreground, p);
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
    const WIDTH: usize = 256*5;
    const HEIGHT: usize = 256*5;
    const RADIUS: usize = WIDTH / 3;

    const WAVELENGTH: f32 = WIDTH as f32 / 5.0;
    const AMPLITUTE: f32 = HEIGHT as f32 / 40.0;

    const FOREGROUND: u32 = 0xFF00FF;
    const BACKGROUND: u32 = 0x181818;
    let mut pixels = [0u32; WIDTH*HEIGHT];

    pixels.fill(0xFF0000);
    stripe_pattern(&mut pixels, WIDTH, HEIGHT, WIDTH / 16, FOREGROUND, BACKGROUND);
    save_as_ppm("stride_pattern.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    checker_pattern(&mut pixels, WIDTH, HEIGHT, WIDTH / 16, FOREGROUND, BACKGROUND);
    save_as_ppm("checker_pattern.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    diagonal_gradient(&mut pixels, WIDTH, HEIGHT, FOREGROUND, BACKGROUND);
    save_as_ppm("diagonal-gradient.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    fill_solid_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND);
    save_as_ppm("solid_circle.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    sine_wave_pattern(&mut pixels, WIDTH, HEIGHT, WAVELENGTH, AMPLITUTE, FOREGROUND, BACKGROUND);
    save_as_ppm("sine-wave-pattern.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    fill_solid_aa_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND, blend_pixels_naively);
    save_as_ppm("solid-aa-naively.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    fill_solid_aa_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND, blend_pixels_gamma_corrected);
    save_as_ppm("solid-aa-gamma-corrected.ppm", &pixels, WIDTH, HEIGHT).unwrap();
    draw_hollow_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND);
    save_as_ppm("hollow_circle.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    draw_circle_with_mid_point_algorithm(&mut pixels, WIDTH, HEIGHT, WIDTH/2, FOREGROUND, BACKGROUND);
    let _ = save_as_ppm("mid_point_circle_algo.ppm", &pixels, WIDTH, HEIGHT).unwrap();
}
