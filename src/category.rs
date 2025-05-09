
pub fn get_category(extension: &str) -> &str {
    match extension {
        "jpg" | "jpeg" | "png" | "gif" => "Images",
        "mp4" | "mov" | "mkv" => "Videos",
        "pdf" | "docx" | "txt" => "Docs",
        "zip" | "rar" => "Archives",
        _ => "Others",
    }
}

