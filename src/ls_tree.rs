use crate::{
    objects::{BlobKind, Object},
    TreeOptions,
};
use anyhow::{bail, Context};
use std::io::{BufRead, Read};

fn check_file_type(mode: u32) -> anyhow::Result<BlobKind> {
    let tree_mode = format!("{mode}");
    let tree_mode = u32::from_str_radix(&tree_mode, 8)?;
    if ( tree_mode & 0o170000) == 0o40000 {
        return Ok(BlobKind::Tree);
    } else if (mode & 0o170000) == 0o100000 {
        return Ok(BlobKind::Blob);
    } else if (mode & 0o170000) == 0o120000 {
        return Ok(BlobKind::Tree);
    }
    bail!("weird ass file")
}
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

        let mode = mode.parse::<u32>()?;
        let kind = check_file_type(mode )?;

        match tree_options {
            Some(ref option) => match option {
                TreeOptions::ShowSize => {
                    let blob = Object::read(&hash).context("tree has a corrupt file")?;
                    println!("{mode:o} {} {} {}    {name}", kind, hash, blob.size);
                }
                TreeOptions::NamesOnly => println!("{name}"),
                TreeOptions::OnlyTrees => {
                    if matches!(kind, BlobKind::Tree) {
                        println!("{mode:o} {} {}    {name}", kind, hash);
                    }
                }

                TreeOptions::Recurse => {
                    if matches!(kind, BlobKind::Tree) {
                        ls_tree(Some(TreeOptions::Recurse), &hash)?;
                    } else {
                        println!("{mode:o} {} {}    {name}", kind, hash);
                    }
                }
            },
            None => println!("{mode:o} {} {}    {name}", kind, hash),
        }
    }
    return Ok(());
}
