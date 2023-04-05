use std::collections::LinkedList;

#[derive(PartialEq)]
pub enum ParseTypes {
	Word,
	Pipe,
	Redirection,
	End,
}

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
	pub fn get_value(&self) -> String {
		self.value.clone()
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