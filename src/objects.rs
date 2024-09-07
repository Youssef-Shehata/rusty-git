use crate::git::get_wd;

use anyhow::{anyhow, bail, Context};
use flate2::bufread::ZlibDecoder;
use std::{
    error::Error,
    ffi::CStr,
    fmt::{format, write, Display},
    fs::{self},
    io::{BufRead, BufReader, Cursor, Read, Write},
    path::Path,
};

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

pub struct Object {
    kind: BlobKind,
    content: Vec<u8>,
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();
        match self.kind {
            BlobKind::Blob => {
                stdout
                    .write_all(&self.content)
                    .expect("couldn't write blob to stdout");
                return Ok(());
            }
            BlobKind::Commit => Ok(()),
            BlobKind::Tree => {
                let mut mode = Vec::new();
                let mut hash= Vec::new();
                let mut buf = BufReader::new(Cursor::new( &self.content));
                buf.read_until(0,&mut mode).expect("couldnt read tree");
                buf.read_until(0,&mut hash).expect("couldnt read tree");
                write!(f , "{}" , String::from_utf8_lossy(&mode))?;
                write!(f , "{:?}" ,hash.len())?;







                Ok(())
            },

        }
    }
}
impl Object {
    pub fn read(sha: &String) -> anyhow::Result<Object> {
        if sha.len() < 4 {
            bail!("minimum 4 letters needed for the sha");
        }
        let compressed_text = find_blob(sha).context("blob not found")?;

        let mut buff = Vec::new();
        let z_lib = ZlibDecoder::new(&compressed_text[..]);
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
        let size = size.parse::<usize>().context("couldn't read blob size")?;

        buff.clear();
        buff.resize(size, 0);
        buff_reader
            .read_exact(&mut buff)
            .context("failed to read blob contents")?;

        let n = buff_reader.read(&mut [0]).context("")?;

        if n != 0 {
            bail!(
            "size of blob exceeded expectations , expected {size} bytes, found {n} trailing bytes."
        );
        }

        Ok(Object {
            kind,
            content: buff,
        })
    }
}

fn find_blob(sha: &String) -> anyhow::Result<Vec<u8>> {
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

    let mut f = fs::File::open(&files[0]).context("corrupted blob")?;
    let mut encoded_bytes = Vec::new();
    f.read_to_end(&mut encoded_bytes)
        .context("error reading blob")?;
    return Ok(encoded_bytes);
}
