use std::{collections::LinkedList, process::{Command, Stdio, Child}};

#[derive(PartialEq)]
#[derive(Clone)]
pub enum ParseTypes {
	Word,
	Pipe,
	Redirection,
	End,
}
#[derive(Clone)]
pub struct ElementLine {
	parse_type: ParseTypes,
	value: String,
}

impl ElementLine {
	pub fn new() -> ElementLine {
		ElementLine  {
			parse_type: ParseTypes::End,
			value: String::new(),
		}
	}
	pub fn select_type(&mut self, value: & String){
		if value == "|" {
			self.parse_type = ParseTypes::Pipe;
		} else if value == ">" || value == "<" || value == ">>" || value == "<<" {
			self.parse_type = ParseTypes::Redirection;
		} else {
			self.parse_type = ParseTypes::Word;
		}
	}
	pub fn add_value(&mut self, value: String) {
		self.value.push_str(&value);
	}
	pub fn get_value(&self) -> &String {
		&self.value
	}
	pub fn get_type(&self) -> &ParseTypes {
		&self.parse_type
	}
	pub fn execute(&self, pipe_in: Stdio, pipe_out: Stdio) -> Child {
		let splitted = self.value.split(" ").collect::<Vec<&str>>();
		let sed_child;
		if splitted[1..].concat() != "" {
			sed_child = Command::new(splitted[0]).arg(splitted[1..].concat()).stdin(pipe_in).stdout(pipe_out).spawn().expect("error on spawn");
		}
		else {
			sed_child = Command::new(splitted[0]).stdin(pipe_in).stdout(pipe_out).spawn().expect("error on spawn");
		}
		return sed_child;
	}
}

pub struct ParsedHead {
	n_cmds: i32,
	n_redirections: i32,
	pub cmds: LinkedList<ElementLine>,
	pub redirections: LinkedList<ElementLine>,
}

impl ParsedHead {
	pub fn add_cmd(&mut self, cmd: ElementLine) {
		self.cmds.push_back(cmd);
		self.n_cmds += 1;
	}
	pub fn add_redirection(&mut self, redirection: ElementLine) {
		self.redirections.push_back(redirection);
		self.n_redirections += 1;
	}

	pub fn new() -> ParsedHead {
		ParsedHead {
			n_cmds: 0,
			n_redirections: 0,
			cmds: LinkedList::new(),
			redirections: LinkedList::new(),
		}
	}
}
