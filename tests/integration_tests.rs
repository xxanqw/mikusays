use std::process::Command;
use std::str;

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mikusays"))
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
