use clap::Parser;

#[derive(Parser)]
#[command(name = "PreSubOrg")]
#[command(author = "Jeremy D. <jeremy.delley@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Rename subs for Sonarr Sub Manager", long_about = None)]
struct Cli {
    #[arg(long)]
    two: String,
    #[arg(long)]
    one: String,
}

fn get_args() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
}
