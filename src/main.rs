use clap::Parser;
use crossterm::terminal::size;
use std::env;
use unicode_width::UnicodeWidthStr;

mod mikuart;
use mikuart::get_miku_art;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Text to display in the speech bubble
    text: String,

    /// Style of the Miku art. Random is chosen if not specified.
    #[arg(short, long)]
    style: Option<i32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let text = args.text;
    let style = match args.style {
        Some(s) => s,
        None => -1, // Default to -1 to select a random style
    };
    draw_miku_says(&text, style)?;

    Ok(())
}

fn draw_miku_says(text: &str, style: i32) -> Result<(), Box<dyn std::error::Error>> {
    let speech_bubble_lines = get_speech_bubble_lines(text);
    let miku_art = get_miku_art(style);

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
            println!("{horizontal_padding}{line}");
        }
    }

    for _ in 0..top_padding {
        println!();
    }

    Ok(())
}

fn get_speech_bubble_lines(text: &str) -> Vec<String> {
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

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
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
