use std::{
    collections::LinkedList,
    process::{Child, Command, Stdio},
};

#[derive(PartialEq, Clone)]
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

impl PartialEq for ElementLine {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

impl ElementLine {
    pub fn new() -> ElementLine {
        ElementLine {
            parse_type: ParseTypes::End,
            value: String::new(),
        }
    }
    pub fn select_type(&mut self, value: &String) {
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
            sed_child = Command::new(splitted[0])
                .arg(splitted[1..].concat())
                .stdin(pipe_in)
                .stdout(pipe_out)
                .spawn()
                .expect("error on spawn");
        } else {
            sed_child = Command::new(splitted[0])
                .stdin(pipe_in)
                .stdout(pipe_out)
                .spawn()
                .expect("error on spawn");
        }
        return sed_child;
    }
}

pub struct ParsedHead {
    n_cmds: i32,
    pub tokens: LinkedList<ElementLine>,
}

impl ParsedHead {
    pub fn add_token(&mut self, cmd: ElementLine) {
        self.tokens.push_back(cmd);
        self.n_cmds += 1;
    }

    pub fn new() -> ParsedHead {
        ParsedHead {
            n_cmds: 0,
            tokens: LinkedList::new(),
        }
    }
    pub fn get_all_until_next(&mut self, now: &ElementLine) -> LinkedList<&ElementLine> {
        let mut list = LinkedList::new();
        let mut now_pos = false;
        for token in self.tokens.iter() {
            if token == now {
                now_pos = true;
            } else if now_pos && token.get_type() == &ParseTypes::Pipe {
                break;
            } else if now_pos && token.get_type() == &ParseTypes::End {
                break;
            } else if now_pos {
                list.push_back(token);
            }
        }
        return list;
    }
}
