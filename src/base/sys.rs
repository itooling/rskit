#[cfg(target_os = "linux")]
pub fn is_running(process: &str) -> bool {
    use std::process::Command;
    let output = Command::new("pgrep")
        .arg(process)
        .output()
        .expect("failed to execute pgrep");

    !output.stdout.is_empty()
}
