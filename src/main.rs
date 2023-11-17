use std::fs;
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

    Ok(())
}
