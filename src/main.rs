mod git;

use add::git_add;
use anyhow::bail;

use clap::{Parser, Subcommand};
use git::init_repo;
use ls_tree::ls_tree;
mod hash_object;
use crate::hash_object::*;
mod cat_file;
use crate::cat_file::*;
mod add;
mod objects;
mod ls_tree;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Debug, Subcommand)]
enum Commands {
    Init {
        name_option: Option<String>,
    },
    LsTree{
    #[arg(short, long, default_value_t = false)]
        names_only: bool,
        sha: String,
    },
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        sha: String,
    },
    HashFile {
        #[clap(short = 'w')]
        write_to_objects: bool,

        file_name: String,
    },
    Add {
        files_option: Option<Vec<String>>,
    },
}
fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    match args.command {
        Some(Commands::Init { name_option }) => {
            init_repo(name_option)?;
        }
        Some(Commands::CatFile { pretty_print, sha }) => {
            cat_file(pretty_print, &sha)?;
        }
        Some(Commands::LsTree{ names_only, sha }) => {
           ls_tree(names_only, &sha)?;
        }
        Some(Commands::Add { files_option }) => match files_option {
            Some(files) => {
                let _ = git_add(&files)?;
            }
            None => {
                bail!("add what dumb motherfucker");
            }
        },
        Some(Commands::HashFile {
            write_to_objects,
            file_name,
        }) => {
            let hash = hash_file(write_to_objects, file_name)?;
            println!("{hash}");
        }
        None => bail!("uknown command"),
    }
    Ok(())
}
