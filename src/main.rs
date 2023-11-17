use std::{
    fs, io,
    path::{Path, PathBuf},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the path to the directory
    let directory_path = "."; // Change this to your desired directory path
    let mut files: Vec<String> = Vec::new();

    // Read the contents of the directory
    let entries = fs::read_dir(directory_path)?;
    // Iterate over the directory entries
    for entry in entries {
        // Get the file path as a string
        let file_path = entry?.path();
        let file_path_str = file_path.to_string_lossy().into_owned();
        files.push(file_path_str.clone());

        println!("File path: {}", file_path_str);
    }
    let p = Path::new("/home/monheim/Downloads/");

    let mut files2: Vec<String> = get_files(p)?
        .into_iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect();
    for f in files2.iter() {
        println!("{f}");
    }
    files2.sort();
    for f in files2.iter() {
        println!("{f}");
    }
    // let _ = files2.into_iter().map(|s| println!("{s}"));

    Ok(())
}

fn get_files(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)?
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap().path())
        .filter(|f| f.is_file())
        .collect())
}
