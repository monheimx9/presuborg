use std::path::PathBuf;
use walkdir::WalkDir;

pub fn list_subtitles() {
    let wd: Vec<PathBuf> = WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().is_file())
        .map(|f| f.into_path())
        .collect();

    for entry in wd {
        if entry.is_file() {
            println!("{}", entry.display());
        }
    }
}
