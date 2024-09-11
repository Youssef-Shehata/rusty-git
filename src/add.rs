use std::path::Path;

use crate::files::get_wd;
use crate::files::*;
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
