use crate::{
    files::{get_wd, IGNORED},  hash_object, objects::BlobKind
};
use anyhow::{bail, Context};
use std::{
    fs::{self},
    io::Write,
    os::unix::fs::MetadataExt,
    path::Path,
};
#[derive(Debug)]
struct FileObject {
    mode: String,
    sha: String,
    name: String,
}
impl FileObject {
    fn new(mode: String, sha: String, name: String) -> Self {
        FileObject { mode, sha, name }
    }
}

pub fn write_tree(dir_name: &String) -> anyhow::Result<String> {
    let path = Path::new(dir_name);
    let mut files = Vec::new();
    for entry in fs::read_dir(path).expect("cant write-tree something that isnt a tree.") {
        let path = entry.context("invalid path")?.path();
        let Some(file_path) = path.to_str() else {
            bail!(format!("couldnt read file name at {}", path.display()));
        };
        
        let Some(file_name) = path.file_name() else {
            bail!(format!("couldnt read file name at {}", path.display()));
        };

        let file_name = file_name.to_string_lossy().to_string();
        if IGNORED.contains(&&file_name[..]) {
            continue;
        }

        if path.is_dir() {
            //TODO!!   RECURSE
            //hash_tree( &path)?;
        } else if path.is_file() {
            let hash = hash_object(false, BlobKind::Blob, &file_path.to_string())?;
            let mode = path
                .metadata()
                .context("failed to read metadata of file")?
                .mode()
                .to_string();
            files.push(FileObject::new(mode, hash, file_name.to_string()));
        }
    }

    let wd = get_wd()?;
    let tmp_path = format!("{wd}/.git/tmp");
    let mut f = fs::File::create(&tmp_path).context("writing temp")?;
    for file in files.iter() {
        f.write(format!("{} {}\0", file.mode, file.name).as_bytes())?;
        f.write(&hex::decode(file.sha.clone())?)?;
    }
    let Ok(sha) = hash_object(true, BlobKind::Tree, &tmp_path) else {
        bail!("caouldnt hash tree");
    };
    fs::remove_file(tmp_path)?;

    Ok(sha)
}
