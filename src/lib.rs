use walkdir::WalkDir;

pub fn list_files() {
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }
}
