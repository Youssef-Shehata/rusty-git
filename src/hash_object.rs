use anyhow::{anyhow, bail, Context};
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{
    fs::{self, File},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    path::Path,
};
use crate::git::get_wd;
pub fn hash_file(write_to_objects: bool, file_path: String) -> anyhow::Result<String> {
    //read the the file contents
    let file_path = Path::new(&file_path);
    let mut f = fs::File::open(file_path).context("openning file")?;
    let size = f.metadata()?.size();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).context("reading file")?;

    //create a sha1 hasher and has the contents
    let mut hasher = Sha1::new();
    hasher.update(format!("blob {}\0", size));
    hasher.update(&buf);
    let hashed_bytes = hasher.finalize();
    let sha = hex::encode(hashed_bytes);

    if write_to_objects {
        //get the path of git/objects and create the new sha folder if it doesnt exist
        let wd = get_wd()?;
        let wd = wd + &format!("/.git/objects/{}", &sha[..2]);
        let path = Path::new(&wd);
        let _ = fs::create_dir(&path);

        //create blob path
        //if blob doesnt exist : create the blob and write compressed content to it
        let blob_hash = &format!("{}/{}", wd.clone(), &sha[2..]);
        let blob_hash_path = Path::new(&blob_hash);
        if blob_hash_path.is_file() {
            return Ok(sha);
        } else {
            let mut f = File::create(blob_hash_path).context("creating new blob")?;
            let mut z = ZlibEncoder::new(Vec::new(), Compression::default());
            z.write_all(format!("blob {}\0", size).as_bytes())?;
            z.write_all(&buf)?;
            let compressed = z.finish().context("compressing file")?;
            f.write_all(&compressed)?;
        }
    }
    Ok(sha)
}
