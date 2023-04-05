use super::commands::{ParsedHead, ParseTypes};
use std::{process::{Command, Stdio, ExitStatus}, io::{stdin, Read}};

pub fn execute(tokens : ParsedHead ) {
	for cmd in tokens.cmds.iter() {
		if cmd.get_type() == &ParseTypes::Word {
			let splitted= cmd.get_value().split(" ").collect::<Vec<&str>>();
			let sed_child = Command::new(splitted[0]).stdout(Stdio::piped())
			.spawn();
			match sed_child {
				Ok( child_out) => {
					let pipe = child_out.wait_with_output().unwrap();
				},
				Err(_) => println!("Error: {}: command not found", splitted[0]),
				
			}
		}
	}
}