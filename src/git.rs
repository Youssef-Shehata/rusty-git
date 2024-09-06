use anyhow::{anyhow, bail, Context};
use flate2::{ bufread::ZlibDecoder,write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{
    ffi::CStr,
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

// temporarily to limit the outpu files when testing and developing features
const IGNORED: &[&str] = &["target", ".git", ".gitignore", ".env"];
enum BlobKind {
    Blob,
    Commit,
    Tree,
}
use std::path::Path;
pub fn get_wd() -> anyhow::Result<String> {
    let cwd = std::env::current_dir().expect("failed to get cwd");
    let wd = cwd.to_string_lossy().into_owned();
    let wd_path = Path::new(&wd);
    let mut curr = wd_path;
    loop {
        if let None = curr.parent() {
            break;
        }
        let name = curr.to_str().unwrap().to_string() + "/.git";
        let target = Path::new(&name);
        if target.exists() && target.is_dir() {
            return Ok(target
                .to_str()
                .unwrap()
                .trim_end_matches("/.git")
                .to_string());
        }
        curr = curr.parent().unwrap();
    }
    return Err(anyhow!("fatal: Not a git repository"));
}

pub fn init_repo(name: Option<String>) -> anyhow::Result<()> {
    let cwd = std::env::current_dir().expect("failed to get cwd");
    let mut wd = cwd.to_string_lossy().into_owned();

    if let Some(dir_name) = name {
        if dir_name != "." {
            wd = wd + &dir_name;
        }
    }

    if let Ok(()) = assert_wd_is_repo(&wd) {
        println!("reinitializing repo");
        return Ok(());
    }
    let git_path = wd + "/.git";
    fs::create_dir_all(format!("{}/refs", git_path))?;
    fs::create_dir_all(format!("{}/objects", git_path))?;
    let mut f = fs::File::create_new(format!("{}/HEAD", git_path))?;
    f.write_all("ref: refs/head/main\n".as_bytes())?;
    println!("Initialized empty git repository in {}", git_path);
    Ok(())
}

pub fn assert_wd_is_repo(wd: &String) -> anyhow::Result<()> {
    let head_path = wd.to_owned() + "/.git/HEAD";
    let mut f = std::fs::File::open(&head_path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    if content.starts_with("ref: refs/head") {
        return Ok(());
    }
    bail!("not a repo");
}

pub fn cat_file(pretty_print: bool, sha: &String) -> anyhow::Result<()> {
    if sha.len() < 4 {
        bail!("minimum 4 letters needed for the sha");
    }
    let wd = get_wd()?;
    let git_path = wd + "/.git";

    let objects_path = git_path + &format!("/objects/{}/", &sha[..2]);
    let compressed_text =
        find_blob(&objects_path, &sha[2..].to_string()).context("blob not found")?;

    anyhow::ensure!(pretty_print, "please provide a valid option");
    let mut buff = Vec::new();
    let z_lib = ZlibDecoder::new(&compressed_text[..]);
    let mut buff_reader = BufReader::new(z_lib);
    //read the  header of the blob :blob <size>/0<content>
    buff_reader
        .read_until(0, &mut buff)
        .context("couldnt read blob")?;

    let header = CStr::from_bytes_until_nul(&buff).context("blob header is corruupted")?;
    let header = header
        .to_str()
        .context("Blob has invalid characters , make sure its all UTF-8")?;

    let Some((kind, size)) = header.split_once(" ") else {
        bail!("invalid header of blob file")
    };
    let kind = match kind {
        "blob" => BlobKind::Blob,
        "commit" => BlobKind::Commit,
        "tree" => BlobKind::Tree,
        _ => bail!("uknown file type {kind}"),
    };
    let size = size.parse::<usize>().context("couldn't read blob size")?;

    buff.clear();
    buff.resize(size, 0);
    buff_reader
        .read_exact(&mut buff)
        .context("failed to read blob contents")?;

    let n = buff_reader.read(&mut [0]).context("")?;
    if n != 0 {
        bail!(
            "size of blob exceeded expectations , expected {size} bytes, found {n} trailing bytes."
        );
    }

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    match kind {
        BlobKind::Blob => {
            stdout
                .write_all(&buff)
                .context("couldn't write blob to stdout")?;
        }
        _ => bail!("cant print this one yet, sorry."),
    }
    Ok(())
}

fn find_blob(path: &String, sha: &String) -> anyhow::Result<Vec<u8>> {
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
                if files.len() > 1 {
                    bail!("multiple objects found , provide a longer sha");
                }
            }
        }
    }
    if files.is_empty() {
        bail!("couldnt find blob");
    }

    let mut f = fs::File::open(&files[0]).context("corrupted blob")?;
    let mut encoded_bytes = Vec::new();
    f.read_to_end(&mut encoded_bytes)
        .context("error reading blob")?;
    return Ok(encoded_bytes);
}

pub fn git_add(args: &Vec<String>) -> anyhow::Result<()> {
    let wd = get_wd()?;
    if args[0] == ".".to_string() {
        let dir = Path::new(&wd);

        let files = collect_tracked_files(dir)?;
        for file in files.iter() {
            println!("{}", file.display());
        }
    }
    Ok(())
}
fn collect_tracked_files(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && !IGNORED.contains(&path.file_name().unwrap().to_str().unwrap()) {
            files.extend(collect_tracked_files(&path)?);
        } else if path.is_file() && !IGNORED.contains(&path.file_name().unwrap().to_str().unwrap())
        {
            files.push(path);
        }
    }
    Ok(files)
}

pub fn hash_file(write_to_objects: bool, file_path: String) -> anyhow::Result<String> {
    let file_path = Path::new(&file_path);
    let mut f = fs::File::open(file_path).context("openning file")?;
    let size = f.metadata()?.size();
    println!("{size}");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).context("reading file")?;
    let mut hasher = Sha1::new();
    hasher.update(format!("blob {}\0", size));
    hasher.update(&buf);
    let hashed_bytes = hasher.finalize();
    let sha = hex::encode(hashed_bytes);
    if write_to_objects {
        let wd = get_wd()?;
        println!("working dir {}", wd);
        let wd = wd + &format!("/.git/objects/{}", &sha[..2]);
        let path = Path::new(&wd);
        if path.is_dir() {
        } else {
            fs::create_dir(&path).context("creating blob folder")?;
            let new_file =  &format!("{}/{}" , wd.clone(),&sha[2..]);
            println!("newpath: {}" , new_file);
            let new_file_path = Path::new(&new_file);
            if let Ok(_) = File::open(new_file_path) {
            } else {
                let mut f = File::create(new_file_path).context("creating new blob")?;
                let mut z = ZlibEncoder::new(Vec::new(), Compression::default());
                let ccontent  = String::from_utf8_lossy(&buf);
                println!("content {}", ccontent.to_string());
    z.write_all(format!("blob {}\0", size).as_bytes())?;
                z.write_all(&buf)?;
                let compressed = z.finish().context("compressing file")?;
                f.write_all(&compressed)?;
                
                
            
            }
        }
    }
    Ok(sha)
}
