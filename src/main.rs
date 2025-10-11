mod colorscheme;
mod renderer;

use clap::Parser;
use num::complex::Complex;
use colorscheme::{Color, ColorScheme, Gradient};
use renderer::{OutputFormat, RenderData, Renderer};

#[derive(Parser)]
#[command(name = "Mandelbrot Renderer")]
#[command(author = "rodrigo s")]
#[command(version = "0.1")]
struct Args {
    /// width in pixels
    #[arg(short, long, default_value_t = 150)]
    width: usize,

    /// height in pixels
    #[arg(short = 'H', long, default_value_t = 50)]
    height: usize,

    /// max iterations
    #[arg(short, long, default_value_t = 1000)]
    iterations: usize,

    /// color scheme: classic, ocean, fire, psychedelic, forest, sunset, grayscale
    #[arg(short, long, default_value = "classic")]
    color: String,

    /// output format: auto, ascii, extended, ansi256, truecolor
    #[arg(short, long, default_value = "auto")]
    format: String,

    /// save to PPM file
    #[arg(short, long)]
    save: Option<String>,

    /// coordinate preset: default, seahorse, spiral, elephant, julia
    #[arg(short, long)]
    preset: Option<String>,

    /// min X coordinate
    #[arg(long)]
    xmin: Option<f64>,

    /// max X coordinate
    #[arg(long)]
    xmax: Option<f64>,

    /// min Y coordinate
    #[arg(long)]
    ymin: Option<f64>,

    /// max Y coordinate
    #[arg(long)]
    ymax: Option<f64>,

    /// list available color schemes
    #[arg(long)]
    list_colors: bool,

    /// disable smooth coloring
    #[arg(long)]
    no_smooth: bool,
}
struct MandelbrotResult {
    iterations: usize, 
    z_norm: f64,
}

fn get_preset_coords(preset: &str) -> Option<(f64, f64, f64, f64)> {
    match preset.to_lowercase().as_str() {
        "default" => Some((-2.0, 1.0, -1.0, 1.0)),
        "seahorse" => Some((-0.75, -0.735, 0.095, 0.11)),
        "spiral" => Some((-0.7269, -0.7266, 0.1889, 0.1892)),
        "elephant" => Some((0.275, 0.285, 0.005, 0.015)),
        "julia" => Some((-1.5, 1.5, -1.5, 1.5)),
        _ => None,
    }
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
    let args = Args::parse();
    if args.list_colors {
        println!("Available color schemes:");
        for scheme in ColorScheme::list_schemes() {
            println!(" • {}", scheme);
        }
        return;
    }

    let (x_min, x_max, y_min, y_max) = if let Some(preset) = &args.preset {
        get_preset_coords(preset)
            .unwrap_or_else(|| {
                eprintln!("Warning: Unknown preset: '{}', using default", preset);
                (-2.0, 1.0, -1.0, 1.0)
            })
    } else {
        (
            args.xmin.unwrap_or(-2.0), 
            args.xmax.unwrap_or(1.0),
            args.ymin.unwrap_or(-1.0),
            args.ymax.unwrap_or(1.0),
        )
    };

    let color_scheme = ColorScheme::from_str(&args.color)
        .unwrap_or_else(|| {
            eprintln!("Warning: Unknown color scheme '{}', using 'classic'", args.color);
            ColorScheme::Classic
        });

    let output_format = match args.format.to_lowercase().as_str() {
        "auto" => renderer::detect_terminal_capabilities(),
        "ascii" => OutputFormat::Ascii,
        "extended" => OutputFormat::AsciiExtended,
        "ansi256" => OutputFormat::Ansi256,
        "truecolor" => OutputFormat::AnsiTrueColor,
        _ => {
            eprintln!("Warning: Unknown format '{}', using auto-detect", args.format);
            renderer::detect_terminal_capabilities()
        }
    };

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Resolution: {}×{}", args.width, args.height);
    println!("Iterations: {}", args.iterations);
    println!("Region: x=[{:.4}, {:.4}], y=[{:.4}, {:.4}]", x_min, x_max, y_min, y_max);
    println!("Colors: {}", args.color);
    println!();

    println!("Calculating Mandelbrot set...");
    let (iterations, z_norms) = calculate_mandelbrot(
        args.iterations, 
        x_min, 
        x_max, 
        y_min, 
        y_max, 
        args.width,
        args.height,
    );

    let render_data = RenderData::new(iterations, z_norms, args.iterations);
    let renderer = Renderer::new(color_scheme, output_format)
        .with_smooth_coloring(!args.no_smooth);

    renderer.render_to_terminal(&render_data);
    if let Some(filename) = args.save {
        println!("\n saving to {}...", filename);
        match renderer.save_as_ppm(&render_data, &filename) {
            Ok(_) => println!("saved successfully"),
            Err(e) => eprintln!("error: {}", e),
        }
    }
}

/*
fn custom_schemes_ex() {
    let cyberpunk = Gradient::new(vec![
        (0.0, Color::new(0, 0, 0)),           // Black
        (0.2, Color::new(255, 0, 255)),       // Magenta
        (0.4, Color::new(0, 255, 255)),       // Cyan
        (0.7, Color::new(255, 0, 128)),       // Hot pink
        (1.0, Color::new(255, 255, 255)),     // White
    ]);
    let scheme = ColorScheme::Custom(cyberpunk);
}
*/


