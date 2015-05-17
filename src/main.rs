#![feature(fs_walk)]

use std::io;
use std::fs::{self, PathExt, DirEntry, walk_dir};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let p = Path::new(&args[1]);
    let walker = walk_dir(p).unwrap();
    for w in walker {
    	let ent = w.unwrap();
 		println!("({:?})", ent.path() );
    	//println!("Path: {}", path);
    }
    1;

}
