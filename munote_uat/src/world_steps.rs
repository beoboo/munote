use std::string::String;

use anyhow::Result;
use cucumber::{codegen::anyhow, gherkin::Step, given, then, when};
use munote::{context::ContextPtr, score::Score};

use crate::MusicWorld;

#[given(expr = "\"{word}\" file with:")]
fn given_filename(w: &mut MusicWorld, step: &Step, file_name: String) {
    let content = step.docstring().unwrap();

    w.files.insert(file_name, content.clone());
}

#[when(expr = "I parse \"{word}\"")]
fn parse_filename(w: &mut MusicWorld, file_name: String) -> Result<()> {
    let content = &w.files[&file_name];

    let score = Score::parse(content.as_str(), ContextPtr::default())?;

    w.score = Some(score);

    Ok(())
}

#[then(expr = "there are {int} staff(s)")]
fn check_staff_count(w: &mut MusicWorld, num: usize) {
    assert_eq!(w.count_staffs(), num)
}
