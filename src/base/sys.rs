use std::process::Command;

pub fn is_running(process: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("tasklist")
            .output()
            .expect("failed to execute tasklist");
        String::from_utf8_lossy(&output.stdout).contains(process)
    }

    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("pgrep")
            .arg(process)
            .output()
            .expect("failed to execute pgrep");

        !output.stdout.is_empty()
    }
}
