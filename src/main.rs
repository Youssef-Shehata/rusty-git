mod git;

use anyhow::{ bail};

use clap::{Parser, Subcommand};
use git::{cat_file, get_wd, git_add, init_repo};

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
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        sha: String,
    },
    Add {
        files_option: Option<Vec<String>>,
    },
    Wd
    ,
}
fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    match args.command {
        Some(Commands::Init { name_option }) => {
            init_repo(name_option)?;
        }
        Some(Commands::CatFile { pretty_print, sha }) => {
            let res = cat_file(pretty_print, &sha)?;
            println!("{res}");
        }
        Some(Commands::Add { files_option }) => match files_option {
            Some(files) => {
                let _ = git_add(&files)?;
            }
            None => {
                bail!("add what dumb motherfucker");
            }
        },
        Some(Commands::Wd)=>{
            get_wd()?;
        }
        None => bail!("uknown command"),
    }
    Ok(())
}
