use super::heredoc::heredoc;
use super::commands::{ParsedHead, ParseTypes};
use std::{process::{ Stdio}, fs::File };

fn execute_pipes(tokens : &ParsedHead ) -> (Stdio, Stdio) {
	let mut pipe_in = Stdio::inherit();
	let mut pipe_out = Stdio::inherit();
	let mut value = &String::new();
	for red in tokens.redirections.iter() {
		if red.get_type() == &ParseTypes::Word {
			if value == ">" {
				let file = File::create(red.get_value()).expect("error on open file");
				pipe_out = Stdio::from(file);
			}
			else if value == "<"{
				let file = File::open(red.get_value()).expect("error on open file");
				pipe_out = Stdio::from(file);
			}
			else if value == ">>" {
				let file = File::create(red.get_value()).expect("error on open file");
				pipe_in = Stdio::from(file);
			}
			else if value == "<<" {
				heredoc(red.get_value());
			}
		} else {
			value = red.get_value()
		}
	}
	return (pipe_in, pipe_out);
}

pub fn execute(tokens : ParsedHead ) {
	let (mut pipe_in, pipe_out) = execute_pipes(&tokens);
	for (u, cmd) in tokens.cmds.iter().enumerate() {
		if cmd.get_type() == &ParseTypes::Word {
			if u == tokens.cmds.len() - 1 {
				break ;
			}
			else {
				let sed = cmd.execute(pipe_in, Stdio::piped());
				pipe_in = sed.stdout.expect("error on stdout").into();
			}
		}
	}
	let sed = tokens.cmds.back().expect("errror").execute(pipe_in, pipe_out);
	(sed.wait_with_output().expect("error on wait"));
}
