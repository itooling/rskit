#[cfg(target_os = "linux")]
pub fn is_running(process: &str) -> bool {
    use std::process::Command;
    let output = Command::new("pgrep")
        .arg(process)
        .output()
        .expect("failed to execute pgrep");

    !output.stdout.is_empty()
}

pub fn is_running_current() -> bool {
    let pid = std::process::id();
    let exe = std::env::current_exe().unwrap();
    let exe_name = exe.to_str().unwrap();
    let system = sysinfo::System::new_all();
    for ele in system.processes().values() {
        if ele.name().to_str().unwrap() == exe_name && ele.pid().as_u32() != pid {
            return true;
        }
    }
    return false;
}

#[cfg(target_os = "windows")]
pub fn set_windows_startup(name: &str) -> Result<(), anyhow::Error> {
    let exe = std::env::current_exe()?;
    let exe_name = exe.to_str().unwrap();
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let (run, _) = hkcu.create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run")?;
    run.set_value(name, &exe_name)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_running_current() {
        assert_eq!(false, is_running_current());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_set_windows_startup() {
        set_windows_startup("test").unwrap();
    }
}
