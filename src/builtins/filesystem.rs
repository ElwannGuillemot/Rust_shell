use std::fs;
use std::path::Path;

pub fn create_directory(directory: &str) -> Result<(), String> {
    let path = Path::new(directory);

    if path.exists() {
        return Err(format!("Directory '{}' already exists", directory));
    }
    fs::create_dir_all(directory).map_err(|err| format!("Failed to create directory: {}", err))?;
    Ok(())
}

pub fn remove_directory(directory: &str) -> Result<(), String> {
    let path = Path::new(directory);

    if !path.exists() {
        return Err(format!("Directory '{}' does not exist", directory));
    }
    fs::remove_dir_all(directory).map_err(|err| format!("Failed to remove directory: {}", err))?;
    Ok(())
}

pub fn list_directory_contents(directory: &str) -> Result<Vec<String>, String> {
    let path = Path::new(directory);

    if !path.exists() {
        return Err(format!("Directory '{}' does not exist", directory));
    }
    let entries = fs::read_dir(directory)
        .map_err(|err| format!("Failed to read directory contents: {}", err))?
        .map(|entry| {
            entry
                .map_err(|err| format!("Failed to read directory entry: {}", err))
                .and_then(|entry| {
                    entry
                        .file_name()
                        .into_string()
                        .map_err(|_| "Failed to convert file name to string".to_string())
                })
        })
        .collect::<Result<Vec<String>, String>>()?;
    Ok(entries)
}
