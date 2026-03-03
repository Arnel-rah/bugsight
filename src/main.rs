use clap::Parser;
use colored::*;

#[derive(Parser)]
#[command(name = "bugsight")]
#[command(about = "Debug smarter, not harder ")]
struct Cli {
    #[arg(short, long)]
    file: Option<String>,
    #[arg(short, long)]
    explain: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(error) = cli.explain {
        println!("{} {}", " Analyzing:".yellow(), error);
    } else if let Some(file) = cli.file {
        println!("{} {}", " Reading file:".cyan(), file);
    } else {
        println!("{}", " Run with --help to see options".green());
    }
}