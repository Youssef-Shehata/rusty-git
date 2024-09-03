use flate2::bufread::ZlibDecoder;
use std::{
    fs::{self, DirEntry},
    io::{self, Read, Write},
};

use std::path::Path;
pub struct Repo {
    pub wd: String,
    pub git_path: String,
    pub ignored: Vec<String>,
}

impl Repo {
    pub fn new(dir_path: &String) -> io::Result<Repo> {
        let git_path = dir_path.clone() + "/.git";
        if let Ok(_) = fs::metadata(&git_path) {
            println!("reinializing repository!");
        } else {
            fs::create_dir_all(format!("{}/refs", git_path))?;
            fs::create_dir_all(format!("{}/objects", git_path))?;
            let mut f = fs::File::create_new(format!("{}/HEAD", git_path))?;
            f.write_all("ref: refs/head/main\n".as_bytes())?;
        }
        Ok(Repo {
            wd: dir_path.to_string(),
            git_path,
            ignored: vec![
                String::from("target"),
                String::from("node modules"),
                String::from(".env"),
                String::from(".git"),
            ],
        })
    }

    pub fn assert_is_repo(&self) {
        let path = self.git_path.clone() + "/HEAD";
        println!("asserting path {}  is a repo .", path);
        if let Ok(mut f) = std::fs::File::open(&path) {
            let mut content = String::new();
            let _ = f.read_to_string(&mut content);
            if !content.starts_with("ref: refs/head") {
                panic!("fatal: couldnt read head.");
            }
        } else {
            panic!("fatal: not a git repository.");
        }
    }
    pub fn git_add(&self, args: &Vec<String>) -> Result<(), String> {
        self.assert_is_repo();

        if args.len() == 0 {
            return Err("Invlaid number of arguments".to_string());
        }
        if args[0] == ".".to_string() {
            println!("working dir : {}", self.wd);
            let dir = Path::new(&self.wd);
            self.map_dir_files(dir, &call_back).expect("manga");
        }
        Ok(())
    }
    pub fn cat_file(self, pretty_print: bool, sha: &String) -> Result<String, String> {
        if sha.len() < 4 {
            return Err("minimum 4 letters needed for the sha".to_string());
        }
        let path = self.git_path + &format!("/objects/{}/", &sha[..2]);
        println!("path {path}");
        let encoded_bytes = Self::find_blob(&path, &sha[2..].to_string()).expect("blob not found");
        if pretty_print {
            let mut s = String::new();
            let mut d = ZlibDecoder::new(&encoded_bytes[..]);
            d.read_to_string(&mut s).unwrap();
            return Ok(s);
        }

        Err("not a valid option".to_string())
    }

    fn find_blob(path: &String, sha: &String) -> io::Result<Vec<u8>> {
        let dir = Path::new(path);
        let mut files = Vec::new();
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file()
                    && path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned()
                        .starts_with(sha)
                {
                    files.push(path);
                }
            }
        }
        if files.is_empty(){
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "couldnt find blob",
        ));
        }
        if files.len() > 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "multiple objects found , provide a longer sha",
        ));
        }

        if let Ok(mut f) = fs::File::open(&files[0]) {
            let mut encoded_bytes = Vec::new();
            f.read_to_end(&mut encoded_bytes)
                .expect("error reading blob");
            return Ok(encoded_bytes);
        }else{

        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "corrupted blob",
        ));
        }
    }
    fn map_dir_files(&self, dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
        if dir.is_dir()
            && !self
                .ignored
                .contains(&dir.file_name().unwrap().to_string_lossy().into_owned())
        {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.map_dir_files(&path, cb)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }
}

fn call_back(dir: &DirEntry) {
    println!("{:?}", dir);
}
