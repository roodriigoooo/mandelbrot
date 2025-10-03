use crate::colorscheme::{Color, ColorScheme};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Ascii,
    AsciiExtended,
    Ansi256,
    AnsiTrueColor,
}

pub struct RenderData {
    pub iterations: Vec<Vec<usize>>,
    pub z_norms: Vec<Vec<f64>>, // For smooth coloring
    pub max_iterations: usize,
}

impl RenderData {
    pub fn new(iterations: Vec<Vec<usize>>, z_norms: Vec<Vec<f64>>, max_iterations: usize) -> Self {
        Self {
            iterations,
            z_norms,
            max_iterations,
        }
    }

    pub fn width(&self) -> usize {
        self.iterations.first().map(|row| row.len()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.iterations.len()
    }
}


pub struct Renderer {
    color_scheme: ColorScheme,
    output_format: OutputFormat,
    use_smooth_coloring: bool,
}

impl Renderer {
    pub fn new(color_scheme: ColorScheme, output_format: OutputFormat) -> Self {
        Self {
            color_scheme,
            output_format,
            use_smooth_coloring: true,
        }
    }

    pub fn with_smooth_coloring(mut self, smooth: bool) -> Self {
        self.use_smooth_coloring = smooth;
        self
    }

    pub fn render_to_terminal(&self, data: &RenderData) {
        match self.output_format {
            OutputFormat::Ascii => self.render_ascii(data),
            OutputFormat::AsciiExtended => self.render_ascii_extended(data),
            OutputFormat::Ansi256 => self.render_ansi_256(data),
            OutputFormat::AnsiTrueColor => self.render_ansi_truecolor(data),
        }
    }

    fn render_ascii(&self, data: &RenderData) {
        let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
        
        for y in 0..data.height() {
            let mut line = String::with_capacity(data.width());
            for x in 0..data.width() {
                let iters = data.iterations[y][x];
                
                if iters >= data.max_iterations {
                    line.push(' ');
                } else {
                    let idx = ((iters as f64 / data.max_iterations as f64) * (chars.len() - 1) as f64) as usize;
                    line.push(chars[idx.min(chars.len() - 1)]);
                }
            }
            println!("{}", line);
        }
    }

    fn render_ascii_extended(&self, data: &RenderData) {
        let chars = [
            ' ', '·', '∙', '•', '○', '◦', '⋅', '⋆', '∗', '⊕',
            '⊗', '⊛', '⊚', '◉', '●', '◐', '◑', '◒', '◓', '█'
        ];
        
        for y in 0..data.height() {
            let mut line = String::with_capacity(data.width() * 3); // Unicode chars
            for x in 0..data.width() {
                let iters = data.iterations[y][x];
                
                if iters >= data.max_iterations {
                    line.push(' ');
                } else {
                    let idx = ((iters as f64 / data.max_iterations as f64) * (chars.len() - 1) as f64) as usize;
                    line.push(chars[idx.min(chars.len() - 1)]);
                }
            }
            println!("{}", line);
        }
    }

    fn render_ansi_256(&self, data: &RenderData) {
        for y in 0..data.height() {
            for x in 0..data.width() {
                let iters = data.iterations[y][x];
                let z_norm = data.z_norms[y][x];
                
                let color = if self.use_smooth_coloring {
                    self.color_scheme.get_smooth_color(iters, data.max_iterations, z_norm)
                } else {
                    self.color_scheme.get_color(iters, data.max_iterations)
                };

                let ansi_color = rgb_to_ansi256(color.r, color.g, color.b);
                print!("\x1b[48;5;{}m \x1b[0m", ansi_color);
            }
            println!();
        }
    }

    fn render_ansi_truecolor(&self, data: &RenderData) {
        for y in 0..data.height() {
            for x in 0..data.width() {
                let iters = data.iterations[y][x];
                let z_norm = data.z_norms[y][x];
                
                let color = if self.use_smooth_coloring {
                    self.color_scheme.get_smooth_color(iters, data.max_iterations, z_norm)
                } else {
                    self.color_scheme.get_color(iters, data.max_iterations)
                };

                print!("\x1b[48;2;{};{};{}m \x1b[0m", color.r, color.g, color.b);
            }
            println!();
        }
    }

    pub fn save_as_ppm(&self, data: &RenderData, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(filename)?;
        
        // PPM header
        writeln!(file, "P6")?;
        writeln!(file, "{} {}", data.width(), data.height())?;
        writeln!(file, "255")?;

        // Pixel data
        for y in 0..data.height() {
            for x in 0..data.width() {
                let iters = data.iterations[y][x];
                let z_norm = data.z_norms[y][x];
                
                let color = if self.use_smooth_coloring {
                    self.color_scheme.get_smooth_color(iters, data.max_iterations, z_norm)
                } else {
                    self.color_scheme.get_color(iters, data.max_iterations)
                };

                file.write_all(&[color.r, color.g, color.b])?;
            }
        }

        Ok(())
    }
}

fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    // Use the 216-color cube (16-231)
    // Each component is divided into 6 levels (0-5)
    let r = (r as u16 * 5 / 255) as u8;
    let g = (g as u16 * 5 / 255) as u8;
    let b = (b as u16 * 5 / 255) as u8;
    
    16 + 36 * r + 6 * g + b
}

// helper
pub fn detect_terminal_capabilities() -> OutputFormat {
    // Check COLORTERM environment variable for truecolor support
    if let Ok(colorterm) = std::env::var("COLORTERM") {
        if colorterm == "truecolor" || colorterm == "24bit" {
            return OutputFormat::AnsiTrueColor;
        }
    }

    // Check TERM for 256 color support
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("256color") {
            return OutputFormat::Ansi256;
        }
    }

    // Default to ASCII
    OutputFormat::Ascii
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_ansi256() {
        assert_eq!(rgb_to_ansi256(0, 0, 0), 16);
        assert_eq!(rgb_to_ansi256(255, 255, 255), 231);
        assert_eq!(rgb_to_ansi256(255, 0, 0), 196);
    }
}