

use std::{fs, process::Command};

use rstest::rstest;

#[rstest]
fn test_enum_strum_serde_compiles_with_renamed_serde() {
    let temp_dir = tempfile::tempdir().unwrap();
    let source_dir = temp_dir.path().join("src");
    fs::create_dir(&source_dir).unwrap();

    let model_path = env!("CARGO_MANIFEST_DIR").replace('\\', "/");
    let manifest = format!(
        r#"[package]
name = "serde-renamed-consumer"
version = "0.0.0"
edition = "2024"

[workspace]

[dependencies]
nice-model = {{ path = "{model_path}" }}
renamed-serde = {{ package = "serde", version = "1" }}
"#,
    );
    fs::write(temp_dir.path().join("Cargo.toml"), manifest).unwrap();
    fs::write(
        source_dir.join("main.rs"),
        include_str!("../test_data/serde_renamed/main.rs"),
    )
    .unwrap();

    let output = Command::new(env!("CARGO"))
        .args([
            "check",
            "--offline",
            "--quiet",
            "--jobs",
            "2",
            "--manifest-path",
        ])
        .arg(temp_dir.path().join("Cargo.toml"))
        .arg("--target-dir")
        .arg(temp_dir.path().join("target"))
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "renamed serde consumer failed to compile\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
