use std::env;
use std::path::Path;

pub fn change_directory(directory: &str) -> Result<(), String> {
    let home_dir = env::var("HOME").unwrap_or_default();
    let target_dir = if directory == "-" {
        env::var("OLDPWD").unwrap_or_default()
    } else if directory.starts_with('~') {
        let expanded_dir = directory.replacen("~", &home_dir, 1);
        expanded_dir
    } else {
        directory.to_string()
    };
    let target_path = Path::new(&target_dir);
    if !target_path.exists() {
        return Err(format!("Directory '{}' does not exist", target_dir));
    }
    let current_dir = env::current_dir().map_err(|err| format!("Failed to get current directory: {}", err))?;
    let target_abs_path = if target_path.is_relative() {
        current_dir.join(target_path)
    } else {
        target_path.to_path_buf()
    };
    env::set_current_dir(&target_abs_path).map_err(|err| format!("Failed to change directory: {}", err))?;
    env::set_var("OLDPWD", current_dir.to_string_lossy().as_ref());
    Ok(())
}
