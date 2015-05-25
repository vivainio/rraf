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
use getopts::Options;

mod futil;
mod winhandle;
use futil::*;
use winhandle::*;
extern crate regex;
extern crate getopts;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("c", "close", "close locked file handles");
    opts.optflag("h", "help", "print this help menu");
    /*
    if args.len() < 2 {
        println!("Usage: rraf DIRECTORY_TO_DELETE");
        env::set_exit_status(1);
        return;

    }
    */
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };


    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }        

    let free = &matches.free;
    if free.len() < 1 {
        println!("rraf: error: Must specify path to delete");
 
        env::set_exit_status(1);
        return;
    }
    let p = Path::new(&free[0]);
    let apb = abspath(&p);
    let ap = apb.as_path();
    
    if !ap.is_dir() {
        println!("rraf: error: path does not exist: {:?}", ap);
 
        env::set_exit_status(1);
        return;
 
    }

    if matches.opt_present("c") {
        println!("Closing handles {:?}",ap );
        close_handles(ap);
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

fn close_handles(root: &Path) {
    let handles = winhandle::get_handles(&root);
    for h in handles {
        h.close_handle();
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


