mod colorscheme;
mod renderer;

use num::complex::Complex;
use colorscheme::{Color, ColorScheme, Gradient};
use renderer::{OutputFormat, RenderData, Renderer};

struct MandelbrotResult {
    iterations: usize, 
    z_norm: f64,
}

fn calculate_mandelbrot(
    max_iters: usize, 
    x_min: f64, 
    x_max: f64, 
    y_min: f64, 
    y_max: f64, 
    width: usize, 
    height: usize
) -> (Vec<Vec<usize>>, Vec<Vec<f64>>) {
    let mut iterations: Vec<Vec<usize>> = Vec::with_capacity(height);
    let mut z_norms: Vec<Vec<f64>> = Vec::with_capacity(height);

    for img_y in 0..height {
        let mut iter_row: Vec<usize> = Vec::with_capacity(width);
        let mut norm_row: Vec<f64> = Vec::with_capacity(width);

        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;

            let result = mandelbrot_at_point(cx, cy, max_iters);
            iter_row.push(result.iterations);
            norm_row.push(result.z_norm);
        }

        iterations.push(iter_row);
        z_norms.push(norm_row);
    }

    (iterations, z_norms)
}

// 
fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> MandelbrotResult {
    let mut z = Complex { re: 0.0, im: 0.0};
    let c = Complex::new(cx, cy);

    for i in 0..max_iters {
        if z.norm() > 2.0 {
            return MandelbrotResult {
                iterations: i, 
                z_norm: z.norm(),
            };
        }
        z = z * z + c;
    }
    MandelbrotResult {
        iterations: max_iters, 
        z_norm: z.norm(),
    }
}


fn main() {
    let max_iterations = 1000;
    let width = 150;
    let height = 50;

    let (x_min, x_max, y_min, y_max) = (-2.0, 1.0, -1.0, 1.0);

    println!("Calculating Mandelbrot set...");
    let (iterations, z_norms) = calculate_mandelbrot(
        max_iterations, 
        x_min, 
        x_max, 
        y_min, 
        y_max, 
        width,
        height,
    );

    let render_data = RenderData::new(iterations, z_norms, max_iterations);

    // classic color scheme with auto-detected terminal format
    println!("\n=== Classic Color Scheme ===");
    let format = renderer::detect_terminal_capabilities();
    let renderer = Renderer::new(ColorScheme::Classic, format);
    renderer.render_to_terminal(&render_data);

    //creating custom gradients
    println!("\n=== custom cyberpunk ===");
    let cyberpunk = Gradient::new(vec![
        (0.0, Color::new(16, 16, 16)),
        (0.3, Color::new(112, 66, 20)),
        (0.6, Color::new(196, 164, 132)),
        (1.0, Color::new(255, 240, 220)),
    ]);
    let scheme = ColorScheme::Custom(cyberpunk);
    let renderer = Renderer::new(
        scheme, 
        OutputFormat::AnsiTrueColor
    );
    renderer.render_to_terminal(&render_data);
    // save as ppm also example
    renderer.save_as_ppm(&render_data, "custom_mandelbrot_cyberpunk.ppm")
        .expect("Failed to save image");
}




