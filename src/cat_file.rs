use std::io::Read;

use anyhow::{bail, Context, Ok};

use crate::{
    ls_tree::ls_tree,
    objects::{BlobKind, Object},
    CatOptions,
};

pub fn cat_file(option: Option<CatOptions>, sha: &String) -> anyhow::Result<()> {
    let mut obj = Object::read(sha)?;
    let mut buf = Vec::new();

    let size = obj.size.parse::<usize>().context("failed to read blob size")?;
    buf.resize(size, 0);
    println!("obj :{:?}", obj.size );
    let size = obj
        .size
        .parse::<usize>()
        .context("failed to read blob size")?;
    buf.resize(size, 0);

    match obj.kind {
        BlobKind::Blob => {
            let _ = obj.buffer.read_exact(&mut buf).context("")?;
            let content = String::from_utf8_lossy(&buf);

            match option {
                Some(op) => match op {
                    CatOptions::PrettyPrint => println!("{}", content.to_string()),
                    CatOptions::ShowType => println!("{}", obj.kind),
                    CatOptions::ShowSize => println!("{}", size),
                },
                None => bail!("please provide a valid option"),
            }
        }

        BlobKind::Tree => ls_tree(None, &sha)?,

        _ => bail!(format!("can't cat a {}", obj.kind)),
    }

    Ok(())
}
