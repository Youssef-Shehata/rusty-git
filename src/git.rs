use anyhow::{ bail};
use std::{
    fs::{self},
    io::{Read, Write},
};

// temporarily to limit the outpu files when testing and developing features

pub fn init_repo(name: Option<String>) -> anyhow::Result<()> {
    let cwd = std::env::current_dir().expect("failed to get cwd");
    let mut wd = cwd.to_string_lossy().into_owned();

    if let Some(dir_name) = name {
        if dir_name != "." {
            wd = wd + &dir_name;
        }
    }

    if let Ok(()) = assert_wd_is_repo(&wd) {
        println!("reinitializing repo");
        return Ok(());
    }
    let git_path = wd + "/.git";
    fs::create_dir_all(format!("{}/refs/heads", git_path))?;
    fs::create_dir_all(format!("{}/objects", git_path))?;
    let mut f = fs::File::create_new(format!("{}/HEAD", git_path))?;
    f.write_all("ref: refs/head/main\n".as_bytes())?;
    println!("Initialized empty git repository in {}", git_path);
    Ok(())
}

pub fn assert_wd_is_repo(wd: &String) -> anyhow::Result<()> {
    let head_path = wd.to_owned() + "/.git/HEAD";
    let mut f = std::fs::File::open(&head_path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    if content.starts_with("ref: refs/head") {
        return Ok(());
    }
    bail!("not a repo");
}
