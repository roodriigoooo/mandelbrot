#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8, 
    pub g: u8, 
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn lerp(self, other: Color, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f64 + (other.r as f64 - self.r as f64) * t) as u8,
            g: (self.g as f64 + (other.g as f64 - self.g as f64) * t) as u8, 
            b: (self.b as f64 + (other.b as f64 - self.b as f64) * t) as u8,
        }
    }

    pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
        let h = h % 360.0;
        let s = s.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let c = v*s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v -c;

        let (r, g, b) = match h as i32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c), 
            _ => (c, 0.0, x),
        };

        Color {
            r: ((r+m) * 255.0) as u8, 
            g: ((g+m) * 255.0) as u8,
            b: ((b+m) * 255.0) as u8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gradient {
    stops: Vec<(f64, Color)> // (position, color), where position has to be between 0.0 and 1.0
}

impl Gradient {
    pub fn new(stops: Vec<(f64, Color)>) -> Self {
        let mut stops = stops;
        stops.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Self { stops }
    }

    pub fn get_color(&self, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);

        if self.stops.is_empty() {
            return Color::new(0,0,0);
        }

        if t <= self.stops[0].0 {
            return self.stops[0].1;
        }

        if t >= self.stops.last().unwrap().0 {
            return self.stops.last().unwrap().1;
        }

        // find two stops to interpolate between them
        for i in 0..self.stops.len() - 1 {
            let (pos1, color1) = self.stops[i];
            let (pos2, color2) = self.stops[i + 1];  

            if t >= pos1 && t <= pos2 {
                let local_t = (t - pos1) / (pos2 - pos1);
                return color1.lerp(color2, local_t);
            }
        }
        self.stops[0].1
    }
}

pub enum ColorScheme {
    Grayscale,
    Classic,
    Ocean,
    Fire,
    Psychedelic,
    Forest,
    Sunset,
    Custom(Gradient),
}

impl ColorScheme {
    pub fn get_color(&self, iterations: usize, max_iterations: usize) -> Color {
        if iterations >= max_iterations {
            // point is in the set - return black
            return Color::new(0, 0, 0);
        }

        // normalize to 0.0 - 1.0
        let t = iterations as f64 / max_iterations as f64;

        match self {
            ColorScheme::Grayscale => {
                let intensity = (t * 255.0) as u8;
                Color::new(intensity, intensity, intensity)
            }
            ColorScheme::Classic => {
                // blue to white classic
                let gradient = Gradient::new(vec![
                    (0.0, Color::new(0, 7, 100)),
                    (0.16, Color::new(32, 107, 203)),
                    (0.42, Color::new(237, 255, 255)),
                    (0.6425, Color::new(255, 170, 0)),
                    (0.8575, Color::new(0, 2, 0)),
                    (1.0, Color::new(0, 7, 100)),
                ]);
                gradient.get_color(t)
            }
            ColorScheme::Ocean => {
                let gradient = Gradient::new(vec![
                    (0.0, Color::new(0, 0, 128)),
                    (0.3, Color::new(0, 128, 255)),
                    (0.6, Color::new(64, 224, 208)),
                    (1.0, Color::new(240, 255, 255)),
                ]);
                gradient.get_color(t)
            }
            ColorScheme::Fire => {
                let gradient = Gradient::new(vec![
                    (0.0, Color::new(0, 0, 0)),
                    (0.25, Color::new(128, 0, 0)),
                    (0.5, Color::new(255, 0, 0)),
                    (0.75, Color::new(255, 165, 0)),
                    (1.0, Color::new(255, 255, 0)),
                ]);
                gradient.get_color(t)
            }
            ColorScheme::Psychedelic => {
                // hsv for smooth color cycling
                Color::from_hsv(t * 360.0 * 3.0, 1.0, 1.0)
            }
            ColorScheme::Forest => {
                let gradient = Gradient::new(vec![
                    (0.0, Color::new(0, 20, 0)),
                    (0.3, Color::new(34, 139, 34)),
                    (0.6, Color::new(144, 238, 144)),
                    (1.0, Color::new(240, 255, 240)),
                ]);
                gradient.get_color(t)
            }
            ColorScheme::Sunset => {
                let gradient = Gradient::new(vec![
                    (0.0, Color::new(25, 25, 112)),
                    (0.3, Color::new(255, 69, 0)),
                    (0.6, Color::new(255, 140, 0)),
                    (0.8, Color::new(255, 215, 0)),
                    (1.0, Color::new(255, 250, 205)),
                ]);
                gradient.get_color(t)
            }
            ColorScheme::Custom(gradient) => gradient.get_color(t),
        }
    }

