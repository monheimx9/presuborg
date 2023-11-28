use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use regex::{self, Regex};
use serde::{Deserialize, Serialize};
use sevenz_rust::decompress_file;

#[derive(Debug, Serialize, Deserialize)]
struct Language {
    ext: Vec<String>,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LanguageMap(HashMap<String, Language>);

impl LanguageMap {
    fn find_language_key(&self, extension: &str) -> Option<&str> {
        for (key, language) in self.0.iter() {
            if language.ext.contains(&extension.to_lowercase()) {
                return Some(key);
            }
        }
        None
    }
    fn get_track_name(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|lang| lang.name.as_str())
    }
}
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
    let langs = get_languages();
    for pe in pb {
        let lang = get_lang_flag_from_filename(pe, &langs);
    }
    unimplemented!()
}

fn get_lang_flag_from_filename<'a>(p: &Path, langs: &'a LanguageMap) -> &'a str {
    unimplemented!()
}

fn get_episode_number_from_filename(fname: &str) -> u32 {
    let re = Regex::new(r".+(\s|S\d{0,2}E)?(\d{2})\s?.+").unwrap();
    let research = re.captures(fname).unwrap();
    let epnum: u32 = research[1].parse().unwrap();
    epnum
}

fn get_languages() -> LanguageMap {
    let mut file = fs::File::open("langs.json").expect("Failed to open langs.json");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Faile to read json string data");
    LanguageMap(serde_json::from_str(&json_data).expect("Failed to parse json"))
}
