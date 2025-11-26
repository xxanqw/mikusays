use clap::ValueEnum;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Named(NamedColor),
    Hex(String),
    Rgb(u8, u8, u8),
    Hsl(u16, u8, u8),      // h: 0-360, s: 0-100, l: 0-100
    Ansi(u8),              // ANSI color code (0-255)
    Truecolor(u8, u8, u8), // 24-bit RGB
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

#[derive(Debug)]
pub enum ColorParseError {
    InvalidFormat,
    InvalidHex,
    InvalidRgb,
    InvalidHsl,
    UnknownNamedColor,
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorParseError::InvalidFormat => write!(f, "Invalid color format"),
            ColorParseError::InvalidHex => write!(f, "Invalid hex color code"),
            ColorParseError::InvalidRgb => write!(f, "Invalid RGB format. Expected format: R,G,B"),
            ColorParseError::InvalidHsl => {
                write!(f, "Invalid HSL format. Expected format: hsl(H,S%,L%)")
            }
            ColorParseError::UnknownNamedColor => write!(f, "Unknown named color"),
        }
    }
}

impl Error for ColorParseError {}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_lowercase();

        // Try named color first
        if let Ok(named) = NamedColor::from_str(s, true) {
            return Ok(Color::Named(named));
        }

        // Try hex color (#RRGGBB or #RGB)
        if s.starts_with('#') {
            return parse_hex_color(s);
        }

        // Try HSL (hsl(180,100%,50%))
        if s_lower.starts_with("hsl(") && s.ends_with(')') {
            return parse_hsl_color(s);
        }

        // Try RGB tuple (255,0,128)
        if s.contains(',') {
            return parse_rgb_color(s);
        }

        // Try ANSI code (number between 0-255)
        if let Ok(code) = s.parse::<u8>() {
            return Ok(Color::Ansi(code));
        }

        Err(ColorParseError::InvalidFormat)
    }
}

impl FromStr for NamedColor {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s, false)
    }
}

impl NamedColor {
    fn from_str(s: &str, ignore_case: bool) -> Result<Self, ColorParseError> {
        let s = if ignore_case {
            s.to_lowercase()
        } else {
            s.to_string()
        };

        match s.as_str() {
            "black" => Ok(NamedColor::Black),
            "red" => Ok(NamedColor::Red),
            "green" => Ok(NamedColor::Green),
            "yellow" => Ok(NamedColor::Yellow),
            "blue" => Ok(NamedColor::Blue),
            "magenta" => Ok(NamedColor::Magenta),
            "cyan" => Ok(NamedColor::Cyan),
            "white" => Ok(NamedColor::White),
            "brightblack" | "bright_black" => Ok(NamedColor::BrightBlack),
            "brightred" | "bright_red" => Ok(NamedColor::BrightRed),
            "brightgreen" | "bright_green" => Ok(NamedColor::BrightGreen),
            "brightyellow" | "bright_yellow" => Ok(NamedColor::BrightYellow),
            "brightblue" | "bright_blue" => Ok(NamedColor::BrightBlue),
            "brightmagenta" | "bright_magenta" => Ok(NamedColor::BrightMagenta),
            "brightcyan" | "bright_cyan" => Ok(NamedColor::BrightCyan),
            "brightwhite" | "bright_white" => Ok(NamedColor::BrightWhite),
            _ => Err(ColorParseError::UnknownNamedColor),
        }
    }
}

fn parse_hex_color(s: &str) -> Result<Color, ColorParseError> {
    let hex = s.trim_start_matches('#');

    match hex.len() {
        3 => {
            // #RGB format
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                .map_err(|_| ColorParseError::InvalidHex)?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                .map_err(|_| ColorParseError::InvalidHex)?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                .map_err(|_| ColorParseError::InvalidHex)?;
            Ok(Color::Hex(format!("#{:02x}{:02x}{:02x}", r, g, b)))
        }
        6 => {
            // #RRGGBB format
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorParseError::InvalidHex)?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorParseError::InvalidHex)?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorParseError::InvalidHex)?;
            Ok(Color::Hex(format!("#{:02x}{:02x}{:02x}", r, g, b)))
        }
        _ => Err(ColorParseError::InvalidHex),
    }
}

fn parse_rgb_color(s: &str) -> Result<Color, ColorParseError> {
    let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
    if parts.len() != 3 {
        return Err(ColorParseError::InvalidRgb);
    }

    let r = parts[0]
        .parse::<u8>()
        .map_err(|_| ColorParseError::InvalidRgb)?;
    let g = parts[1]
        .parse::<u8>()
        .map_err(|_| ColorParseError::InvalidRgb)?;
    let b = parts[2]
        .parse::<u8>()
        .map_err(|_| ColorParseError::InvalidRgb)?;

    Ok(Color::Rgb(r, g, b))
}

