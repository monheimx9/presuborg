use std::path::{Path, PathBuf};
use walkdir::WalkDir;

struct Episode {
    number: u32,
    lang: String,
    file: String,
}

pub fn list_subtitles() -> Vec<PathBuf> {
    let wd: Vec<PathBuf> = WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().is_file())
        .map(|f| f.into_path())
        .collect();

    for entry in wd.iter() {
        if entry.is_file() {
            println!("{}", entry.display());
        }
    }
    wd
}

fn parse_subtitles_into_episodes(pb: &[PathBuf]) -> Vec<Episode> {
    unimplemented!()
}

fn get_lang_flag_from_filename(p: &Path) -> &str {
    unimplemented!()
}

pub fn group_by_extension(pb: &[PathBuf]) -> Vec<Vec<PathBuf>> {
    unimplemented!();
}

pub fn group_by_lang() {
    unimplemented!();
}
