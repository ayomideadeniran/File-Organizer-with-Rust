use std::path::PathBuf;
use std::fs::{self, create_dir_all, rename};

use crate::category::get_category;

pub fn organising(target_dir: &PathBuf) {
    for entry in fs::read_dir(&target_dir).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();

    if path.is_file() {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let category = get_category(ext);
            let category_path = target_dir.join(category);

            if !category_path.exists() {
                create_dir_all(&category_path).unwrap();
            }

            let file_name = path.file_name().unwrap();
            let new_path = category_path.join(file_name);
            rename(&path, &new_path).unwrap();

            println!("Moved {:?} to {:?}", path, new_path);
        }
    }
}
}



