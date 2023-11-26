use walkdir::WalkDir;

pub fn list_files() {
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().is_file())
    {
        if entry.file_type().is_file() {
            println!("{}", entry.path().display());
        }
    }
}