    pub fn get_smooth_color(&self, iterations: usize, max_iterations: usize, z_norm: f64) -> Color {
        if iterations >= max_iterations {
            return Color::new(0, 0, 0);
        }

        // smooth coloring using logarithmic smoothing
        let smooth_iter = iterations as f64 + 1.0 - (z_norm.ln() / 2.0_f64.ln()).ln() / 2.0_f64.ln();
        let t = smooth_iter / max_iterations as f64;
        let t = t.clamp(0.0, 1.0);

        match self {
            ColorScheme::Grayscale => {
                let intensity = (t * 255.0) as u8;
                Color::new(intensity, intensity, intensity)
            }
            ColorScheme::Classic => {
                let gradient = Gradient::new(vec![
                    (0.0, Color::new(0, 7, 100)),
                    (0.16, Color::new(32, 107, 203)),
                    (0.42, Color::new(237, 255, 255)),
                    (0.6425, Color::new(255, 170, 0)),
                    (0.8575, Color::new(0, 2, 0)),
                    (1.0, Color::new(0, 7, 100)),
                ]);
                gradient.get_color(t)
            }
            ColorScheme::Ocean => {
                let gradient = Gradient::new(vec![
                    (0.0, Color::new(0, 0, 128)),
                    (0.3, Color::new(0, 128, 255)),
                    (0.6, Color::new(64, 224, 208)),
                    (1.0, Color::new(240, 255, 255)),
                ]);
                gradient.get_color(t)
            }
            ColorScheme::Fire => {
                let gradient = Gradient::new(vec![
                    (0.0, Color::new(0, 0, 0)),
                    (0.25, Color::new(128, 0, 0)),
                    (0.5, Color::new(255, 0, 0)),
                    (0.75, Color::new(255, 165, 0)),
                    (1.0, Color::new(255, 255, 0)),
                ]);
                gradient.get_color(t)
            }
            ColorScheme::Psychedelic => {
                Color::from_hsv(t * 360.0 * 3.0, 1.0, 1.0)
            }
            ColorScheme::Forest => {
                let gradient = Gradient::new(vec![
                    (0.0, Color::new(0, 20, 0)),
                    (0.3, Color::new(34, 139, 34)),
                    (0.6, Color::new(144, 238, 144)),
                    (1.0, Color::new(240, 255, 240)),
                ]);
                gradient.get_color(t)
            }
            ColorScheme::Sunset => {
                let gradient = Gradient::new(vec![
                    (0.0, Color::new(25, 25, 112)),
                    (0.3, Color::new(255, 69, 0)),
                    (0.6, Color::new(255, 140, 0)),
                    (0.8, Color::new(255, 215, 0)),
                    (1.0, Color::new(255, 250, 205)),
                ]);
                gradient.get_color(t)
            }
            ColorScheme::Custom(gradient) => gradient.get_color(t),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "grayscale" | "gray" => Some(ColorScheme::Grayscale),
            "classic" => Some(ColorScheme::Classic),
            "ocean" => Some(ColorScheme::Ocean),
            "fire" => Some(ColorScheme::Fire),
            "psychedelic" | "rainbow" => Some(ColorScheme::Psychedelic),
            "forest" => Some(ColorScheme::Forest),
            "sunset" => Some(ColorScheme::Sunset),
            _ => None,
        }
    }

    pub fn list_schemes() -> Vec<&'static str> {
        vec![
            "grayscale",
            "classic",
            "ocean",
            "fire",
            "psychedelic",
            "forest",
            "sunset",
        ]
    }
}