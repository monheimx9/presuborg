use clap::Parser;

#[derive(Parser)]
#[command(name = "PreSubOrg")]
#[command(author = "Jeremy D. <jeremy.delley@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Rename subs for Sonarr Sub Manager", long_about = None)]
pub struct Cli {
    #[arg(long, short = 'R')]
    release_group: String,
    #[arg(long)]
    season: String,
}

pub fn get_args() -> Cli {
    Cli::parse()
}
