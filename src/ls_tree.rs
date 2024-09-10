use std::io::{BufRead, Read};

use anyhow::{bail, Context};

use crate::objects::{BlobKind, Object};

pub fn ls_tree(names_only: bool, sha: &String) -> anyhow::Result<()> {
    let mut obj = Object::read(sha)?;

    match obj.kind {
        BlobKind::Tree => {
            loop {
                let mut mode_and_name = Vec::new();
                let mut hash = Vec::new();
                hash.resize(20, 0);
                let n= &obj.buffer
                    .read_until(0, &mut mode_and_name).context("error reading tree")?;
                if *n ==0 {break;}
                let mode_and_name= String::from_utf8_lossy(&mode_and_name).to_string();
                let Some((mode,name))= mode_and_name.split_once(" ") else {
                    panic!("error reading tree");
                };
                let _= &obj.buffer.read(&mut hash).expect("couldnt read tree");
                let hash = hex::encode(hash);
                let blob = Object::read(&hash).context("tree has a corrupted blob")?;
                

                println!("{mode} {} {}    {name}", blob.kind  , hash);
            }
            return Ok(());
        }
        _ => bail!(format!("obj {} isnt a tree.", obj.kind)),
    }
}
