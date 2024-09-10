mod git;
use add::git_add;
use anyhow::bail;
use clap::{Args, Parser, Subcommand};
use git::init_repo;
use ls_tree::ls_tree;
mod hash_object;
use crate::hash_object::*;
mod cat_file;
use crate::cat_file::*;
mod add;
mod ls_tree;
mod objects;
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
    HashFile {
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
    #[arg(short, long, default_value_t = false)]
    show_size: bool,

    #[clap(short = 't')]
    #[arg(short, long, default_value_t = false)]
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
            }
            if option.show_size {
                cat_file(Some(CatOptions::ShowSize), &sha)?;
            }
            if option.show_type {
                cat_file(Some(CatOptions::ShowType), &sha)?;
            }

            Ok(())
        }
        Some(Commands::LsTree { option, sha }) => {
            if option.only_trees {
                ls_tree(Some(TreeOptions::OnlyTrees), &sha)?;
            } if option.show_size {
                ls_tree(Some(TreeOptions::ShowSize), &sha)?;
            } if option.recurse {
                ls_tree(Some(TreeOptions::Recurse), &sha)?;
            } if option.name_only {
                ls_tree(Some(TreeOptions::NamesOnly), &sha)?;
            }

            Ok(())
        }
        Some(Commands::Add { files_option }) => match files_option {
            Some(files) => {
                let _ = git_add(&files)?;
                Ok(())
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
            Ok(())
        }
        None => bail!("uknown command"),
    }
}
