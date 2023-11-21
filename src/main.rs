use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use regex::{self, Regex};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = Path::new("/home/monheim/Downloads/Bucchigire/French/");

    let param = LesParam {
        rel_group: "Erai-raws",
        tvdbid: "418183",
        season: "01",
        track_name: "Français (France)",
    };

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
    test_continuity(&files2)?;
    let mut file_renamed = files2.clone();
    rename_subs(&mut file_renamed, param);

    let mut episodes: HashMap<u8, FileName> = HashMap::new();

    for (index, file) in files2.iter().enumerate() {
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

    for f in files2.iter() {
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
