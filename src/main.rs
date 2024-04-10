use chrono::{DateTime, Utc};
use dircpy::*;
use directories::UserDirs;
use std::fs;
use std::io;
use std::path::Path;

fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry_path = entry?.path();
        if entry_path.is_dir() {
            fs::remove_dir_all(entry_path).expect("Failed to remove a dir");
        } else {
            fs::remove_file(entry_path).expect("Failed to remove a file");
        }
    }
    Ok(())
}

fn sweep_desktop() -> std::io::Result<()> {
    let binding = UserDirs::new().unwrap();
    let cleaned_items_dir = binding.document_dir().unwrap().join("Desktop");
    let desktop_dir = binding.desktop_dir().unwrap();

    let now: DateTime<Utc> = Utc::now();
    let current_year_month = now.format("%Y %B").to_string();

    let destination = cleaned_items_dir.join(current_year_month);

    if !Path::new(&destination).exists() {
        fs::create_dir_all(destination.clone())?;
    }
    copy_dir(desktop_dir, destination)?;
    remove_dir_contents(desktop_dir).unwrap();
    Ok(())
}

fn main() -> std::io::Result<()> {
    sweep_desktop()
}
