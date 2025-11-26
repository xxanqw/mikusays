use std::process::Command;
use std::str;

#[test]
fn test_cli_list_styles() {
    let output = Command::new("target/debug/mikusays")
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
    let output = Command::new("target/debug/mikusays")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Error: Text is required when not using --list"));
}

#[test]
fn test_cli_error_conflicting_args() {
    let output = Command::new("target/debug/mikusays")
        .arg("Test")
        .arg("--rainbow")
        .arg("--color")
        .arg("red")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Cannot use --rainbow and --color together"));
}

#[test]
fn test_cli_basic_functionality() {
    // Test that the binary runs and produces some output
    let output = Command::new("target/debug/mikusays")
        .arg("--list")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}
