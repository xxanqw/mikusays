//! Keyboard-driven config editor. All edits stay in memory until Save.
use crate::{
    Align, Args, Character, config, draw_character_says, get_character_art, parse_color_args,
};
use clap::Parser;
use crossterm::{
    QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{Attribute, Print, SetAttribute},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self, IsTerminal, Write},
    path::PathBuf,
};
use unicode_width::UnicodeWidthChar;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const FIELDS: [&str; 15] = [
    "Character",
    "Style",
    "Size filter",
    "Bubble",
    "Alignment",
    "Color mode",
    "Solid color",
    "Gradient",
    "Saturation",
    "Brightness",
    "Excluded styles",
    "Custom artwork",
    "Config file",
    "Preview message",
    "Save configuration",
];

struct Screen;
impl Screen {
    fn enter() -> Result<Self> {
        terminal::enable_raw_mode()?;
        let screen = Self;
        let mut out = io::stdout();
        out.queue(EnterAlternateScreen)?.queue(Hide)?.flush()?;
        Ok(screen)
    }
}
impl Drop for Screen {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let mut out = io::stdout();
        let _ = out.queue(SetAttribute(Attribute::Reset));
        let _ = out.queue(Show);
        let _ = out.queue(LeaveAlternateScreen);
        let _ = out.flush();
    }
}

fn key() -> Result<KeyEvent> {
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Release {
                return Ok(key);
            }
        } else {
            return Ok(KeyEvent::new(KeyCode::Null, KeyModifiers::NONE));
        }
    }
}

fn clipped(text: &str, width: usize) -> String {
    let mut result = String::new();
    let mut used = 0;
    for ch in text.chars().filter(|ch| !ch.is_control()) {
        let columns = ch.width().unwrap_or(0);
        if used + columns > width {
            break;
        }
        result.push(ch);
        used += columns;
    }
    result
}

fn line(out: &mut io::Stdout, row: usize, text: &str, width: usize) -> Result<()> {
    out.queue(MoveTo(0, row as u16))?
        .queue(Print(clipped(text, width.saturating_sub(1))))?;
    Ok(())
}

fn prompt(label: &str, value: &str) -> Result<Option<String>> {
    let mut value = value.to_string();
    loop {
        let (width, height) = terminal::size()?;
        let mut out = io::stdout();
        out.queue(Clear(ClearType::All))?;
        line(&mut out, 1, label, width as usize)?;
        line(&mut out, 3, &format!("> {value}"), width as usize)?;
        line(
            &mut out,
            5.min(height.saturating_sub(1)) as usize,
            "Type a value; Backspace deletes. Enter applies, Esc cancels.",
            width as usize,
        )?;
        out.flush()?;
        let key = key()?;
        match key.code {
            KeyCode::Esc => return Ok(None),
            KeyCode::Enter => return Ok(Some(value)),
            KeyCode::Backspace => {
                value.pop();
            }
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => value.clear(),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(None),
            KeyCode::Char(ch)
                if !key.modifiers.contains(KeyModifiers::CONTROL) && !ch.is_control() =>
            {
                value.push(ch)
            }
            _ => {}
        }
    }
}

fn confirm(label: &str) -> Result<bool> {
    Ok(prompt(&format!("{label} Type y to confirm."), "")?
        .is_some_and(|value| value.eq_ignore_ascii_case("y")))
}

