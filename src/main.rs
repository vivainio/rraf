#![feature(fs_walk)]
#![feature(dir_entry_ext)]

use std::io;
use std::fs::{self, PathExt, DirEntry, walk_dir};
use std::path::Path;
use std::env;
use std::os;

fn to_unc_path(path: &Path) -> String {
	let buf = path.to_str().unwrap().clone();
	let ns = format!(r"\\?\{}", buf);
	ns
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let p = Path::new(&args[1]);
    let uncp = to_unc_path(p);
    let walker = walk_dir(uncp).unwrap();
    for w in walker {
    	let ent = w.unwrap();
    	let md = ent.metadata().unwrap();
		if (md.is_file()) {
 			println!("({:?})", ent.path() );
		} 		
    	//println!("Path: {}", path);
    }
    1;

}