fn parse_hsl_color(s: &str) -> Result<Color, ColorParseError> {
    let content = s.trim_start_matches("hsl(").trim_end_matches(')');
    let parts: Vec<&str> = content.split(',').map(|p| p.trim()).collect();

    if parts.len() != 3 {
        return Err(ColorParseError::InvalidHsl);
    }

    let h = parts[0]
        .trim_end_matches(|c: char| !c.is_ascii_digit())
        .parse::<u16>()
        .map_err(|_| ColorParseError::InvalidHsl)?;
    let s = parts[1]
        .trim_end_matches('%')
        .parse::<u8>()
        .map_err(|_| ColorParseError::InvalidHsl)?;
    let l = parts[2]
        .trim_end_matches('%')
        .parse::<u8>()
        .map_err(|_| ColorParseError::InvalidHsl)?;

    if h > 360 || s > 100 || l > 100 {
        return Err(ColorParseError::InvalidHsl);
    }

    Ok(Color::Hsl(h, s, l))
}

impl Color {
    /// Convert color to RGB tuple
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Named(named) => named.to_rgb(),
            Color::Hex(hex) => {
                let hex = hex.trim_start_matches('#');
                match hex.len() {
                    3 => {
                        // #RGB format
                        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(0);
                        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(0);
                        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(0);
                        (r, g, b)
                    }
                    6 => {
                        // #RRGGBB format
                        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                        (r, g, b)
                    }
                    _ => (0, 0, 0),
                }
            }
            Color::Rgb(r, g, b) => (*r, *g, *b),
            Color::Hsl(h, s, l) => hsl_to_rgb(*h, *s, *l),
            Color::Ansi(code) => ansi_to_rgb(*code),
            Color::Truecolor(r, g, b) => (*r, *g, *b),
        }
    }

    /// Convert color to ANSI escape code for foreground
    pub fn to_ansi_fg(&self, color_mode: ColorMode) -> String {
        match color_mode {
            ColorMode::Truecolor => self.to_truecolor_fg(),
            ColorMode::Ansi256 => self.to_ansi256_fg(),
            ColorMode::Ansi16 => self.to_ansi16_fg(),
            ColorMode::NoColor => "".to_string(),
        }
    }

    fn to_truecolor_fg(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }

    fn to_ansi256_fg(&self) -> String {
        let (r, g, b) = self.to_rgb();
        let code = rgb_to_ansi256(r, g, b);
        format!("\x1b[38;5;{}m", code)
    }

    fn to_ansi16_fg(&self) -> String {
        let (r, g, b) = self.to_rgb();
        let code = rgb_to_ansi16(r, g, b);
        format!("\x1b[{}m", code)
    }
}

