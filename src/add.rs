use std::{
    fs::{self},
    path::{Path, PathBuf},
};

const IGNORED: &[&str] = &["target", ".git", ".gitignore", ".env"];
use crate::git::get_wd;
pub fn git_add(args: &Vec<String>) -> anyhow::Result<()> {
    let wd = get_wd()?;
    if args[0] == ".".to_string() {
        let dir = Path::new(&wd);

        let files = collect_tracked_files(dir)?;
        for file in files.iter() {
            println!("{}", file.display());
        }
    }
    Ok(())
}
fn collect_tracked_files(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && !IGNORED.contains(&path.file_name().unwrap().to_str().unwrap()) {
            files.extend(collect_tracked_files(&path)?);
        } else if path.is_file() && !IGNORED.contains(&path.file_name().unwrap().to_str().unwrap())
        {
            files.push(path);
        }
    }
    Ok(files)
}
