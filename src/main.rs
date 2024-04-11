#![windows_subsystem = "windows"]
use chrono::{DateTime, Utc};
use dircpy::*;
use directories::UserDirs;
use eframe::egui;
use std::fs;
use std::io;
use std::path::Path;

extern crate rfd;

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

fn sweep_desktop(cleaned_items_dir: &Path, group_by: &str) -> std::io::Result<()> {
    let binding = UserDirs::new().unwrap();
    let desktop_dir = binding.desktop_dir().unwrap();

    let now: DateTime<Utc> = Utc::now();
    let dest_dir_format;
    if group_by == "month" {
        dest_dir_format = now.format("%Y %B").to_string();
    } else if group_by == "day" {
        dest_dir_format = now.format("%Y %B %d").to_string();
    } else {
        dest_dir_format = "".to_string();
    }

    let destination = cleaned_items_dir.join(dest_dir_format);

    if !Path::new(&destination).exists() {
        fs::create_dir_all(destination.clone())?;
    }
    copy_dir(desktop_dir, destination)?;
    remove_dir_contents(desktop_dir).unwrap();
    Ok(())
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 120.0]),
        ..Default::default()
    };

    let mut open_panel = 0;

    let groupings = ["Group by month", "Group by day", "Don't group"];
    let mut selected = 0;

    let binding = UserDirs::new().unwrap();
    let default_cleaned_items_dir = binding
        .document_dir()
        .unwrap()
        .join("Desktop")
        .to_str()
        .unwrap()
        .to_string();

    let mut current_uri: String = default_cleaned_items_dir;
    let mut uri_edit_text: String = current_uri.clone();

    eframe::run_simple_native("Desk Sweep", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let style = egui::Style {
                visuals: egui::Visuals::dark(),
                ..egui::Style::default()
            };
            ctx.set_style(style);
            ui.horizontal(|ui| {
                ui.selectable_value(&mut open_panel, 0, "Status");
                ui.selectable_value(&mut open_panel, 1, "Settings");
            });
            if open_panel == 0 {
                ui.centered_and_justified(|ui| {
                    if ui.button("Sweep").clicked() {
                        let group_by;
                        if selected == 0 {
                            group_by = "month";
                        } else if selected == 1 {
                            group_by = "day";
                        } else {
                            group_by = "none";
                        }
                        let _sweep_desktop = sweep_desktop(Path::new(&current_uri), group_by);
                    }
                });
            } else if open_panel == 1 {
                ui.label("Folder for sweeped items");
                ui.horizontal(|ui| {
                    let text_response = ui.text_edit_singleline(&mut uri_edit_text);
                    if text_response.changed() {
                        current_uri = uri_edit_text.clone();
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Browse").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            uri_edit_text = format!("{}", path.display());
                            current_uri = uri_edit_text.clone();
                        }
                    }
                });
                ui.label("Group items into subfolders");
                egui::ComboBox::from_id_source("Group items into subfolders").show_index(
                    ui,
                    &mut selected,
                    groupings.len(),
                    |i| groupings[i],
                );
            }
        });
    })
}
