use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn make_dirs<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn delete_all_temp_folders<P: AsRef<Path>>(base_path: P) -> Result<()> {
    if !base_path.as_ref().exists() {
        return Ok(());
    }
    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let temp_path = path.join("temp");
            if temp_path.exists() {
                let _ = fs::remove_dir_all(temp_path);
            }
        }
    }
    Ok(())
}

pub fn human_readable_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}
