use std::{
    collections::HashMap,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use regex::{self, Regex};
use serde::{Deserialize, Serialize};
use sevenz_rust::decompress_file;

#[derive(Debug, Clone)]
struct LesParam<'a> {
    rel_group: &'a str,
    tvdbid: &'a str,
    season: &'a str,
    track_name: &'a str,
}

#[derive(Debug, Clone)]
struct FileName<'a> {
    old: &'a str,
    new: &'a str,
}

#[derive(Debug, Clone)]
struct Episode<'a> {
    number: u8,
    f_name: &'a FileName<'a>,
}

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = Path::new("/home/monheim/Downloads/Bucchigire/");

    let mut file = fs::File::open("langs.json").expect("Failed to open langs.json");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read langs.json");

    let languages: LanguageMap = serde_json::from_str(&json_data).expect("Failed to parse JSON");

    let param = LesParam {
        rel_group: "Erai-raws",
        tvdbid: "418183",
        season: "01",
        track_name: "Français (France)",
    };

    let archives = get_archive_files(p)?;
    extract_archives(&archives)?;

    let mut files: Vec<String> = get_files(p)?
        .into_iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect();
    for f in files.iter() {
        println!("{f}");
    }
    files.sort();
    for f in files.iter() {
        println!("{f}");
    }
    // let _ = files2.into_iter().map(|s| println!("{s}"));
    test_continuity(&files)?;
    let mut file_renamed = files.clone();
    rename_subs(&mut file_renamed, param);

    let mut episodes: HashMap<u8, FileName> = HashMap::new();

    for (index, file) in files.iter().enumerate() {
        let old1 = file;
        let new1 = &file_renamed[index];

        episodes.insert(
            (index + 1) as u8,
            FileName {
                old: old1,
                new: new1,
            },
        );
    }

    for f in files.iter() {
        println!("{f}");
    }
    let episodes_vec = Vec::from_iter(episodes.values());

    rename_subtitle_files(&episodes_vec);

    Ok(())
}

fn rename_subtitle_files(subs: &[&FileName]) {
    for sub in subs.iter() {
        fs::rename(sub.old, sub.new).unwrap();
    }
}

fn rename_subs(subs: &mut [String], p: LesParam) {
    let mut i = 1;
    for sub in subs.iter_mut() {
        let ext = detect_subtitle_extension(sub);
        let lang = detect_subtitle_lang(sub);
        let diry = format!("./{}/S{}/E{:0>2}/", p.tvdbid, p.season, i);
        make_missing_dirs(&diry);
        *sub = format!(
            "{}S{}.E{:0>2}.[{}]-[{}].{}.{}",
            diry, p.season, i, p.rel_group, p.track_name, lang, ext
        );
        i += 1;
    }
}

fn make_missing_dirs(directory: &str) {
    fs::create_dir_all(directory).unwrap();
}

fn detect_subtitle_extension(file_name: &str) -> &str {
    match file_name {
        x if x.contains(".ass") => "ass",
        x if x.contains(".srt") => "srt",
        _ => "ass",
    }
}

fn detect_subtitle_lang(file_name: &str) -> &str {
    match file_name {
        x if x.contains(".fre.") | x.contains(".fr.") | x.contains(".fra") => "fr-FR",
        x if x.contains(".eng.") | x.contains(".en.") => "en-US",
        _ => "und",
    }
}

fn test_continuity(episodes: &[String]) -> Result<(), io::Error> {
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

fn get_archive_files(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let paths = std::fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .filter_map(|path| {
            if path.extension().map_or(false, |ext| ext == "7z") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(paths)
}

fn extract_archives(archives: &[PathBuf]) -> Result<(), io::Error> {
    Ok(for archive in archives.iter() {
        extract_single_archive(archive.as_path());
    })
}

fn extract_single_archive(archive: &Path) {
    decompress_file(archive, "./extracted").expect("I hope it works mdr");
}
