use clap::Parser;

#[derive(Parser)]
#[command(name = "PreSubOrg")]
#[command(author = "Jeremy D. <jeremy.delley@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Rename subs for Sonarr Sub Manager", long_about = None)]
pub struct Cli {
    #[arg(long, short = 'R', default_value = "Erai-raws")]
    release_group: String,
    #[arg(long, short, default_value = "S01")]
    season: String,
    #[arg(long, short, default_value = "423800")]
    tvdb: String,
}

pub fn get_args() -> Cli {
    Cli::parse()
}
