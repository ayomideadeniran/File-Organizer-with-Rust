use std::fs::{self, create_dir_all, rename}; // fs: make i fit read and write files the wsy i wsnt am. createdirall: i use thid to create to create folders as i choose. // rename: raname and shifting of files. 
use std::path::{Path, PathBuf}; // working with file become easier with this tool, you have patial control of files.


pub fn organize(target_dir: &PathBuf) {
    if !target_dir.exists() || !target_dir.is_dir() {
        eprintln!("specified path is not really valid directory."); // check if the path exits in my laptop
        return;
    }

    for entry in fs::read_dir(&target_dir).unwrap() { // list all tha files i have in that folder
        let entry = entry.unwrap(); // if something goes wrong it might stop it
        let path = entry.path(); // access the file path

        if path.is_file() { // i think am accesing files here not folders
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) { // i think am checking if the file is an extension
                let category = get_category(ext); // diagnosing which folder it should be
                let category_path = target_dir.join(category);

                if !category_path.exists() {  // create it if it doesnt exist
                    create_dir_all(&category_path).unwrap();  
                }

                let file_name = path.file_name().unwrap();  // extract the file name
                let new_path = category_path.join(file_name); // joining it together in names
                rename(&path, &new_path).unwrap(); // renaming 

                println!("Moved {:?} to {:?}", path, new_path); // showing the file shifting 
            }
        }
    }
}

fn get_category(extension: &str) -> &str { //
    match extension {
        "jpg" | "jpeg" | "png" | "gif" | "webp" => "Images",
        "mp4" | "mov" | "mkv" => "Videos",
        "mp3" | "wav" | "flac" => "Audio",
        "pdf" | "docx" | "txt" | "csv" => "Docs",
        "zip" | "rar" => "Archives",
        _ => "Others",
    }
}