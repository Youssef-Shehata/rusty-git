mod git;
use git::Repo;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {

    #[command(subcommand)]
    command:Option<Commands>,
}
#[derive(Debug , Subcommand)]
enum Commands {
    Init{name_option:Option<String>},
    CatFile{
        #[clap(short = 'p')]
        pretty_print:bool ,
        sha:String
    },
    Add{files_option:Option<Vec<String>>},
}
fn main() -> Result<(), String> {
    
    let args = Args::parse();
    let cwd = std::env::current_dir().expect("failed to get cwd");
    let mut wd = cwd.to_string_lossy().into_owned();
    let mut repo = Repo::new(&wd).expect("error");
    match args.command{
        Some(Commands::Init{name_option} )=> {
            if let Some(name) = name_option {
                wd = wd + &format!("/{}", name);
            }
            repo = Repo::new(&wd).expect("error initialiazing repo");
        }
        Some(Commands::CatFile{pretty_print , sha} )=> {
            let res = repo.cat_file(pretty_print , &sha).unwrap();
            println!("{res}");
        }
        Some(Commands::Add{files_option} )=> {
            match files_option {
                Some(files)=>{
                    let _ = repo.git_add(&files);
                },
                None=>{
                    eprintln!("add what dumb motherfucker");
                }
            }
        }
        _ => return Err(format!("uknown option {:?}", args.command.unwrap())),
    }
    Ok(())
}
