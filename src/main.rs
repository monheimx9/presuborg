use std::{
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
    rename_subs(&mut files2, param);

    for f in files2.iter() {
        println!("{f}");
    }

    Ok(())
}

fn rename_subs(subs: &mut [String], param: LesParam) {
    let mut i = 1;
    for sub in subs.iter_mut() {
        *sub = format!(
            "S{}.E{:0>2}.[{}]-[{}]",
            param.season, i, param.rel_group, param.track_name
        );
        i += 1;
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
