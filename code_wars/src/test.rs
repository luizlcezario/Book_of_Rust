use std::{fs::File, process::{Stdio, Command}, };

pub fn test() {
	let  file = File::create("foo.txt").expect("error on open file");
	let outfile = Stdio::from(file);
	let cmd = Command::new("ls").arg("-la").stdout(outfile).spawn().expect("error on spawn");
	cmd.wait_with_output().expect("error on wait");
} 