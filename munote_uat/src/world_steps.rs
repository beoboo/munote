use std::string::String;

use cucumber::{given, when, then};

use crate::MusicWorld;

#[given(expr = "\"{word}\" file with:")]
fn given_filename(w: &mut MusicWorld, file_name: String) {}

#[when(expr = "I parse \"{word}\"")]
fn parse_filename(w: &mut MusicWorld, file_name: String) {}

#[then(expr = "there are {int} staff(s)")]
fn check_staff_count(w: &mut MusicWorld, num: usize) {
    assert_eq!(w.count_staffs(), num)
}
