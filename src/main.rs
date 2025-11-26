use clap::{Parser, ValueEnum};
use crossterm::terminal::size;
use std::error::Error;
use std::str::FromStr;
use unicode_width::UnicodeWidthStr;

mod color;
mod mikuart;
use color::{Color, ColorMode, detect_color_mode, lerp_color_hsv};
use mikuart::get_miku_art;

#[derive(Debug, Clone, ValueEnum)]
enum ColorFormat {
    Hex,
    Rgb,
    Hsl,
    Ansi,
    Named,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Text to display in the speech bubble
    pub text: Option<String>,

    /// Style of the Miku art. A random one is chosen if not specified
    #[arg(short, long)]
    pub style: Option<i32>,

    /// List all available art styles with their indices
    #[arg(short, long)]
    pub list: bool,

    /// Apply a smooth rainbow gradient across the ASCII art
    #[arg(long)]
    pub rainbow: bool,

    /// Saturation level for rainbow gradient (0-100)
    #[arg(long, default_value_t = 100)]
    pub saturation: u8,

    /// Brightness level for rainbow gradient (0-100)
    #[arg(long, default_value_t = 50)]
    pub brightness: u8,

    /// Override with a single solid color
    #[arg(long, value_name = "COLOR")]
    pub color: Option<String>,

    /// Define a custom two-color gradient (e.g., --gradient red:blue)
    #[arg(long, value_name = "START:END")]
    pub gradient: Option<String>,

    /// Disable all coloring (respects NO_COLOR environment variable)
    #[arg(long)]
    pub no_color: bool,
}

#[derive(Debug)]
pub struct ColorConfig {
    pub mode: ColorMode,
    pub rainbow: bool,
    pub saturation: u8,
    pub brightness: u8,
    pub single_color: Option<Color>,
    pub gradient: Option<(Color, Color)>,
    pub color_cache: Vec<Color>,
}