impl NamedColor {
    pub fn to_rgb(self) -> (u8, u8, u8) {
        match self {
            NamedColor::Black => (0, 0, 0),
            NamedColor::Red => (197, 15, 31),
            NamedColor::Green => (19, 161, 14),
            NamedColor::Yellow => (193, 156, 0),
            NamedColor::Blue => (0, 55, 218),
            NamedColor::Magenta => (136, 23, 152),
            NamedColor::Cyan => (58, 150, 221),
            NamedColor::White => (204, 204, 204),
            NamedColor::BrightBlack => (118, 118, 118),
            NamedColor::BrightRed => (231, 72, 86),
            NamedColor::BrightGreen => (22, 198, 12),
            NamedColor::BrightYellow => (249, 241, 165),
            NamedColor::BrightBlue => (59, 120, 255),
            NamedColor::BrightMagenta => (180, 0, 158),
            NamedColor::BrightCyan => (97, 214, 214),
            NamedColor::BrightWhite => (242, 242, 242),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorMode {
    Truecolor,
    Ansi256,
    Ansi16,
    NoColor,
}

/// Convert HSL to RGB
fn hsl_to_rgb(h: u16, s: u8, l: u8) -> (u8, u8, u8) {
    let h = h as f32 / 360.0;
    let s = s as f32 / 100.0;
    let l = l as f32 / 100.0;

    let r;
    let g;
    let b;

    if s == 0.0 {
        r = l;
        g = l;
        b = l;
    } else {
        let hue_to_rgb = |p: f32, q: f32, t: f32| -> f32 {
            let mut t = t;
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                return p + (q - p) * 6.0 * t;
            }
            if t < 1.0 / 2.0 {
                return q;
            }
            if t < 2.0 / 3.0 {
                return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
            }
            p
        };

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    }

    (
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    )
}

/// Convert RGB to ANSI 256 color code
fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    // Calculate grayscale value
    let gray = (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114).round() as u8;

    // Check if it's a grayscale color
    if r == g && g == b {
        // Grayscale ramp (232-255)
        if gray <= 8 {
            16
        } else if gray >= 248 {
            231
        } else {
            ((gray as f32 - 8.0) / 247.0 * 24.0).round() as u8 + 232
        }
    } else {
        // Color cube (16-231)
        let r = (r as f32 / 255.0 * 5.0).round() as u8;
        let g = (g as f32 / 255.0 * 5.0).round() as u8;
        let b = (b as f32 / 255.0 * 5.0).round() as u8;

        16 + (r * 36) + (g * 6) + b
    }
}

/// Convert RGB to ANSI 16 color code
fn rgb_to_ansi16(r: u8, g: u8, b: u8) -> u8 {
    // Calculate brightness
    let brightness = (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114).round() as u8;

    // Determine if it's bright or dark
    let is_bright = brightness > 128;

    // Determine base color
    let base_color = if r > g && r > b {
        // Red dominant
        if r > 192 && g < 64 && b < 64 {
            1 // Red
        } else if r > g && r > b {
            3 // Yellow
        } else {
            7 // White
        }
    } else if g > r && g > b {
        // Green dominant
        if g > 192 && r < 64 && b < 64 {
            2 // Green
        } else if g > b {
            3 // Yellow
        } else {
            6 // Cyan
        }
    } else {
        // Blue dominant
        if b > 192 && r < 64 && g < 64 {
            4 // Blue
        } else if b > r {
            6 // Cyan
        } else {
            5 // Magenta
        }
    };

    if is_bright {
        base_color + 8 // Bright colors are 8-15
    } else {
        base_color // Normal colors are 0-7
    }
}

/// Convert ANSI color code to RGB
fn ansi_to_rgb(code: u8) -> (u8, u8, u8) {
    match code {
        // Standard 16 colors
        0 => (0, 0, 0),        // Black
        1 => (128, 0, 0),      // Red
        2 => (0, 128, 0),      // Green
        3 => (128, 128, 0),    // Yellow
        4 => (0, 0, 128),      // Blue
        5 => (128, 0, 128),    // Magenta
        6 => (0, 128, 128),    // Cyan
        7 => (192, 192, 192),  // White
        8 => (128, 128, 128),  // Bright Black
        9 => (255, 0, 0),      // Bright Red
        10 => (0, 255, 0),     // Bright Green
        11 => (255, 255, 0),   // Bright Yellow
        12 => (0, 0, 255),     // Bright Blue
        13 => (255, 0, 255),   // Bright Magenta
        14 => (0, 255, 255),   // Bright Cyan
        15 => (255, 255, 255), // Bright White

        // 256-color mode
        16..=231 => {
            // Color cube
            let code = code - 16;
            let r = ((code / 36) % 6) * 51;
            let g = ((code / 6) % 6) * 51;
            let b = (code % 6) * 51;
            (r, g, b)
        }

        232..=255 => {
            // Grayscale ramp
            let gray = (code - 232) * 10 + 8;
            (gray, gray, gray)
        }
    }
}

/// Detect the best available color mode for the terminal
pub fn detect_color_mode() -> ColorMode {
    if std::env::var("NO_COLOR").is_ok() {
        return ColorMode::NoColor;
    }

    if let Ok(colorterm) = std::env::var("COLORTERM")
        && (colorterm.contains("truecolor") || colorterm.contains("24bit"))
    {
        return ColorMode::Truecolor;
    }

    if let Ok(term) = std::env::var("TERM") {
        if term.contains("256color") {
            return ColorMode::Ansi256;
        }
        if term.contains("color") {
            return ColorMode::Ansi16;
        }
    }

    // Default fallback
    ColorMode::Ansi16
}

/// Linear interpolation between two colors in HSV space
pub fn lerp_color_hsv(start: &Color, end: &Color, t: f32) -> Color {
    let (h1, s1, v1) = rgb_to_hsv(start.to_rgb());
    let (h2, s2, v2) = rgb_to_hsv(end.to_rgb());

    // Handle hue interpolation correctly (shortest path around the color wheel)
    let h_diff = (h2 - h1 + 180.0) % 360.0 - 180.0;
    let h = (h1 + h_diff * t + 360.0) % 360.0;

    let s = s1 + (s2 - s1) * t;
    let v = v1 + (v2 - v1) * t;

    let (r, g, b) = hsv_to_rgb(h, s, v);
    Color::Truecolor(r, g, b)
}

/// Convert RGB to HSV
fn rgb_to_hsv((r, g, b): (u8, u8, u8)) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };

    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    (h, s, v)
}

/// Convert HSV to RGB
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h {
        h if (0.0..60.0).contains(&h) => (c, x, 0.0),
        h if (60.0..120.0).contains(&h) => (x, c, 0.0),
        h if (120.0..180.0).contains(&h) => (0.0, c, x),
        h if (180.0..240.0).contains(&h) => (0.0, x, c),
        h if (240.0..300.0).contains(&h) => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}
