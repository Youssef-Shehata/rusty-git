use crate::{
    files::{get_wd, IGNORED},
    hash_object,
    objects::BlobKind,
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
    mode: u32,
    sha: String,
    name: String,
}
impl FileObject {
    fn new(mode: u32, sha: String, name: String) -> Self {
        FileObject { mode, sha, name }
    }
}

pub fn write_tree() -> anyhow::Result<String> {
    let wd = get_wd()?;
    let path = Path::new(&wd);
    let hash = hash_tree(path)?;
    Ok(hash)
}
pub fn hash_tree(path : &Path) -> anyhow::Result<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path).expect("cant write-tree something that isnt a tree.") {
        let path = entry.context("invalid path")?.path();
        let file_path = path
            .to_str()
            .expect(&format!("couldn't read file at : {}", path.display()));
        let file_name = path
            .file_name()
            .expect(&format!("couldn't read file at : {}", path.display()));

        let file_name = file_name.to_string_lossy().to_string();
        if IGNORED.contains(&&file_name[..]) {
            continue;
        }


        if path.is_dir() {
            let hash = hash_tree( &path)?;
            let mode = path
                .metadata()
                .context("failed to read metadata of file")?
                .mode();
            files.push(FileObject::new(mode, hash, file_name.to_string()));
        } else if path.is_file() {
            let hash = hash_object(true, BlobKind::Blob, &file_path.to_string())?;
            let mode = path
                .metadata()
                .context("failed to read metadata of file")?
                .mode();
            files.push(FileObject::new(mode, hash, file_name.to_string()));
        }
    }

    let wd = get_wd()?;
    let tmp_path = format!("{wd}/.git/tmp");
    let mut f = fs::File::create(&tmp_path).context("writing temp")?;
    for file in files.iter() {
        f.write(format!("{:o} {}\0", file.mode, file.name).as_bytes())?;
        println!("writing file which mode is : {}", file.mode);
        f.write(&hex::decode(file.sha.clone())?)?;
    }
    let Ok(sha) = hash_object(true, BlobKind::Tree, &tmp_path) else {
        bail!("caouldnt hash tree");
    };
    fs::remove_file(tmp_path)?;

    Ok(sha)
}
