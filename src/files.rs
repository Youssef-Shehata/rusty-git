use std::{fs, path::{Path, PathBuf}};

use anyhow::anyhow;



// temporarily to limit the outpu files when testing and developing features
pub const IGNORED: [&'static str; 4] = ["target", ".git", ".gitignore", ".env"];

pub fn collect_tracked_files_recursive(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && !IGNORED.contains(&path.file_name().unwrap().to_str().unwrap()) {
            files.extend(collect_tracked_files_recursive(&path)?);
        } else if path.is_file() && !IGNORED.contains(&path.file_name().unwrap().to_str().unwrap())
        {
            files.push(path);
        }
    }
    Ok(files)
}

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
