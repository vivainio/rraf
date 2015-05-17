#![feature(fs_walk)]
#![feature(dir_entry_ext)]

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
    let uncp = to_unc_path(p);
    let walker = walk_dir(uncp).unwrap();
    for w in walker {
    	let ent = w.unwrap();
    	let md = ent.metadata().unwrap();
		let path = ent.path();
		if (md.is_file()) {
 			println!("F: ({:?})", path );
            remove_file(&path, &md);
		} else if (md.is_dir()) {
			println!("D: ({:?})", path );
		}

    	//println!("Path: {}", path);
    }

}
