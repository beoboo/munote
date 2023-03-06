use std::{fs, path::Path};

use anyhow::Result;
use clap::Parser;
use colorize::AnsiColor;

use munote::{context::ContextPtr, score::Score};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let path = Path::new(&args.path);

    if path.is_dir() {
        let dir = fs::read_dir(path)?;
        for file in dir {
            parse_score(&file.unwrap().path())?;
        }
    } else {
        parse_score(path)?;
    }

    Ok(())
}

fn parse_score(path: &Path) -> Result<Score> {
    let display = path.display();
    println!("Parsing: {}...\n", display);

    let content = fs::read_to_string(path)?;
    let score = Score::parse(&content, ContextPtr::default())?;

    println!(
        "{}",
        format!("Score \"{display}\" parsed successfully!\n").green()
    );
    // println!("{}", format!("{score:?}").b_black());

    Ok(score)
}
