use std::process::Command;
use std::str;

fn command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_mikusays"));
    command.env(
        "XDG_CONFIG_HOME",
        std::env::temp_dir().join(format!("mikusays-no-config-{}", std::process::id())),
    );
    command
}

#[test]
fn test_cli_teto_list_contains_all_twenty_approved_artworks() {
    let output = command().args(["--teto", "--list"]).output().unwrap();
    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Available Teto art styles:"));
    assert!(stdout.contains("Total styles: 20"));
    assert_eq!(stdout.matches("--- Style").count(), 20);
    assert!(stdout.contains(include_str!("../src/teto/small.txt").trim_end_matches('\n')));
    assert!(stdout.contains(include_str!("../src/teto/large.txt").trim_end_matches('\n')));
    for index in 2..20 {
        let path = format!(
            "{}/src/teto/style-{index:02}.txt",
            env!("CARGO_MANIFEST_DIR")
        );
        let artwork = std::fs::read_to_string(path).unwrap();
        assert!(stdout.contains(artwork.trim_end_matches('\n')));
    }
    assert!(!stdout.contains("Available Miku art styles:"));
}

#[test]
fn test_cli_default_list_does_not_mix_in_teto() {
    let output = command().arg("--list").output().unwrap();
    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Available Miku art styles:"));
    for artwork in [
        include_str!("../src/teto/small.txt"),
        include_str!("../src/teto/large.txt"),
    ] {
        assert!(!stdout.contains(artwork.trim_end_matches('\n')));
    }
}

