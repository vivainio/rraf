use std::io;
use std::fs::{self, PathExt, DirEntry};
use std::path::Path;
use std::env;

// one possible implementation of fs::walk_dir only visiting files
fn visit_dirs(dir: &Path, cb: &mut FnMut(DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if entry.path().is_dir() {
                try!(visit_dirs(&entry.path(), cb));
            } else {
                cb(entry);
            }
        }
    }
    Ok(())
}

fn ParseEntry(ent: DirEntry) {

}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
 
        visit_dirs(args[1], ParseEntry );
        println!("The first argument is {}", args[1]);
    }
}
