use std::io;
use std::fs::{self, PathExt,walk_dir, Metadata};
use std::path::{Path, PathBuf};
use std::env;
use std::thread;



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
            //println!("Delete failed {:?}", path);

            Err(err)
        }
    }
}

pub fn abspath(path: &Path) -> PathBuf {
    let cwd = env::current_dir().unwrap();
    //println!("CWD {:?}", cwd);
    let mut buf = PathBuf::new();
    buf.push(cwd);
    buf.push(path);
    normalize(buf.as_path())
   // buf.push(cwd);
}


#[test]
fn abspath_root() {
	let ap = abspath(Path::new("\\"));

}
