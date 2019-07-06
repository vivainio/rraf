use std::process::Command;
use std::path::{Path, PathBuf};
use std::str::Lines;


pub fn git_ignored_dirs(workdir: &str) -> Vec<String> {
    let out = Command::new("git")
        .arg("ls-files")
        .arg("-o")
        .arg("--directory")
        .current_dir(workdir)
        .output()
        .unwrap().stdout;
    let s = String::from_utf8_lossy(&out).into_owned();
    let lines: Vec<String> = s.lines().map(|l| l.into()).collect();
    lines
}



#[test]
fn test_git() {
    let mut ls = git_ignored_dirs("c:/r/1");
    ls.retain(|l| l.ends_with("/"));
    ls.sort_by_key(|k| k.len());
    for l in ls {
        print!("Line {}\n", l);
    }
    //let mut g = git("c:/p2p".into());
    //  ls-files -o --directory
}
