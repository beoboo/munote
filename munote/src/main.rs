use std::{fs, path::Path};

use anyhow::Result;
use clap::Parser;

use munote::{context::ContextPtr, score::Score};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file_name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let path = Path::new(&args.file_name);
    let display = path.display();
    println!("Parsing {}", display);

    let content = fs::read_to_string(args.file_name)?;
    let score = Score::parse(&content, ContextPtr::default())?;

    println!("Score parsed successfully: ");
    println!("{score:?}");

    Ok(())
}
