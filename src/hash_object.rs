use anyhow::{ Context};
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{
    fs::{self, File},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    path::Path,
};
use crate::{git::get_wd, objects::BlobKind};
pub fn hash_object(write_to_objects:bool, file_path:String) -> anyhow::Result<String> {
    
    let path = Path::new(&file_path);
    let mut f = File::open(path).context("openning file")?;
    let size = f.metadata()?.size();
    if f.metadata()?.is_dir(){
        return Ok(hash_tree(write_to_objects ,  &mut f)?);
    }
    Ok(hash_file(write_to_objects, size ,&mut f)?)

}
pub fn hash_tree(write_to_objects: bool, f: &mut File) -> anyhow::Result<String> {

    Ok("yep".to_string())
}
pub fn hash_file(write_to_objects: bool,size:u64, f: &mut File) -> anyhow::Result<String> {
    let mut buf = Vec::new();

    //WHAT HAPPENS IF SIZE IS ACTUALLY BIGGER THAN USIZE????
    buf.resize(size as usize, 0);
    f.read_exact(&mut buf).context("reading file")?;

    //create a sha1 hasher and has the contents
    let mut hasher = Sha1::new();
    hasher.update(format!("{} {}\0",BlobKind::Blob ,size));
    hasher.update(&buf);
    let hashed_bytes = hasher.finalize();
    let sha = hex::encode(hashed_bytes);

    if write_to_objects {
        write_to_git(&sha, size, BlobKind::Blob, &mut buf).context("writing file to git/objects")?
    }

    Ok(sha)
}


fn write_to_git(sha : &String,size:u64,  kind: BlobKind, buf:&mut Vec<u8>) -> anyhow::Result<()>{

        //get the path of git/objects and create the new sha folder if it doesnt exist
        let wd = get_wd()?;
        let wd = wd + &format!("/.git/objects/{}", &sha[..2]);
        let path = Path::new(&wd);
        let _ = fs::create_dir(&path);

        //create blob path
        //if blob doesnt exist : create the blob and write compressed content to it
        let blob_hash = &format!("{}/{}", wd.clone(), &sha[2..]);
        let blob_hash_path = Path::new(&blob_hash);

        if !blob_hash_path.is_file() {
            let mut f = File::create(blob_hash_path).context("creating new blob")?;
            let mut z = ZlibEncoder::new(Vec::new(), Compression::default());
            z.write_all(format!("{kind} {}\0", size).as_bytes())?;
            z.write_all(&buf)?;
            let compressed = z.finish().context("compressing file")?;
            f.write_all(&compressed)?;
        }

        Ok(())



}