pub fn parse_color_args(args: &Args) -> Result<ColorConfig, Box<dyn Error>> {
    // Detect color mode
    let mut color_mode = detect_color_mode();

    // Override with --no-color if specified
    if args.no_color {
        color_mode = ColorMode::NoColor;
    }

    // Parse single color if provided
    let single_color = if let Some(color_str) = args.color.as_deref() {
        match Color::from_str(color_str) {
            Ok(color) => Some(color),
            Err(e) => {
                eprintln!("Error parsing color '{}': {}", color_str, e);
                eprintln!("Valid color formats:");
                eprintln!("  - Named colors: red, blue, green, etc.");
                eprintln!("  - Hex: #RRGGBB or #RGB (e.g., #FF0000 or #F00)");
                eprintln!("  - RGB: R,G,B (e.g., 255,0,128)");
                eprintln!("  - HSL: hsl(H,S%,L%) (e.g., hsl(180,100%,50%))");
                eprintln!("  - ANSI: 0-255 (e.g., 93 for bright yellow)");
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    // Parse gradient if provided
    let gradient = if let Some(gradient_str) = args.gradient.as_deref() {
        let parts: Vec<&str> = gradient_str.split(':').collect();
        if parts.len() != 2 {
            eprintln!("Error: Gradient format should be START:END (e.g., --gradient red:blue)");
            std::process::exit(1);
        }

        let start = match Color::from_str(parts[0]) {
            Ok(color) => color,
            Err(e) => {
                eprintln!("Error parsing gradient start color '{}': {}", parts[0], e);
                eprintln!("Valid color formats:");
                eprintln!("  - Named colors: red, blue, green, etc.");
                eprintln!("  - Hex: #RRGGBB or #RGB (e.g., #FF0000 or #F00)");
                eprintln!("  - RGB: R,G,B (e.g., 255,0,128)");
                eprintln!("  - HSL: hsl(H,S%,L%) (e.g., hsl(180,100%,50%))");
                eprintln!("  - ANSI: 0-255 (e.g., 93 for bright yellow)");
                std::process::exit(1);
            }
        };

        let end = match Color::from_str(parts[1]) {
            Ok(color) => color,
            Err(e) => {
                eprintln!("Error parsing gradient end color '{}': {}", parts[1], e);
                eprintln!("Valid color formats:");
                eprintln!("  - Named colors: red, blue, green, etc.");
                eprintln!("  - Hex: #RRGGBB or #RGB (e.g., #FF0000 or #F00)");
                eprintln!("  - RGB: R,G,B (e.g., 255,0,128)");
                eprintln!("  - HSL: hsl(H,S%,L%) (e.g., hsl(180,100%,50%))");
                eprintln!("  - ANSI: 0-255 (e.g., 93 for bright yellow)");
                std::process::exit(1);
            }
        };

        Some((start, end))
    } else {
        None
    };

    // Validate saturation and brightness
    let saturation = args.saturation.clamp(0, 100);
    let brightness = args.brightness.clamp(0, 100);

    // Check for conflicting options
    if args.rainbow && single_color.is_some() {
        eprintln!("Error: Cannot use --rainbow and --color together");
        std::process::exit(1);
    }

    if args.rainbow && gradient.is_some() {
        eprintln!("Error: Cannot use --rainbow and --gradient together");
        std::process::exit(1);
    }

    if single_color.is_some() && gradient.is_some() {
        eprintln!("Error: Cannot use --color and --gradient together");
        std::process::exit(1);
    }

    Ok(ColorConfig {
        mode: color_mode,
        rainbow: args.rainbow,
        saturation,
        brightness,
        single_color,
        gradient,
        color_cache: Vec::new(),
    })
}

pub fn get_speech_bubble_lines(text: &str) -> Vec<String> {
    const MAX_BUBBLE_WIDTH: usize = 50;
    const MIN_BUBBLE_WIDTH: usize = 20;

    let temp_lines = wrap_text(text, MAX_BUBBLE_WIDTH - 4);

    let content_width = temp_lines
        .iter()
        .map(|line| line.width())
        .max()
        .unwrap_or(0);

    let bubble_width = (content_width + 4).clamp(MIN_BUBBLE_WIDTH, MAX_BUBBLE_WIDTH);

    let lines = wrap_text(text, bubble_width - 4);
    let mut bubble_lines = Vec::new();

    let top_left = '╭';
    let top_right = '╮';
    let bottom_left = '╰';
    let bottom_right = '╯';
    let horizontal = '─';
    let vertical = '│';

    bubble_lines.push(format!(
        "{}{}{}",
        top_left,
        horizontal.to_string().repeat(bubble_width),
        top_right
    ));

    for line in &lines {
        let line_display_width = line.width();
        let padding_needed = (bubble_width - 2).saturating_sub(line_display_width);
        let left_padding = padding_needed / 2;
        let right_padding = padding_needed - left_padding;

        bubble_lines.push(format!(
            "{} {}{}{} {}",
            vertical,
            " ".repeat(left_padding),
            line,
            " ".repeat(right_padding),
            vertical
        ));
    }

    bubble_lines.push(format!(
        "{}{}{}",
        bottom_left,
        horizontal.to_string().repeat(bubble_width),
        bottom_right
    ));

    bubble_lines.push("\\".to_string());
    bubble_lines.push(" \\".to_string());

    bubble_lines
}

pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current_line = String::new();

    for word in words {
        let word_width = word.width();
        let current_width = current_line.width();

        if current_width + word_width < max_width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

pub fn generate_rainbow_cache(color_config: &mut ColorConfig, total_chars: usize) {
    color_config.color_cache.clear();

    if total_chars == 0 {
        return;
    }

    // Generate smooth rainbow gradient across all characters
    for i in 0..total_chars {
        let t = i as f32 / total_chars as f32;
        let hue = t * 360.0; // Full hue spectrum from 0 to 360

        // Create color with specified saturation and brightness
        let color = Color::Hsl(
            hue.round() as u16,
            color_config.saturation,
            color_config.brightness,
        );
        color_config.color_cache.push(color);
    }
}

pub fn generate_gradient_cache(
    color_config: &mut ColorConfig,
    start: &Color,
    end: &Color,
    total_chars: usize,
) {
    color_config.color_cache.clear();

    if total_chars == 0 {
        return;
    }

    // Generate smooth gradient across all characters
    for i in 0..total_chars {
        let t = i as f32 / total_chars as f32;
        let color = lerp_color_hsv(start, end, t);
        color_config.color_cache.push(color);
    }
}

fn list_all_art_styles() {
    println!("Available Miku art styles:");
    println!("==========================");

    // Get all art styles to determine the count
    let all_art = get_miku_art(None, Some(true));
    let styles_count = all_art.len();
    println!("Total styles: {}", styles_count);

    for i in 0..styles_count {
        println!("\n--- Style {} ---", i);
        let art = get_miku_art(Some(i as i32), Some(false));
        for line in art {
            println!("{}", line);
        }
    }

    println!("\nUse --style <number> to select a specific style, or omit for random selection.");
    println!("\nColor Options:");
    println!("  --rainbow                Apply a smooth rainbow gradient");
    println!("  --saturation <0-100>     Set rainbow saturation level (default: 100)");
    println!("  --brightness <0-100>     Set rainbow brightness level (default: 50)");
    println!(
        "  --color <COLOR>          Set a single solid color (supports named colors, hex, rgb, hsl, ansi)"
    );
    println!("  --gradient <START:END>   Define a custom two-color gradient");
    println!("  --no-color               Disable all coloring");
    println!("\nColor Format Examples:");
    println!("  --color red              Named color");
    println!("  --color #FF0000          Hex color");
    println!("  --color 255,0,128        RGB tuple");
    println!("  --color hsl(180,100%,50%) HSL value");
    println!("  --color 93               ANSI color code");
    println!("  --gradient red:blue      Custom gradient");
}

fn draw_miku_says(
    text: &str,
    style: i32,
    color_config: &mut ColorConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let speech_bubble_lines = get_speech_bubble_lines(text);
    let miku_art = get_miku_art(Some(style), None);

    // Calculate total character count for color distribution
    let total_chars: usize = miku_art.iter().map(|line| line.chars().count()).sum();

    // Generate color cache if needed
    if color_config.rainbow {
        generate_rainbow_cache(color_config, total_chars);
    } else if let Some((start, end)) = color_config.gradient.clone() {
        generate_gradient_cache(color_config, &start, &end, total_chars);
    }

    let (window_width, window_height) = size()?;
    let available_width = window_width as usize;

    let mut all_content_lines = Vec::new();
    all_content_lines.extend(speech_bubble_lines.clone());
    all_content_lines.extend(miku_art.iter().map(|s| s.to_string()));

    let max_content_width = all_content_lines
        .iter()
        .map(|l| l.width())
        .max()
        .unwrap_or(0);

    let overall_left_padding = if available_width > max_content_width {
        (available_width - max_content_width) / 2
    } else {
        0
    };

    let bubble_width = speech_bubble_lines
        .first()
        .map(|line| line.width())
        .unwrap_or(0);

    let first_miku_line = miku_art.first().unwrap_or(&"");
    let tail_pos = first_miku_line
        .chars()
        .position(|c| !c.is_whitespace())
        .unwrap_or(0);

    let bubble_center = bubble_width / 2;
    let miku_tail_absolute_pos = overall_left_padding + tail_pos;

    let bubble_left_padding = miku_tail_absolute_pos.saturating_sub(bubble_center);

    let bubble_line_count = speech_bubble_lines.len();

    let content_height = all_content_lines.len();
    let available_height = window_height as usize;

    let top_padding = if content_height < available_height {
        (available_height - content_height) / 2
    } else {
        0
    };

    for _ in 0..top_padding {
        println!();
    }

    // Track color index for non-whitespace characters
    let mut color_index = 0;

    for (i, line) in all_content_lines.iter().enumerate() {
        let is_bubble_line = i < bubble_line_count;
        let is_tail_line =
            is_bubble_line && (i == bubble_line_count - 2 || i == bubble_line_count - 1);

        if is_bubble_line {
            if is_tail_line {
                let tail_padding = overall_left_padding + tail_pos;
                let horizontal_padding = " ".repeat(tail_padding);
                println!("{horizontal_padding}{line}");
            } else {
                let horizontal_padding = " ".repeat(bubble_left_padding);
                println!("{horizontal_padding}{line}");
            }
        } else {
            let horizontal_padding = " ".repeat(overall_left_padding);

            // Apply coloring to Miku art lines
            if color_config.mode != ColorMode::NoColor && !color_config.color_cache.is_empty() {
                let mut colored_line = String::new();
                colored_line.push_str(&horizontal_padding);

                for c in line.chars() {
                    if c.is_whitespace() {
                        colored_line.push(c);
                    } else if color_index < color_config.color_cache.len() {
                        let color = &color_config.color_cache[color_index];
                        let ansi_code = color.to_ansi_fg(color_config.mode);
                        colored_line.push_str(&ansi_code);
                        colored_line.push(c);
                        colored_line.push_str("\x1b[0m"); // Reset
                        color_index += 1;
                    } else {
                        colored_line.push(c);
                    }
                }

                println!("{}", colored_line);
            } else if let Some(single_color) = &color_config.single_color {
                // Apply single solid color
                let ansi_code = single_color.to_ansi_fg(color_config.mode);
                let reset_code = "\x1b[0m";
                println!("{}{}{}{}", horizontal_padding, ansi_code, line, reset_code);
            } else {
                // No coloring
                println!("{}{}", horizontal_padding, line);
            }
        }
    }

    for _ in 0..top_padding {
        println!();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.list {
        list_all_art_styles();
        return Ok(());
    }

    let text = args.text.clone().unwrap_or_else(|| {
        eprintln!("Error: Text is required when not using --list");
        std::process::exit(1);
    });

    let style = args.style.unwrap_or(-1); // Default to -1 to select a random style

    // Parse color-related arguments
    let mut color_config = parse_color_args(&args)?;

    draw_miku_says(&text, style, &mut color_config)?;

    Ok(())
}

/// MARK: Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Color, ColorMode};

    #[test]
    fn test_wrap_text_basic() {
        let text = "Hello world this is a test";
        let wrapped = wrap_text(text, 15);
        assert_eq!(wrapped, vec!["Hello world", "this is a test"]);
    }

    #[test]
    fn test_wrap_text_empty() {
        let text = "";
        let wrapped = wrap_text(text, 10);
        assert_eq!(wrapped, Vec::<String>::new());
    }

    #[test]
    fn test_wrap_text_long_word() {
        let text = "ThisIsAVeryLongWordThatExceedsMaxWidth";
        let wrapped = wrap_text(text, 10);
        assert_eq!(wrapped, vec!["ThisIsAVeryLongWordThatExceedsMaxWidth"]);
    }

    #[test]
    fn test_wrap_text_exact_width() {
        let text = "Hello world";
        let wrapped = wrap_text(text, 11);
        assert_eq!(wrapped, vec!["Hello world"]);
    }

    #[test]
    fn test_get_speech_bubble_lines_basic() {
        let text = "Hello";
        let bubble = get_speech_bubble_lines(text);

        // Should have: top border, text line, bottom border, 2 tail lines
        assert_eq!(bubble.len(), 5);

        // Check top border
        assert!(bubble[0].starts_with('╭'));
        assert!(bubble[0].ends_with('╮'));

        // Check text line
        assert!(bubble[1].contains("Hello"));

        // Check bottom border
        assert!(bubble[2].starts_with('╰'));
        assert!(bubble[2].ends_with('╯'));

        // Check tail
        assert_eq!(bubble[3], "\\");
        assert_eq!(bubble[4], " \\");
    }

    #[test]
    fn test_get_speech_bubble_lines_long_text() {
        let text = "This is a very long text that should be wrapped into multiple lines";
        let bubble = get_speech_bubble_lines(text);

        // Should have: top border, multiple text lines, bottom border, 2 tail lines
        assert!(bubble.len() > 5);

        // Check top border
        assert!(bubble[0].starts_with('╭'));
        assert!(bubble[0].ends_with('╮'));

        // Check bottom border
        let bottom_idx = bubble.len() - 3;
        assert!(bubble[bottom_idx].starts_with('╰'));
        assert!(bubble[bottom_idx].ends_with('╯'));
    }

    #[test]
    fn test_parse_color_args_no_color() {
        let args = Args {
            text: Some("test".to_string()),
            style: None,
            list: false,
            rainbow: false,
            saturation: 100,
            brightness: 50,
            color: None,
            gradient: None,
            no_color: false,
        };

        let config = parse_color_args(&args).unwrap();
        // Don't assert exact color mode since it depends on environment
        assert!(!config.rainbow);
        assert_eq!(config.saturation, 100);
        assert_eq!(config.brightness, 50);
        assert!(config.single_color.is_none());
        assert!(config.gradient.is_none());
        assert!(config.color_cache.is_empty());
    }

    #[test]
    fn test_parse_color_args_no_color_explicit() {
        let args = Args {
            text: Some("test".to_string()),
            style: None,
            list: false,
            rainbow: false,
            saturation: 100,
            brightness: 50,
            color: None,
            gradient: None,
            no_color: true,
        };

        let config = parse_color_args(&args).unwrap();
        assert_eq!(config.mode, ColorMode::NoColor);
    }

    #[test]
    fn test_wrap_text_edge_cases() {
        // Test with single very long word
        let text = "Supercalifragilisticexpialidocious";
        let wrapped = wrap_text(text, 10);
        assert_eq!(wrapped, vec!["Supercalifragilisticexpialidocious"]);

        // Test with exact width match
        let text = "Hello world";
        let wrapped = wrap_text(text, 11);
        assert_eq!(wrapped, vec!["Hello world"]);

        // Test with multiple spaces
        let text = "Hello    world";
        let wrapped = wrap_text(text, 10);
        assert_eq!(wrapped, vec!["Hello", "world"]);
    }

    #[test]
    fn test_get_speech_bubble_lines_edge_cases() {
        // Test with empty string
        let bubble = get_speech_bubble_lines("");
        assert_eq!(bubble.len(), 4); // Should have: top, empty line, bottom, tail (no extra tail line)

        // Test with very long word
        let text = "Supercalifragilisticexpialidocious";
        let bubble = get_speech_bubble_lines(text);
        assert!(bubble.len() >= 5);
        assert!(bubble[0].len() >= 24); // Should accommodate the long word
    }

    #[test]
    fn test_generate_rainbow_cache_edge_cases() {
        let mut config = ColorConfig {
            mode: ColorMode::Truecolor,
            rainbow: true,
            saturation: 100,
            brightness: 50,
            single_color: None,
            gradient: None,
            color_cache: vec![Color::Hex("#000000".to_string())], // Pre-filled
        };

        // Test with 0 characters
        generate_rainbow_cache(&mut config, 0);
        assert_eq!(config.color_cache.len(), 0);

        // Test with 1 character
        generate_rainbow_cache(&mut config, 1);
        assert_eq!(config.color_cache.len(), 1);
    }

    #[test]
    fn test_generate_gradient_cache_edge_cases() {
        let mut config = ColorConfig {
            mode: ColorMode::Truecolor,
            rainbow: false,
            saturation: 100,
            brightness: 50,
            single_color: None,
            gradient: None,
            color_cache: vec![Color::Hex("#000000".to_string())], // Pre-filled
        };

        let start = Color::Hex("#FF0000".to_string());
        let end = Color::Hex("#0000FF".to_string());

        // Test with 0 characters
        generate_gradient_cache(&mut config, &start, &end, 0);
        assert_eq!(config.color_cache.len(), 0);

        // Test with 1 character
        generate_gradient_cache(&mut config, &start, &end, 1);
        assert_eq!(config.color_cache.len(), 1);
    }

    #[test]
    fn test_generate_rainbow_cache() {
        let mut config = ColorConfig {
            mode: ColorMode::Truecolor,
            rainbow: true,
            saturation: 100,
            brightness: 50,
            single_color: None,
            gradient: None,
            color_cache: Vec::new(),
        };

        generate_rainbow_cache(&mut config, 10);
        assert_eq!(config.color_cache.len(), 10);

        // Check that colors are different (rainbow gradient)
        let first_color = &config.color_cache[0];
        let last_color = &config.color_cache[9];
        assert_ne!(first_color, last_color);
    }

    #[test]
    fn test_generate_gradient_cache() {
        let mut config = ColorConfig {
            mode: ColorMode::Truecolor,
            rainbow: false,
            saturation: 100,
            brightness: 50,
            single_color: None,
            gradient: None,
            color_cache: Vec::new(),
        };

        let start = Color::Hex("#FF0000".to_string()); // Red
        let end = Color::Hex("#0000FF".to_string()); // Blue

        generate_gradient_cache(&mut config, &start, &end, 10);
        assert_eq!(config.color_cache.len(), 10);

        // Check that first color is close to start
        let first_color = &config.color_cache[0];
        // Check that last color is close to end
        let last_color = &config.color_cache[9];
        assert_ne!(first_color, last_color);
    }
}
