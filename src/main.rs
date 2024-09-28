mod git;
use index::git_add;
use anyhow::bail;
use clap::{Args, Parser, Subcommand};
use commit::commit_tree;
use git::init_repo;
use index::git_status;
use ls_tree::ls_tree;
use write_tree::write_tree;
mod hash_object;
use crate::hash_object::*;
mod cat_file;
use crate::cat_file::*;
mod index;
mod files;
mod ls_tree;
mod objects;
mod write_tree;
mod commit;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct MyArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Debug, Subcommand)]
enum Commands {
    Init {
        name_option: Option<String>,
    },
    LsTree {
        #[command(flatten)]
        option: LsTreeOptions,

        sha: String,
    },
    CatFile {
        #[command(flatten)]
        option: CatFileOptions,
        sha: String,
    },
    WriteTree,
    Status,
    CommitTree{

        #[clap(short = 'm')]
        message: String,

    }
    ,
    HashObject {
        #[clap(short = 'w')]
        write_to_objects: bool,

        file_name: String,
    },
    Add {
        files_option: Option<Vec<String>>,
    },
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct LsTreeOptions {
    #[clap(short = 'd')]
    #[arg(long, default_value_t = false)]
    only_trees: bool,

    #[arg(short, long, default_value_t = false)]
    recurse: bool,

    #[arg(short, long, default_value_t = false)]
    show_size: bool,

    #[arg(long, default_value_t = false)]
    name_only: bool,
}
enum TreeOptions {
    OnlyTrees,
    Recurse,
    ShowSize,
    NamesOnly,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct CatFileOptions {
    #[clap(short = 'p')]
    #[arg(long, default_value_t = false)]
    pretty_print: bool,

    #[clap(short = 's')]
    #[arg(long, default_value_t = false)]
    show_size: bool,

    #[clap(short = 't')]
    #[arg(long, default_value_t = false)]
    show_type: bool,
}
enum CatOptions {
    PrettyPrint,
    ShowType,
    ShowSize,
}

fn main() -> Result<(), anyhow::Error> {
    let args = MyArgs::parse();
    match args.command {
        Some(Commands::Init { name_option }) => {
            init_repo(name_option)?;
            Ok(())
        }
        Some(Commands::CatFile { option, sha }) => {
            if option.pretty_print {
                cat_file(Some(CatOptions::PrettyPrint), &sha)?;
            return Ok(());
            }
            if option.show_size {
                cat_file(Some(CatOptions::ShowSize), &sha)?;
            return Ok(());
            }
            if option.show_type {
                cat_file(Some(CatOptions::ShowType), &sha)?;
            return Ok(());
            }

            Ok(())
        }
        Some(Commands::LsTree { option, sha }) => {
            if option.only_trees {
                ls_tree(Some(TreeOptions::OnlyTrees), &sha)?;
                return Ok(());
            }
            if option.show_size {
                ls_tree(Some(TreeOptions::ShowSize), &sha)?;
                return Ok(());
            }
            if option.recurse {
                ls_tree(Some(TreeOptions::Recurse), &sha)?;
                return Ok(());
            }
            if option.name_only {
                ls_tree(Some(TreeOptions::NamesOnly), &sha)?;
                return Ok(());
            }

            ls_tree(None, &sha)?;
            Ok(())
        }
        Some(Commands::Status) => {

            let res = git_status();
            match res {
                Ok(())=> {},
                Err(e)=> eprint!("{e}"),
                
            };
            Ok(())
            
        },

        Some(Commands::Add { files_option }) => match files_option {
            
            Some(files) => {
                let _ = git_add(&files)?;
                Ok(())
            }
            None => {
                bail!("add what dumb motherfucker");
            }
        },
        Some(Commands::HashObject {
            write_to_objects,
            file_name,
        }) => {
            let hash = hash_object(write_to_objects,objects::BlobKind::Blob, &file_name)?;
            println!("{hash}");
            Ok(())
        }
        Some(Commands::WriteTree) => {
            let hash = write_tree()?;
            println!("{hash}");
            Ok(())
        }
        Some(Commands::CommitTree{message}) => {
            let hash = commit_tree(message)?;
            println!("{hash}");
            Ok(())
        }
        None => bail!("uknown command"),
    }
}
