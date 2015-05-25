use std::process::Command;
use std::path::*;
use regex::{Regex, Captures};
use std::str::FromStr;

pub struct HandleEnt {
	pid: String,
	handle: String,
	process_name: String,
	file_name: String
}

impl HandleEnt {
	pub fn close_handle(&self) {
		println!("Closing handle for {}", self.process_name );
		let o = Command::new("handle")
			.arg("-p")
			.arg(&self.pid)
			.arg("-c")
			.arg(&self.handle)
			.arg("-y")
			.output().unwrap();
		

	}

}

fn dump_named_captures(cs: &Captures) {
	for (k,v) in cs.iter_named() {
		println!("Captured: {:?} {:?} ", k,v);
	}

}

fn bytes_to_str(buf: &Vec<u8>) -> String {
	let mut v = buf.clone();
	v.retain(|&ch| ch != '\r' as u8);
	String::from_utf8_lossy(&v).into_owned()

}

pub fn get_handles(path: &Path) -> Vec<HandleEnt> {

	let out = Command::new("handle")
		.arg(path)
		.output().unwrap();

	let stdout = out.stdout;
	let outs = bytes_to_str(&stdout);
	let re = Regex::new(r"(?m)(?P<img>\S+)\s+pid: (?P<pid>\d+)\s+type: File\s+(?P<hnd>\w+): (?P<rest>.+)$").unwrap();

	let mut res: Vec<HandleEnt> = vec!();
	for cap in re.captures_iter(&outs) {
		//dump_named_captures(&cap);
		let ent = HandleEnt {
			pid: cap.name("pid").unwrap().to_string(),
			handle: cap.name("hnd").unwrap().to_string(),
			file_name: cap.name("rest").unwrap().to_string(),
			process_name: cap.name("img").unwrap().to_string()
		};
		res.push(ent);
	}	
	res
}

/*
#[test]
fn test_handle() {
	let handles = get_handles(Path::new(r"c:\p\rraf\t"));
	handles[0].close_handle();
	let mut v = vec!(12,12);
	bytes_to_str(&mut v);
}
*/

