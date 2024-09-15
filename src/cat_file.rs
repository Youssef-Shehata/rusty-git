use std::io::Read;

use anyhow::{bail, Context, Ok};

use crate::{
    ls_tree::ls_tree,
    objects::{BlobKind, Object},
    CatOptions,
};

pub fn cat_file(option: Option<CatOptions>, sha: &String) -> anyhow::Result<()> {
    if option.is_none() {
        bail!("please provide a valid option");
    };
    let option = option.unwrap();
    let mut obj = Object::read(sha)?;
    let mut buf = Vec::new();

    let size = obj
        .size
        .parse::<usize>()
        .context("failed to read blob size")?;
    buf.resize(size, 0);
    let size = obj
        .size
        .parse::<usize>()
        .context("failed to read blob size")?;
    buf.resize(size, 0);
    obj.buffer.read_exact(&mut buf).context("")?;
    let content = String::from_utf8_lossy(&buf);

    match obj.kind {
        BlobKind::Blob => match option {
            CatOptions::PrettyPrint => println!("{}", content.to_string()),
            CatOptions::ShowType => println!("tree"),
            CatOptions::ShowSize => println!("{}", size),
        },
        BlobKind::Commit => match option {
            CatOptions::PrettyPrint => println!("{}", content.to_string()),
            CatOptions::ShowType => println!("commit"),
            CatOptions::ShowSize => println!("{}", size),
        },

        BlobKind::Tree => match option {
            CatOptions::PrettyPrint => ls_tree(None, &sha)?,
            CatOptions::ShowType => println!("tree"),
            CatOptions::ShowSize => println!("{}", size),
        },

    }

    Ok(())
}
