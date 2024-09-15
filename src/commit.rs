use crate::{files::get_wd, hash_object, objects::BlobKind, write_tree::hash_tree};
use anyhow::{bail, Context};
use chrono::{DateTime, Local};
use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

//little hard coding wouldnt hurt noone
static COMMITTER: &str = "sh7toot";
static COMMITTER_EMAIL: &str = "sh7toot@github.com";
static AUTHOR: &str = "sh7toot";
static AUTHOR_EMAIL: &str = "sh7toot@github.com";
static DEFAULT_BRANCH: &str = "main";

pub fn commit_tree(message: String) -> anyhow::Result<String> {
    //date and timezone
    let time: DateTime<Local> = Local::now();
    let author_date_seconds = time.timestamp();
    let author_date_timezone = time.offset();
    let committer_date_seconds = time.timestamp();
    let committer_date_timezone = time.offset();

    let wd = get_wd()?;
    let wd_path = Path::new(&wd);
    let tree = hash_tree(wd_path)?;

    //tmp file to write the commit to , hash it , then delete the file..
    let parent_path = format!("{wd}/.git/refs/heads/{DEFAULT_BRANCH}");
    let parent_path = Path::new(&parent_path);
    let tmp_path = format!("{wd}/.git/tmp_comm");

    let mut tmp_file = fs::File::create(&tmp_path).context("writing temp")?;

    tmp_file.write_all(format!("tree {}\n", tree).as_bytes())?;

    if let Ok(mut parent_file) = fs::File::open(parent_path) {
        let mut parent = String::new();
        parent_file.read_to_string(&mut parent)?;
        tmp_file.write_all(format!("{}\n", parent).as_bytes())?;
    }

    tmp_file.write_all(format!("author {AUTHOR} <{AUTHOR_EMAIL}> {author_date_seconds} {author_date_timezone}\n").as_bytes())?;
    tmp_file.write_all(format!("committer {COMMITTER} <{COMMITTER_EMAIL}> {committer_date_seconds} {committer_date_timezone}\n\n").as_bytes())?;
    tmp_file.write_all(format!("{}", message).as_bytes())?;

    let Ok(sha) = hash_object(true, BlobKind::Commit, &tmp_path) else {
        bail!("couldnt hash tree");
    };
    let mut parent_file = fs::File::create(parent_path)?;
    parent_file
        .write_all(sha.as_bytes())
        .context(format!("writing commit to refs/{DEFAULT_BRANCH}"))?;
    fs::remove_file(tmp_path)?;

    Ok(sha)
}
