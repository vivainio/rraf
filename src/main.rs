#![feature(fs_walk)]
#![feature(dir_entry_ext)]
#![feature(fs)]
#![feature(path_ext)]
#![feature(exit_status)]

use std::io;
use std::fs::{self, PathExt,walk_dir, Metadata};
use std::path::{Path, PathBuf};
use std::env;
use std::thread;

mod futil;
mod winhandle;
use futil::*;
use winhandle::*;
extern crate regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rraf DIRECTORY_TO_DELETE");
        env::set_exit_status(1);
        return;

    }
    let p = Path::new(&args[1]);
    let apb = abspath(&p);
    let ap = apb.as_path();
    
    if !ap.is_dir() {
    	panic!("rraf: You must specify existing directory name!");

    }
    let uncp = to_unc_path(ap);
    let mut counter = 10;
    loop {
        let ok = nuke_tree(&uncp);
        if ok {
            break;
        }
        counter = counter-1;
        if counter == 0 {
            break;
        }
        thread::sleep_ms(2000);
    }
    
}


fn nuke_tree(root: &str) -> bool {
    let walker = walk_dir(root).unwrap();
    let mut failed_files = 0;
    for w in walker {
    	let ent = w.unwrap();
    	let md = ent.metadata().unwrap();
		let path = ent.path();
		if md.is_file() {
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
                            println!("File: {:?} Error: {:?}", path, err.raw_os_error() );

                        }
                    }

                    failed_files += 1;
 
                }    
            }
		} 
        //else if md.is_dir() {
			//println!("D: {:?}", path );
		//}

    }    
    if failed_files > 0 {
        println!("Failed files: {}", failed_files);
        return false;
    }
    let r = fs::remove_dir_all(root);
    if !r.is_err() {
        return true;
    }

    return false;

}


