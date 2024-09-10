use std::io::{BufRead, Read};
use anyhow::Context;
use crate::{
    objects::{BlobKind, Object},
    TreeOptions,
};

pub fn ls_tree(tree_options: Option<TreeOptions>, sha: &String) -> anyhow::Result<()> {
    let mut obj = Object::read(sha)?;

    anyhow::ensure!(
        matches!(obj.kind, BlobKind::Tree),
        format!("obj {} isnt a tree.", obj.kind)
    );

    loop {
        let mut mode_and_name = Vec::new();
        let mut hash = Vec::new();
        hash.resize(20, 0);
        let n = &obj
            .buffer
            .read_until(0, &mut mode_and_name)
            .context("error reading tree")?;

        if *n == 0 {
            break;
        }

        let mode_and_name = String::from_utf8_lossy(&mode_and_name).to_string();
        let Some((mode, name)) = mode_and_name.split_once(" ") else {
            panic!("error reading tree");
        };

        let _ = &obj.buffer.read(&mut hash).expect("couldnt read tree");
        let hash = hex::encode(hash);
        let blob = Object::read(&hash).context("tree has a corrupted blob")?;

        match tree_options {
            Some(ref option) => match option {
                TreeOptions::ShowSize  => println!("{mode} {} {} {}    {name}",blob.kind,hash,blob.size),
                TreeOptions::NamesOnly => println!("{name}"),
                TreeOptions::OnlyTrees => {
                    if matches!(blob.kind, BlobKind::Tree) {
                        println!("{mode} {} {}    {name}", blob.kind, hash);
                    }
                }

                TreeOptions::Recurse => {
                    if matches!(blob.kind, BlobKind::Tree) {
                        ls_tree(Some(TreeOptions::Recurse), &hash)?;
                    } else {
                        println!("{mode} {} {}    {name}", blob.kind, hash);
                    }
                }
            },
            None => println!("{mode} {} {}    {name}", blob.kind, hash),
        }
    }
    return Ok(());
}