#[test]
fn test_cli_teto_style_bounds_match_the_expanded_collection() {
    let output = command()
        .args(["test", "--teto", "--style", "20"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("style 20 does not exist; choose a value between 0 and 19"));
    assert!(!stderr.contains("panicked"));

    let output = command()
        .args(["test", "--teto", "--style", "19", "--no-color"])
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
fn test_cli_list_styles() {
    let output = command()
        .arg("--list")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

    // Should contain style listing information
    assert!(stdout.contains("Available Miku art styles:"));
    assert!(stdout.contains("Total styles:"));
    assert!(stdout.contains("--- Style"));
}

#[test]
fn test_cli_error_no_text() {
    let output = command().output().expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Error: Text is required when not using --list"));
}

#[test]
fn test_cli_error_conflicting_args() {
    let output = command()
        .arg("Test")
        .arg("--rainbow")
        .arg("--color")
        .arg("red")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("cannot be used with"));
}

#[test]
fn test_cli_basic_functionality() {
    // Test that the binary runs and produces some output
    let output = command()
        .arg("--list")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_cli_no_color_overrides_solid_color() {
    let output = command()
        .args(["test", "--style", "0", "--color", "red", "--no-color"])
        .env("TERM", "xterm-256color")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(!output.stdout.contains(&0x1b));
}

#[test]
fn test_cli_no_color_environment_overrides_gradient() {
    let output = command()
        .args(["test", "--style", "0", "--gradient", "red:blue"])
        .env("NO_COLOR", "1")
        .env("TERM", "xterm-256color")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(!output.stdout.contains(&0x1b));
}

#[test]
fn test_cli_rejects_out_of_range_style_without_panicking() {
    let output = command()
        .args(["test", "--style", "999"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(2));
    let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("style 999 does not exist"));
    assert!(!stderr.contains("panicked"));
}

#[test]
fn test_cli_rejects_negative_style() {
    let output = command()
        .args(["test", "--style", "-1"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(2));
    let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
    assert!(!stderr.contains("panicked"));
}

#[test]
fn test_cli_rejects_out_of_range_saturation() {
    let output = command()
        .args(["test", "--rainbow", "--saturation", "101"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_rejects_invalid_color() {
    let output = command()
        .args(["test", "--color", "not-a-color"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(2));
    let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("invalid color 'not-a-color'"));
}

struct Fixture(std::path::PathBuf);
impl Fixture {
    fn new() -> Self {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "mikusays-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn file(&self, name: &str, contents: &str) -> std::path::PathBuf {
        let path = self.0.join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, contents).unwrap();
        path
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn success(output: std::process::Output) -> String {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn stdin_supports_unicode_and_paragraphs_but_explicit_text_wins() {
    use std::io::Write;
    use std::process::Stdio;
    for text in [None, Some("explicit")] {
        let mut cmd = command();
        cmd.args(["--style", "0", "--no-color"]);
        if let Some(text) = text {
            cmd.arg(text);
        }
        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all("Привіт\n初音ミク\n".as_bytes())
            .unwrap();
        let stdout = success(child.wait_with_output().unwrap());
        if text.is_some() {
            assert!(stdout.contains("explicit"));
            assert!(!stdout.contains("Привіт"));
        } else {
            assert!(stdout.contains("Привіт"));
            assert!(stdout.contains("初音ミク"));
        }
    }
}

#[test]
fn custom_art_thought_bubbles_and_alignment_work_together() {
    use unicode_width::UnicodeWidthStr;
    let fixture = Fixture::new();
    let art = fixture.file("my art.txt", "  CUSTOM-ART\n   界\n");
    let mut starts = Vec::new();
    for align in ["left", "center", "right"] {
        let stdout = success(
            command()
                .args(["hello", "--think", "--align", align, "--no-color", "--art"])
                .arg(&art)
                .output()
                .unwrap(),
        );
        let lines: Vec<_> = stdout.lines().collect();
        let top = lines.iter().position(|line| line.contains('╭')).unwrap();
        let left = lines[top].find('╭').unwrap();
        let width = lines[top].trim().width();
        assert_eq!(lines[top + 3].find('o').unwrap(), left + width / 2);
        assert_eq!(lines[top + 4].find('o').unwrap(), left + width / 2 + 1);
        assert!(stdout.contains("  CUSTOM-ART\n"));
        starts.push(left);
    }
    assert_eq!(starts[0], 0);
    assert!(starts[0] < starts[1] && starts[1] < starts[2]);
}

#[test]
fn random_character_listing_includes_both_collections() {
    let stdout = success(
        command()
            .args(["--character", "random", "--list"])
            .output()
            .unwrap(),
    );
    assert!(stdout.contains("Available Miku art styles:"));
    assert!(stdout.contains("Available Teto art styles:"));
    assert_eq!(stdout.matches("--- Style").count(), 40);
    let teto = success(
        command()
            .args(["--character", "teto", "--list"])
            .output()
            .unwrap(),
    );
    assert!(teto.contains("Available Teto"));
    assert!(!teto.contains("Available Miku"));
}

#[test]
fn excluded_collections_fall_back_to_text_and_style_overrides_filters() {
    let excludes = (0..20).map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let stdout = success(
        command()
            .args([
                "hello",
                "--character",
                "random",
                "--exclude-styles",
                &excludes,
                "--align",
                "left",
                "--no-color",
            ])
            .output()
            .unwrap(),
    );
    assert_eq!(stdout, "hello\n");
    let stdout = success(
        command()
            .args([
                "hello",
                "--teto",
                "--style",
                "19",
                "--small",
                "--exclude-styles",
                &excludes,
                "--no-color",
            ])
            .output()
            .unwrap(),
    );
    assert!(
        stdout.contains(
            include_str!("../src/teto/style-19.txt")
                .lines()
                .find(|line| !line.trim().is_empty())
                .unwrap()
        )
    );
}

#[test]
fn config_round_trip_and_cli_override_character_size_color_and_style() {
    let fixture = Fixture::new();
    let saved = fixture.0.join("saved.conf");
    success(
        command()
            .args([
                "--teto",
                "--style",
                "0",
                "--think",
                "--align",
                "right",
                "--medium",
                "--color",
                "red",
                "--save-config",
            ])
            .arg(&saved)
            .output()
            .unwrap(),
    );
    let contents = std::fs::read_to_string(&saved).unwrap();
    assert!(contents.contains("--think"));
    let stdout = success(
        command()
            .args(["hello", "--config"])
            .arg(&saved)
            .arg("--no-color")
            .output()
            .unwrap(),
    );
    assert!(
        stdout.contains(
            include_str!("../src/teto/small.txt")
                .lines()
                .find(|line| !line.trim().is_empty())
                .unwrap()
        )
    );
    assert!(!stdout.contains('\\'));
    let stdout = success(
        command()
            .args(["hello", "--config"])
            .arg(&saved)
            .args([
                "--character",
                "miku",
                "--small",
                "--gradient",
                "red:blue",
                "--style",
                "1",
                "--no-color",
            ])
            .output()
            .unwrap(),
    );
    assert!(stdout.contains("hello"));
    assert!(!stdout.contains(include_str!("../src/teto/small.txt").trim()));
}

#[test]
fn config_relative_art_paths_and_custom_art_cli_overrides() {
    let fixture = Fixture::new();
    fixture.file("my art.txt", "MY-CUSTOM-ART\n");
    let config = fixture.file("custom.conf", "--art my art.txt\n--think\n");
    let stdout = success(
        command()
            .args(["hello", "--config"])
            .arg(&config)
            .output()
            .unwrap(),
    );
    assert!(stdout.contains("MY-CUSTOM-ART"));
    let stdout = success(
        command()
            .args(["hello", "--config"])
            .arg(&config)
            .args(["--style", "0"])
            .output()
            .unwrap(),
    );
    assert!(!stdout.contains("MY-CUSTOM-ART"));
    let config = fixture.file("builtin.conf", "--teto\n--style 19\n");
    let art = fixture.0.join("my art.txt");
    let stdout = success(
        command()
            .args(["hello", "--config"])
            .arg(&config)
            .arg("--art")
            .arg(&art)
            .output()
            .unwrap(),
    );
    assert!(stdout.contains("MY-CUSTOM-ART"));
}

#[test]
fn automatic_config_can_be_disabled_and_reports_invalid_values() {
    let fixture = Fixture::new();
    fixture.file("mikusays/config", "--definitely-invalid\n");
    let output = command()
        .env("XDG_CONFIG_HOME", &fixture.0)
        .arg("hello")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unsupported config option"));
    success(
        command()
            .env("XDG_CONFIG_HOME", &fixture.0)
            .args(["hello", "--no-config"])
            .output()
            .unwrap(),
    );
    fixture.file("mikusays/config", "--align sideways\n");
    let output = command()
        .env("XDG_CONFIG_HOME", &fixture.0)
        .arg("hello")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn cli_rejects_conflicts_bad_exclusions_and_empty_custom_art() {
    for args in [
        vec!["hello", "--small", "--large"],
        vec!["hello", "--teto", "--character", "random"],
        vec!["hello", "--exclude-styles", "20"],
        vec!["hello", "--exclude-styles", "-1"],
        vec!["hello", "--align", "up"],
    ] {
        let output = command().args(args).output().unwrap();
        assert_eq!(output.status.code(), Some(2));
    }
    let fixture = Fixture::new();
    let empty = fixture.file("empty.txt", "  \n");
    let output = command()
        .args(["hello", "--art"])
        .arg(empty)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("custom artwork is empty"));
}

#[test]
fn automatic_defaults_load_and_cli_overrides_saved_no_color() {
    let fixture = Fixture::new();
    fixture.file(
        "mikusays/config",
        "--teto\n--think\n--style 0\n--no-color\n",
    );
    let stdout = success(
        command()
            .env("XDG_CONFIG_HOME", &fixture.0)
            .arg("hello")
            .output()
            .unwrap(),
    );
    assert!(stdout.contains("hello"));
    assert!(!stdout.contains('\\'));
    assert!(!stdout.contains('\u{1b}'));
    let stdout = success(
        command()
            .env("XDG_CONFIG_HOME", &fixture.0)
            .env_remove("NO_COLOR")
            .env("TERM", "xterm-256color")
            .args(["hello", "--color", "red"])
            .output()
            .unwrap(),
    );
    assert!(stdout.contains('\u{1b}'));
}

#[test]
fn missing_explicit_files_report_errors() {
    let fixture = Fixture::new();
    let missing = fixture.0.join("missing");
    for option in ["--config", "--art"] {
        let output = command()
            .args(["hello", option])
            .arg(&missing)
            .output()
            .unwrap();
        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("cannot read"), "{stderr}");
        assert!(stderr.contains("missing"), "{stderr}");
    }
}
