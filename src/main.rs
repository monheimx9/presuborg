use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

use regex::{self, Regex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let season = "S01";
    let tvdbid = "418183";

    let p = Path::new("/home/monheim/Downloads/Bucchigire/French/");

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
    test_continuity(files2)?;
    Ok(())
}

fn test_continuity(episodes: Vec<String>) -> Result<(), io::Error> {
    let re = Regex::new(r".+\s(\d{2})\s.+").unwrap();
    let mut index: u8 = 1;
    for ep in episodes.iter() {
        let curr = re.captures(ep).unwrap();
        let curr0 = curr[1].parse::<u8>().unwrap();
        if curr0 == index {
            println!("hehe");
        } else {
            println!(":((((((");
            panic!("hehe")
        }
        index += 1;
    }
    Ok(())
}

fn get_files(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)?
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap().path())
        .filter(|f| f.is_file())
        .collect())
}
