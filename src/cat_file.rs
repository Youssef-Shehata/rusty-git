use std::io::{Read};

use anyhow::{bail, Context, Ok};

use crate::{ls_tree::ls_tree, objects::{BlobKind, Object}};

pub fn cat_file(pretty_print: bool, sha: &String) -> anyhow::Result<()> {
    let mut file_object = Object::read(sha)?;
    anyhow::ensure!(pretty_print , "please choose a valid option");
    let mut buf = Vec::new();
    match file_object.kind {
        BlobKind::Blob => {
            let _ = file_object.buffer.read_to_end(&mut buf).context("")?;
            let content = String::from_utf8_lossy(&buf);
            println!("{}", content.to_string());
        },
        BlobKind::Tree =>{

            ls_tree(false, &sha)?;

        },
        _ => bail!(format!("can't cat a {}", file_object.kind)),
    }

            Ok(())
}
