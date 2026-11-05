use std::process::Command;
use std::str;

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mikusays"))
}

#[test]
fn test_cli_teto_list_contains_only_the_issue_artworks() {
    let output = command().args(["--teto", "--list"]).output().unwrap();
    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Available Teto art styles:"));
    assert!(stdout.contains("Total styles: 2"));
    assert_eq!(stdout.matches("--- Style").count(), 2);
    assert!(stdout.contains(include_str!("../src/teto/small.txt").trim_end_matches('\n')));
    assert!(stdout.contains(include_str!("../src/teto/large.txt").trim_end_matches('\n')));
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
fn test_cli_teto_style_indices_are_separate_from_miku() {
    let output = command()
        .args(["test", "--teto", "--style", "2"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("style 2 does not exist; choose a value between 0 and 1"));
    assert!(!stderr.contains("panicked"));

    let output = command()
        .args(["test", "--style", "2", "--color-debug"])
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
