use crate::Args;
use clap::{CommandFactory, FromArgMatches};
use std::ffi::OsString;
use std::path::PathBuf;

pub fn default_path() -> Option<PathBuf> {
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

pub fn load(path: &std::path::Path) -> Result<Args, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let options = config_options(&contents, path)?;
    Ok(Args::from_arg_matches(
        &Args::command().try_get_matches_from(
            std::iter::once(OsString::from("mikusays")).chain(
                options.into_iter().flat_map(|(option, value)| {
                    std::iter::once(OsString::from(option)).chain(value)
                }),
            ),
        )?,
    )?)
}

type ConfigOptions = Vec<(String, Option<OsString>)>;

fn config_options(
    contents: &str,
    path: &std::path::Path,
) -> Result<ConfigOptions, Box<dyn std::error::Error>> {
    let mut options = Vec::new();
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
        let value = value.map(|value| {
            if option == "--art" {
                path.parent()
                    .unwrap_or(std::path::Path::new("."))
                    .join(value)
                    .into_os_string()
            } else {
                OsString::from(value)
            }
        });
        options.push((option.to_string(), value));
    }
    Ok(options)
}

pub fn parse() -> Result<Args, Box<dyn std::error::Error>> {
    let cli: Vec<OsString> = std::env::args_os().collect();
    // Validate the CLI independently, including conflicts and help/version exits.
    let matches = Args::command().get_matches_from(&cli);
    let initial = Args::from_arg_matches(&matches)?;
    if initial.no_config || initial.configure {
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
    for (option, value) in config_options(&contents, &path)? {
        let option = option.as_str();
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
            defaults.push(value);
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
