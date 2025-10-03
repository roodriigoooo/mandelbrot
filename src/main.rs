use num::complex::Complex;

fn calculate_mandelbrot(
    max_iters: usize, x_min: f64, 
    x_max: f64, y_min: f64, 
    y_max: f64, width: usize, 
    height: usize) -> Vec<Vec<usize>> {

    let mut rows: Vec<_> = Vec::with_capacity(width);
    // Vec::with_capacity(width) creates a container to house the data from each row
    // for the desired result then, we need to create a container to house
    // the data for each column (Vec::with_capacity(height))
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = (img_x as f64 / width as f64);
            let y_percent = (img_y as f64 / height as f64);
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            // the above is called at every pixel (every row and column printed to stdout)
            row.push(escaped_at);
        }

    // the above calculates the proportio of the space covered in our output.
    // and then converts that to points within the search space.

        rows.push(row)
    }
    rows
}
 
// 
fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0};
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters // i no longer in scope
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ', 
                2..=5 => '.',
                5..=10 => '•',
                11..=30 => '*', 
                31..=100 => '+', 
                101..=200 => 'x', 
                201..=400 => '$', 
                401..=700 => '#', 
                _ => '%', 
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(10000, -2.0, 2.0, -1.0, 1.0, 200, 12);
    render_mandelbrot(mandelbrot);
}

