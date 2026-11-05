use crate::Args;
use clap::{CommandFactory, FromArgMatches};
use std::ffi::OsString;
use std::path::PathBuf;

fn default_path() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(path).join("mikusays/config"));
    }
    #[cfg(windows)]
    if let Some(path) = std::env::var_os("APPDATA") {
        return Some(PathBuf::from(path).join("mikusays/config"));
    }
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config/mikusays/config"))
}

fn group(option: &str) -> &str {
    match option {
        "--teto" | "--character" => "character",
        "--small" | "--medium" | "--large" => "size",
        "--rainbow" | "--color" | "--gradient" | "--no-color" => "color",
        other => other,
    }
}

const OPTIONS: &[&str] = &[
    "--teto",
    "--character",
    "--style",
    "--small",
    "--medium",
    "--large",
    "--think",
    "--align",
    "--art",
    "--exclude-styles",
    "--rainbow",
    "--color",
    "--gradient",
    "--saturation",
    "--brightness",
    "--no-color",
];

pub fn parse() -> Result<Args, Box<dyn std::error::Error>> {
    let cli: Vec<OsString> = std::env::args_os().collect();
    // Validate the CLI independently, including conflicts and help/version exits.
    let matches = Args::command().get_matches_from(&cli);
    let initial = Args::from_arg_matches(&matches)?;
    if initial.no_config {
        return Ok(initial);
    }
    let explicit = initial.config.is_some();
    let Some(path) = initial.config.clone().or_else(default_path) else {
        return Ok(initial);
    };
    let contents = match std::fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(error) if !explicit && error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(initial);
        }
        Err(error) => return Err(format!("cannot read config {}: {error}", path.display()).into()),
    };
    let mut defaults = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (option, value) = line
            .split_once(['=', ' ', '\t'])
            .map_or((line, None), |(key, value)| (key, Some(value.trim())));
        if !OPTIONS.contains(&option) {
            return Err(format!(
                "{}:{}: unsupported config option {option}",
                path.display(),
                index + 1
            )
            .into());
        }
        let id = option.trim_start_matches('-').replace('-', "_");
        let overridden = matches.ids().any(|cli_id| {
            matches.value_source(cli_id.as_str()) == Some(clap::parser::ValueSource::CommandLine)
                && (cli_id.as_str() == id
                    || group(&format!("--{}", cli_id.as_str().replace('_', "-"))) == group(option))
        });
        let art_override =
            initial.art.is_some() && matches!(option, "--style" | "--teto" | "--character");
        let collection_override = option == "--art"
            && (initial.style.is_some()
                || initial.teto
                || initial.character.is_some()
                || initial.list);
        if overridden || art_override || collection_override {
            continue;
        }
        defaults.push(OsString::from(option));
        if let Some(value) = value {
            if option == "--art" {
                let art = PathBuf::from(value);
                defaults.push(
                    path.parent()
                        .unwrap_or(std::path::Path::new("."))
                        .join(art)
                        .into_os_string(),
                );
            } else {
                defaults.push(OsString::from(value));
            }
        }
    }
    let merged = std::iter::once(cli[0].clone())
        .chain(defaults)
        .chain(cli.into_iter().skip(1));
    Ok(Args::from_arg_matches(
        &Args::command().get_matches_from(merged),
    )?)
}

pub fn save(args: &Args, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = vec!["# mikusays defaults: one option per line".to_string()];
    macro_rules! flag {
        ($field:ident, $name:literal) => {
            if args.$field {
                lines.push($name.into());
            }
        };
    }
    flag!(teto, "--teto");
    flag!(small, "--small");
    flag!(medium, "--medium");
    flag!(large, "--large");
    flag!(think, "--think");
    flag!(rainbow, "--rainbow");
    flag!(no_color, "--no-color");
    if let Some(character) = args.character {
        lines.push(format!("--character {}", character.as_str()));
    }
    if let Some(style) = args.style {
        lines.push(format!("--style {style}"));
    }
    lines.push(format!("--align {}", args.align.as_str()));
    lines.push(format!("--saturation {}", args.saturation));
    lines.push(format!("--brightness {}", args.brightness));
    if let Some(color) = &args.color {
        lines.push(format!("--color {color}"));
    }
    if let Some(gradient) = &args.gradient {
        lines.push(format!("--gradient {gradient}"));
    }
    if !args.exclude_styles.is_empty() {
        lines.push(format!(
            "--exclude-styles {}",
            args.exclude_styles
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        ));
    }
    if let Some(art) = &args.art {
        lines.push(format!("--art {}", std::fs::canonicalize(art)?.display()));
    }
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, lines.join("\n") + "\n")?;
    Ok(())
}
