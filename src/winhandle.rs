use std::process::Command;
use std::path::*;
use regex::Regex;

pub fn get_handles(path: &Path) {
	let out = Command::new("handle")
		.arg(path)
		.output().unwrap();
	let stdout = &out.stdout;
	let outs = String::from_utf8_lossy(&out.stdout);
	let re = Regex::new(r"(?m)(\S+)\s+pid: (\d+)\s+type: File\s+(\w+):(.+)$").unwrap();


	for cap in re.captures_iter(&outs) {
		println!("0 {:?} 1 {:?}", cap.at(0), cap.at(1) );
	}	
	println!("Out {:?}", outs);


}

#[test]
fn test_handle() {
	get_handles(Path::new(r"c:\p\rraf"));

}


