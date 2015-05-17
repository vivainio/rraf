#![feature(fs_walk)]
#![feature(dir_entry_ext)]
#![feature(fs)]
#![feature(collections)]
#![feature(path_ext)]


use std::io;
use std::fs::{self, PathExt, DirEntry, walk_dir, Metadata};
use std::path::Path;
use std::env;
use std::os;

fn to_unc_path(path: &Path) -> String {
	let buf = path.to_str().unwrap().clone();
	let ns = format!(r"\\?\{}", buf);
	ns
}

fn remove_file(path: &Path, metadata: &Metadata) -> io::Result<()> {
    let mut perms = metadata.permissions();

    if (perms.readonly()) {
        perms.set_readonly(false);
        fs::set_permissions(path, perms);
    }
    let res = fs::remove_file(path);
    
    match res {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("Delete failed {:?}", path);

            Err(io::Error::last_os_error())
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let p = Path::new(&args[1]);

    if !p.is_absolute() {
    	panic!("rraf: You must specify absolute path name!");
    }

    if !p.is_dir() {
    	panic!("rraf: You must specify existing directory name!");

    }

    let uncp = to_unc_path(p);
    nuke_tree(&uncp);
}

fn nuke_tree(root: &str) {
    let walker = walk_dir(root).unwrap();
    let mut to_retry: Vec<String> = Vec::new(); 
    for w in walker {
    	let ent = w.unwrap();
    	let md = ent.metadata().unwrap();
		let path = ent.path();
		if md.is_file() {
 			println!("F: {:?}", path );
            let r = remove_file(&path, &md);
            if r.is_err() {
                to_retry.push(String::from_str(path.to_str().unwrap()));
            }

		} else if md.is_dir() {
			println!("D: {:?}", path );
		}

    	//println!("Path: {}", path);
    }
    if to_retry.len() > 0 {
    	println!("To retry: {}", to_retry.len());
    } else {
    	let r = fs::remove_dir_all(root);
    }
}


