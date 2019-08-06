use std::io;
use std::fs::{self, Metadata};
use std::path::{Path, PathBuf};
use std::env;
//use std::thread;
use glob;
use winhandle::get_handles;
use std::process;
use std::num;
use std::env::temp_dir;
use core::borrow::Borrow;
use rand::Rng;
use walkdir::WalkDir;
use std::ffi::OsStr;


pub fn normalize(path: &Path) -> PathBuf {
  use std::path::Component::*;

  let mut ret = PathBuf::new();
  for component in path.components() {
    match component {
      CurDir => {},
      ParentDir => { ret.pop(); }
      _ => ret.push(component.as_os_str())
    }
  }
  ret
}

pub fn expand_arg_globs(globs: &Vec<String>, warn: bool) -> Vec<PathBuf> {
    let mut res: Vec<PathBuf> = Vec::new();
    for g in globs {
        let hits = glob::glob(g).unwrap();
        let mut found = false;
        for ho in hits {
            let h: PathBuf = ho.unwrap();
            found = true;
            res.push(h);
        }
        if !found && warn {
            println!("rraf: warning: no files matching pattern '{}'", g);

        }
    }
    return res;
}


pub fn to_unc_path(path: &Path) -> String {
    let buf = path.to_str().unwrap().clone();
    let ns = format!(r"\\?\{}", buf);
    ns
}

pub fn remove_file(path: &Path, metadata: &Metadata) -> io::Result<()> {
    let mut perms = metadata.permissions();

    if perms.readonly() {
        perms.set_readonly(false);
        let _ = fs::set_permissions(path, perms);
    }
    let res = fs::remove_file(path);

    match res {
        Ok(()) => Ok(()),
        Err(err) => {
            Err(err)
        }
    }
}

pub fn abspath(path: &Path) -> PathBuf {
    let cwd = env::current_dir().unwrap();
    let mut buf = PathBuf::new();
    buf.push(cwd);
    buf.push(path);
    normalize(buf.as_path())
}

pub struct Trash {
    root_path: PathBuf,
    cur_path: PathBuf
}

impl Trash {
    pub fn new() -> Trash {
        let pid: String = process::id().to_owned().to_string();
        let tdir = get_trash_dir();
        let mut cd = tdir.clone();
        
        cd.push(pid );
        fs::create_dir_all(&cd).expect("Could not create trash dir");
        Trash {
            root_path: tdir,
            cur_path: cd
        }
    }
    
    pub fn move_path(&self, path: &Path) -> Result<(), io::Error> {
        let tdir = &self.cur_path;
        let mut r = tdir.clone();
        let mut dname = rand::thread_rng().gen_range(0, 9999999).to_owned().to_string();
        
        let stem = path.file_stem().unwrap_or(OsStr::new("nn")).to_str().unwrap();
        dname.push_str(&stem);
        r.push(dname);
        print!("Trash to: {} -> {}\n", &path.display(), &r.display());
        fs::rename(&path, &r)
        
    }
    
    pub fn purge(&self) {
        print!("Purging {}\n", self.cur_path.display());
        fs::remove_dir_all(&self.cur_path).expect("purge failed");
    }
}

pub fn get_trash_dir() -> PathBuf {
    let mut p = env::temp_dir();
    p.push("rraf_trash");
    p
}

pub fn nuke_tree(root: &str) -> bool {
    //let walker = walk_dir(root).unwrap();
    let walker = WalkDir::new(root);
    let mut failed_files = 0;
    for w in walker {
        let ent = w.unwrap();
        let md = ent.metadata().unwrap();
        let path = ent.path();
        let file_type = md.file_type();

        if file_type.is_symlink() {
            println!("rraf: error: Symlink found, bailing out: {:?}", path);
            return false;
        }

        if file_type.is_file() {

            //println!("F: {:?}", path );
            let r = remove_file(&path, &md);
            match r {
                Ok(()) => (),
                Err(err) =>  {
                    match err.raw_os_error() {
                        Some(32) => {
                            println!("Busy: {:?}", path);
                        },
                        _ => {
                            println!("File: {:?} Error: {:?}", path, err.raw_os_error());
                        }
                    }
                    failed_files += 1;
                }
            }
        } else if file_type.is_dir() {
            let _ =fs::remove_dir_all(path);
        }
    }
    if failed_files > 0 {
        println!("Failed files: {}", failed_files);
        return false;
    }
    let r = fs::remove_dir_all(root);
    return match r {
        Ok(()) => true,
        Err(err) => {
            if err.raw_os_error().unwrap() == 2 { true } else { false }
        }
    }

}


#[test]
fn use_trash() {
    let t = Trash::new();
    
    // 1. create dir to nuke
    let mut td = env::temp_dir();
    td.push("one");
    fs::create_dir(&td);
    t.move_path(td.as_path());
    
}   
     
#[test]
fn test_trash_dir() {
    let td = get_trash_dir();
    println!("trash: {}", td.to_str().unwrap());
}


#[test]
fn abspath_root() {
	let _ = abspath(Path::new("\\"));
}
#[test]
fn arg_globs() {
    {
        let mut vec = Vec::new();
        vec.push("hello".to_string());
        vec.push("world".to_string());
        vec.push("c:/Users/*".to_string());
        let matches = expand_arg_globs(&vec, true);
        assert!(matches.len() > 0);
    }
}

