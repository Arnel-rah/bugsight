mod parsers;
mod analyzer;

use clap::Parser;
use colored::*;

#[derive(Parser)]
#[command(name = "bugsight")]
#[command(about = "Debug smarter, not harder 🔍")]
struct Cli {
    #[arg(short, long)]
    explain: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(error) = cli.explain {
        println!("{} {}\n", " Analyzing:".yellow(), error);

        match analyzer::analyze(&error) {  
            Some(result) => {
                println!("{} {}", "Type:".bold(), result.error_type.red());
                println!("{} {}", "Message:".bold(), result.message);
                println!("{} {}", "Suggestion:".green().bold(), result.suggestion);
            }
            None => {
                println!("{}", "No pattern matched. Try --help.".dimmed());
            }
        }
    }
}