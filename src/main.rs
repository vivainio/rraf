#![feature(fs_walk)]
//#![feature(dir_entry_ext)]
//#![feature(fs)]
#![feature(path_ext)]
//#![feature(exit_status)]

use std::io;
use std::fs::{self, PathExt,walk_dir, Metadata};
use std::path::{Path, PathBuf};
use std::env;
use std::thread;
use getopts::{Matches, Options};
use std::process;


mod futil;
mod winhandle;
use futil::*;
use winhandle::*;

extern crate regex;
extern crate getopts;
extern crate glob;

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
    opts.optflag("v", "verbose", "show directories to be deleted");
    opts.optflag("g", "gitclean", "clean all directories in git ignore list");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("g") {
    }


    let free: &Vec<String> = &matches.free;

    if free.len() < 1 {
        println!("rraf: error: Must specify path(s) to delete, wildcards ok");
        process::exit(1);
    }
    let paths = futil::expand_arg_globs(free, matches.opt_present("v"));
    let abspaths: Vec<PathBuf> = paths.into_iter().map(|p| abspath(p.as_path())).collect();

    if abspaths.len() == 0 {
        println!("rraf: warning: nothing to do, no matching paths found")
    }

    nuke_abspaths(&abspaths, &matches);


}

fn nuke_abspaths(abspaths: &Vec<PathBuf>, matches: &Matches) {
    for apbuf in abspaths {
        let ap = apbuf.as_path();
        if matches.opt_present("v") {
            println!("{:?}", ap);
        }
        if !ap.is_dir() {
            println!("Skipping missing directory {:?}", ap);
            continue;
        }

        if matches.opt_present("c") {
            println!("Closing handles: {:?}",ap);
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


