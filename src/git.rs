use anyhow::{anyhow, bail, Context};
use flate2::bufread::ZlibDecoder;
use std::{
    fs::{self, DirEntry},
    io::{self, Read, Write},
};

use std::path::Path;
pub fn get_wd() -> anyhow::Result<String> {
    let cwd = std::env::current_dir().expect("failed to get cwd");
    let wd = cwd.to_string_lossy().into_owned();
    let wd_path = Path::new(&wd);
    let mut curr = wd_path;
    loop {
        if let None = curr.parent() {
            break;
        }
        let name = curr.to_str().unwrap().to_string() + "/.git";
        let target = Path::new(&name);
        if target.exists() && target.is_dir() {
            return Ok(target
                .to_str()
                .unwrap()
                .trim_end_matches("/.git")
                .to_string());
        }
        curr = curr.parent().unwrap();
    }
    return Err(anyhow!("fatal: Not a git repository"));
}
pub fn init_repo(name: Option<String>) -> anyhow::Result<()> {
    let cwd = std::env::current_dir().expect("failed to get cwd");
    let mut wd = cwd.to_string_lossy().into_owned();

    if let Some(dir_name) = name {
        if dir_name != "." {
            wd = wd + &dir_name;
        }
    }

    if let Ok(()) = assert_is_repo(&wd) {
        println!("reinitializing repo");
        return Ok(());
    }
    let git_path = wd + "/.git";
    fs::create_dir_all(format!("{}/refs", git_path))?;
    fs::create_dir_all(format!("{}/objects", git_path))?;
    let mut f = fs::File::create_new(format!("{}/HEAD", git_path))?;
    f.write_all("ref: refs/head/main\n".as_bytes())?;
    println!("Initialized empty git repository in {}", git_path);
    Ok(())
}

pub fn assert_is_repo(wd: &String) -> anyhow::Result<()> {
    let head_path = wd.to_owned() + "/.git/HEAD";
    let mut f = std::fs::File::open(&head_path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    if content.starts_with("ref: refs/head") {
        return Ok(());
    }
    bail!("not a repo");
}

pub fn git_add(args: &Vec<String>) -> anyhow::Result<()> {
    let wd = get_wd()?;
    if args[0] == ".".to_string() {
        println!("working dir : {}", wd);
        let dir = Path::new(&wd);
        map_dir_files(dir, &call_back).expect("manga");
    }
    Ok(())
}
pub fn cat_file(pretty_print: bool, sha: &String) -> anyhow::Result<String> {
    if sha.len() < 4 {
        bail!("minimum 4 letters needed for the sha");
    }
    let wd = get_wd()?;
    let git_path = wd + "/.git";

    let objects_path = git_path + &format!("/objects/{}/", &sha[..2]);
    println!("path {objects_path}");
    let compressed_text =
        find_blob(&objects_path, &sha[2..].to_string()).context("blob not found")?;

    if pretty_print {
        let mut s = String::new();
        let mut d = ZlibDecoder::new(&compressed_text[..]);
        d.read_to_string(&mut s)
            .context("error reading blob content")?;
        return Ok(s);
    }

    bail!("not a valid option")
}

fn find_blob(path: &String, sha: &String) -> anyhow::Result<Vec<u8>> {
    let dir = Path::new(path);
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file()
                && path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
                    .starts_with(sha)
            {
                files.push(path);
                if files.len() > 1 {
                    bail!("multiple objects found , provide a longer sha");
                }
            }
        }
    }
    if files.is_empty() {
        bail!("couldnt find blob");
    }

    let mut f = fs::File::open(&files[0]).context("corrupted blob")?;
    let mut encoded_bytes = Vec::new();
    f.read_to_end(&mut encoded_bytes)
        .context("error reading blob")?;
    return Ok(encoded_bytes);
}

fn map_dir_files(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                map_dir_files(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn call_back(dir: &DirEntry) {
    println!("{:?}", dir);
}