fn color_mode(args: &Args) -> &'static str {
    if args.no_color {
        "none"
    } else if args.rainbow {
        "rainbow"
    } else if args.gradient.is_some() {
        "gradient"
    } else if args.color.is_some() {
        "solid"
    } else {
        "auto"
    }
}
fn values(args: &Args, path: &std::path::Path, message: &str) -> Vec<String> {
    vec![
        if args.art.is_some() {
            "custom artwork".into()
        } else {
            args.character
                .unwrap_or(if args.teto {
                    Character::Teto
                } else {
                    Character::Miku
                })
                .as_str()
                .into()
        },
        args.style
            .map_or("random (fits terminal)".into(), |style| style.to_string()),
        if args.small {
            "small"
        } else if args.medium {
            "medium"
        } else if args.large {
            "large"
        } else {
            "any"
        }
        .into(),
        if args.think { "thoughts" } else { "speech" }.into(),
        args.align.as_str().into(),
        color_mode(args).into(),
        args.color
            .clone()
            .unwrap_or_else(|| "red (Enter to set)".into()),
        args.gradient
            .clone()
            .unwrap_or_else(|| "cyan:blue (Enter to set)".into()),
        args.saturation.to_string(),
        args.brightness.to_string(),
        args.exclude_styles
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(","),
        args.art
            .as_ref()
            .map_or("none".into(), |path| path.display().to_string()),
        path.display().to_string(),
        message.into(),
        "Enter or S".into(),
    ]
}

fn validate(args: &Args) -> Result<()> {
    let teto = args.teto || args.character == Some(Character::Teto);
    let count = get_character_art(None, Some(true), teto).len();
    if args
        .style
        .iter()
        .chain(args.exclude_styles.iter())
        .any(|style| *style < 0 || *style as usize >= count)
    {
        return Err(format!("Style index must be between 0 and {}", count - 1).into());
    }
    parse_color_args(args)?;
    if let Some(path) = &args.art {
        let art = std::fs::read_to_string(path)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        if art.trim().is_empty() {
            return Err("Custom artwork is empty".into());
        }
    }
    Ok(())
}

fn clear_color(args: &mut Args) {
    args.no_color = false;
    args.rainbow = false;
    args.color = None;
    args.gradient = None;
}

fn edit(field: usize, args: &mut Args, path: &mut PathBuf, message: &mut String) -> Result<bool> {
    let mut next = args.clone();
    match field {
        0 => {
            next.character = Some(
                match args.character.unwrap_or(if args.teto {
                    Character::Teto
                } else {
                    Character::Miku
                }) {
                    Character::Miku => Character::Teto,
                    Character::Teto => Character::Random,
                    Character::Random => Character::Miku,
                },
            );
            next.teto = false;
            next.art = None;
        }
        1 => {
            let Some(value) = prompt(
                "Style: 0-19, or random (Ctrl+U clears)",
                &args
                    .style
                    .map_or("random".into(), |style| style.to_string()),
            )?
            else {
                return Ok(false);
            };
            next.style = if value.trim().is_empty() || value.trim() == "random" {
                None
            } else {
                Some(value.trim().parse()?)
            };
            next.art = None;
        }
        2 => {
            let current = if args.small {
                1
            } else if args.medium {
                2
            } else if args.large {
                3
            } else {
                0
            };
            let selected = (current + 1) % 4;
            next.small = selected == 1;
            next.medium = selected == 2;
            next.large = selected == 3;
        }
        3 => next.think = !args.think,
        4 => {
            next.align = match args.align {
                Align::Left => Align::Center,
                Align::Center => Align::Right,
                Align::Right => Align::Left,
            }
        }
        5 => {
            let mode = color_mode(args);
            clear_color(&mut next);
            match mode {
                "auto" => next.color = Some("red".into()),
                "solid" => next.gradient = Some("cyan:blue".into()),
                "gradient" => next.rainbow = true,
                "rainbow" => next.no_color = true,
                _ => {}
            }
        }
        6 | 7 => {
            let (label, current) = if field == 6 {
                (
                    "Solid color: named, hex, RGB, HSL or ANSI",
                    args.color.as_deref().unwrap_or("red"),
                )
            } else {
                (
                    "Gradient: START:END (for example cyan:blue)",
                    args.gradient.as_deref().unwrap_or("cyan:blue"),
                )
            };
            let Some(value) = prompt(label, current)? else {
                return Ok(false);
            };
            clear_color(&mut next);
            if field == 6 {
                next.color = Some(value);
            } else {
                next.gradient = Some(value);
            }
        }
        8 | 9 => {
            let current = if field == 8 {
                args.saturation
            } else {
                args.brightness
            };
            let Some(value) = prompt("Enter a number from 0 to 100", &current.to_string())? else {
                return Ok(false);
            };
            let value: u8 = value.trim().parse()?;
            if value > 100 {
                return Err("Value must be between 0 and 100".into());
            }
            if field == 8 {
                next.saturation = value;
            } else {
                next.brightness = value;
            }
        }
        10 => {
            let current = args
                .exclude_styles
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",");
            let Some(value) = prompt("Excluded indices, comma-separated (empty = none)", &current)?
            else {
                return Ok(false);
            };
            next.exclude_styles = if value.trim().is_empty() {
                Vec::new()
            } else {
                value
                    .split(',')
                    .map(|index| index.trim().parse())
                    .collect::<std::result::Result<_, _>>()?
            };
        }
        11 => {
            let Some(value) = prompt(
                "Custom UTF-8 art file (empty = use built-in artwork)",
                &args
                    .art
                    .as_ref()
                    .map_or(String::new(), |path| path.display().to_string()),
            )?
            else {
                return Ok(false);
            };
            next.art = if value.is_empty() {
                None
            } else {
                Some(PathBuf::from(value))
            };
            if next.art.is_some() {
                next.character = None;
                next.teto = false;
                next.style = None;
            }
        }
        12 => {
            let Some(value) = prompt("Save/load configuration path", &path.display().to_string())?
            else {
                return Ok(false);
            };
            if value.trim().is_empty() {
                return Err("Config file path cannot be empty".into());
            }
            *path = PathBuf::from(value);
            return Ok(true);
        }
        13 => {
            let Some(value) = prompt("Preview message (not saved in config)", message)? else {
                return Ok(false);
            };
            *message = value;
            return Ok(false);
        }
        _ => return Ok(false),
    }
    validate(&next)?;
    *args = next;
    Ok(true)
}

