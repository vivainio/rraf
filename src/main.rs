//#![feature(fs_walk)]
//#![feature(dir_entry_ext)]
//#![feature(fs)]
//#![feature(path_ext)]
//#![feature(exit_status)]

//use std::io;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::env;
use std::thread;
use getopts::{Matches, Options};
use std::process;
use std::time::Duration;

mod futil;
mod winhandle;
use futil::*;
mod gitcmd;
use gitcmd::git_ignored_dirs;

extern crate regex;
extern crate getopts;
extern crate glob;
extern crate walkdir;
extern crate core;
extern crate rand;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    println!("rraf version: 1.0.1");
    println!("Project homepage: https://github.com/vivainio/rraf")
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("c", "close", "close locked file handles");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "verbose", "show directories to be deleted");
    opts.optflag("", "version", "show version info");
    opts.optflag("g" ,"git", "faster gitclean");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("version") {
        print_version();
        return;
    }

    if matches.opt_present("git") {
        handle_gitignore();
        return;
    }
    let free: &Vec<String> = &matches.free;

    if free.len() < 1 {
        println!("rraf: error: Must specify path(s) to delete, wildcards ok");
        println!("'rraf -h' for help on available options");

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
            let ok = futil::nuke_tree(&uncp);
            if ok {
                break;
            }
            counter = counter-1;
            if counter == 0 {
                break;
            }
            thread::sleep(Duration::from_secs(2));
        }
    }

}

fn close_handles(root: &Path) {
    let handles = winhandle::get_handles(&root);
    for h in handles {
        h.close_handle();
    }
}


fn handle_gitignore() {
    
    env::set_current_dir("c:/r/1");
    let mut ls = git_ignored_dirs("c:/r/1");
    ls.retain(|l| l.ends_with("/"));
    ls.sort_by_key(|k| k.len());
    let trash = Trash::new();
    
    for l in ls {
        print!("Line {}\n", l);
        let p = Path::new(&l);
        let status = trash.move_path(&p);
        if status.is_err() {
            print!("Error moving out {}", &l);
        }
    }
    
    trash.purge();
    
}