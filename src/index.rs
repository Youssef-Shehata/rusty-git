use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::mem;
use std::path::Path;

use anyhow::Context;

use crate::files::get_wd;
use crate::files::*;
pub fn git_status() -> anyhow::Result<()> {
    let wd = get_wd()?;
    let idx_path = &format!("{wd}/.git/index");
    let idx = Path::new(&idx_path);
    let f = File::open(idx)?;
    let mut reader = BufReader::new(f);

    let mut header_buffer = [0u8; 12];
    reader.read_exact(&mut header_buffer)?;
    println!("header: {:?}", String::from_utf8_lossy(&header_buffer));

    let n: [u8; 4] = header_buffer[8..].try_into().context("fuck this file")?;
    let num_i_entries = u32::from_be_bytes(n);

    println!("number of indwex entries: {}", num_i_entries);

    for _ in 0..num_i_entries {
        let mut garbage = [0u8; 62];
        reader.read_exact(&mut garbage)?;

        let mut name: Vec<u8> = Vec::new();
        reader.read_until(0, &mut name)?;
            name.truncate(name.len()-1 ); // Keep only elements before the null byte
        println!("file name : {:?}", String::from_utf8_lossy(&name));

        let mut garbage = [0u8; 7];
        reader.read_exact(&mut garbage)?;
    }

    Ok(())
}
pub fn git_add(args: &Vec<String>) -> anyhow::Result<()> {
    let wd = get_wd()?;
    if args[0] == ".".to_string() {
        let dir = Path::new(&wd);

        let files = collect_tracked_files_recursive(dir)?;
        for file in files.iter() {
            println!("{}", file.display());
        }
    }
    Ok(())
}
