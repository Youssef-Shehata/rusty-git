use crate::files::get_wd;

use anyhow::{bail, Context};
use flate2::read::ZlibDecoder;
use std::{
    ffi::CStr,
    fmt::Display,
    fs::{self},
    io::{BufRead, BufReader, Read},
    path::Path,
};

#[derive(Debug)]
pub enum BlobKind {
    Blob,
    Commit,
    Tree,
}

impl Display for BlobKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlobKind::Blob => write!(f, "blob"),
            BlobKind::Tree => write!(f, "tree"),
            BlobKind::Commit => write!(f, "commit"),
        }
    }
}

#[derive(Debug)]
pub struct Object<R> {
    pub size:String,
    pub kind: BlobKind,
    pub buffer: R,
}
impl Object<()> {
    pub fn read(sha: &String) -> anyhow::Result<Object<impl BufRead>> {
        if sha.len() < 4 {
            bail!("minimum 4 letters needed for the sha");
        }
        let comp_file = find_blob(sha).context("blob not found")?;

        let mut buff = Vec::new();
        let z_lib = ZlibDecoder::new(comp_file);
        let mut buff_reader = BufReader::new(z_lib);

        //read the  header of the blob :blob <size>/0<content>
        buff_reader
            .read_until(0, &mut buff)
            .context("couldnt read blob")?;

        let header = CStr::from_bytes_until_nul(&buff).context("blob header is corruupted")?;
        let header = header
            .to_str()
            .context("Blob has invalid characters , make sure its all UTF-8")?;

        let Some((kind, size)) = header.split_once(" ") else {
            bail!("invalid header of blob file")
        };
        let kind = match kind {
            "blob" => BlobKind::Blob,
            "commit" => BlobKind::Commit,
            "tree" => BlobKind::Tree,
            _ => bail!("tf is a {kind}"),
        };

        let size_num = size.parse::<usize>().context("couldn't read blob size")?;
        let buffer = buff_reader.take(size_num as u64);
        Ok(Object {
            kind,
            size: size.to_string(),
            buffer,
        })
    }

    //buff.resize(size, 0);
}

fn find_blob(sha: &String) -> anyhow::Result<std::fs::File> {
    let wd = get_wd()?;
    let blob_folder = &format!("{}/.git/objects/{}/", wd, &sha[..2]);
    let blob_folder_path = Path::new(&blob_folder);
    let mut files = Vec::new();
    if blob_folder_path.is_dir() {
        for entry in fs::read_dir(blob_folder_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file()
                && path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
                    .starts_with(&sha[2..])
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

    let f = fs::File::open(&files[0]).context("corrupted blob")?;
    return Ok(f);
}