fn preview(args: &Args, message: &str) -> Result<()> {
    validate(args)?;
    let custom = args.art.as_ref().map(std::fs::read_to_string).transpose()?;
    let mut colors = parse_color_args(args)?;
    // Cooked output keeps renderer newlines correct; the guard still restores on errors.
    terminal::disable_raw_mode()?;
    let result = (|| {
        let mut out = io::stdout();
        out.queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?
            .flush()?;
        draw_character_says(message, args, custom.as_deref(), &mut colors)
    })();
    terminal::enable_raw_mode()?;
    result?;
    let (width, height) = terminal::size()?;
    let mut out = io::stdout();
    line(
        &mut out,
        height.saturating_sub(1) as usize,
        "Preview - press any key to return to configuration",
        width as usize,
    )?;
    out.flush()?;
    key()?;
    Ok(())
}

pub fn run(initial: Args) -> Result<()> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("--configure requires an interactive terminal (stdin and stdout)".into());
    }
    let mut path = initial
        .config
        .clone()
        .or_else(config::default_path)
        .unwrap_or_else(|| PathBuf::from("mikusays.conf"));
    let mut status =
        "Use arrows and Enter to edit. Changes are not saved automatically.".to_string();
    let mut args = Args::parse_from(["mikusays"]);
    if !initial.no_config && path.exists() {
        match config::load(&path).and_then(|loaded| {
            validate(&loaded)?;
            Ok(loaded)
        }) {
            Ok(loaded) => {
                args = loaded;
                status = "Loaded existing configuration.".into();
            }
            Err(error) => {
                status =
                    format!("Could not load config: {error}. Edit defaults and save to repair it.")
            }
        }
    }
    let mut message = initial
        .text
        .unwrap_or_else(|| "Hello from mikusays!".into());
    let mut dirty = false;
    let mut selected: usize = 0;
    let _screen = Screen::enter()?;
    loop {
        let (width, height) = terminal::size()?;
        let mut out = io::stdout();
        out.queue(Clear(ClearType::All))?;
        if width < 45 || height < 14 {
            line(
                &mut out,
                0,
                "Enlarge terminal to at least 45x14; Q exits.",
                width as usize,
            )?;
        } else {
            line(
                &mut out,
                0,
                &format!(
                    "mikusays configuration{}",
                    if dirty { " * unsaved" } else { "" }
                ),
                width as usize,
            )?;
            line(
                &mut out,
                1,
                &format!("File: {}", path.display()),
                width as usize,
            )?;
            let visible = height as usize - 8;
            let start = selected.saturating_sub(visible - 1);
            let values = values(&args, &path, &message);
            for index in start..FIELDS.len().min(start + visible) {
                if index == selected {
                    out.queue(SetAttribute(Attribute::Reverse))?;
                }
                line(
                    &mut out,
                    index - start + 3,
                    &format!(
                        "{} {:<18} {}",
                        if index == selected { ">" } else { " " },
                        FIELDS[index],
                        values[index]
                    ),
                    width as usize,
                )?;
                out.queue(SetAttribute(Attribute::Reset))?;
            }
            line(
                &mut out,
                height as usize - 4,
                "Up/Down: select  Enter: edit/cycle  P: preview  S: save",
                width as usize,
            )?;
            line(
                &mut out,
                height as usize - 3,
                "L: load current file  R: reset defaults  Q/Esc: exit",
                width as usize,
            )?;
            line(&mut out, height as usize - 2, &status, width as usize)?;
        }
        out.flush()?;
        let event = key()?;
        let mut exit = false;
        let result: Result<()> = (|| {
            match event.code {
                KeyCode::Up => selected = selected.checked_sub(1).unwrap_or(FIELDS.len() - 1),
                KeyCode::Down | KeyCode::Tab => selected = (selected + 1) % FIELDS.len(),
                KeyCode::Char('q' | 'Q') | KeyCode::Esc => {
                    if !dirty || confirm("Discard unsaved changes and exit?")? {
                        exit = true;
                    }
                }
                KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                    if !dirty || confirm("Discard unsaved changes and exit?")? {
                        exit = true;
                    }
                }
                KeyCode::Char('p' | 'P') => {
                    preview(&args, &message)?;
                }
                KeyCode::Char('r' | 'R') => {
                    args = Args::parse_from(["mikusays"]);
                    dirty = true;
                    status = "Reset to defaults (not saved).".into();
                }
                KeyCode::Char('l' | 'L') => {
                    if !dirty || confirm("Discard edits and load current file?")? {
                        let loaded = config::load(&path)?;
                        validate(&loaded)?;
                        args = loaded;
                        dirty = false;
                        status = "Loaded configuration.".into();
                    }
                }
                KeyCode::Char('s' | 'S') | KeyCode::Enter
                    if event.code != KeyCode::Enter || selected == 14 =>
                {
                    validate(&args)?;
                    config::save(&args, &path)?;
                    dirty = false;
                    status = format!("Saved to {}", path.display());
                }
                KeyCode::Enter | KeyCode::Right | KeyCode::Char(' ')
                    if edit(selected, &mut args, &mut path, &mut message)? =>
                {
                    dirty = true;
                    status = "Changed (not saved). P previews; S saves.".into();
                }
                _ => {}
            }
            Ok(())
        })();
        if exit {
            break;
        }
        if let Err(error) = result {
            status = format!("Error: {error}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ui_clips_display_columns_and_removes_control_sequences() {
        assert_eq!(clipped("初音ミク", 5), "初音");
        assert_eq!(clipped("a\nb\rc\td", 20), "abcd");
        assert_eq!(clipped("hello", 0), "");
    }
    #[test]
    fn editor_validation_rejects_invalid_styles_and_colors() {
        let mut args = Args::parse_from(["mikusays"]);
        assert!(validate(&args).is_ok());
        args.style = Some(20);
        assert!(validate(&args).is_err());
        args.style = None;
        args.exclude_styles = vec![-1];
        assert!(validate(&args).is_err());
        args.exclude_styles.clear();
        args.color = Some("invalid-color".into());
        assert!(validate(&args).is_err());
        clear_color(&mut args);
        assert_eq!(color_mode(&args), "auto");
        assert!(validate(&args).is_ok());
    }
}
