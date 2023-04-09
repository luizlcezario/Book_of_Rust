use super::commands::{ParseTypes, ParsedHead};
use super::heredoc::heredoc;
use std::io::Error;
use std::{fs::File, process::Stdio};

struct Pipe {
    pipe_in: Stdio,
    pipe_out: Stdio,
    value: String,
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe {
            pipe_in: Stdio::inherit(),
            pipe_out: Stdio::inherit(),
            value: String::new(),
        }
    }
    pub fn open_read(&mut self, path: &String) -> bool {
        let file = File::open(path);
        let error = Pipe::verify_open(file, path, &mut self.pipe_in);
        error
    }
    pub fn open_write(&mut self, path: &String) -> bool {
        let file = File::create(path);
        let error = Pipe::verify_open(file, path, &mut self.pipe_out);
        error
    }
    fn verify_open(file: Result<File, Error>, value: &String, pipe: &mut Stdio) -> bool {
        match file {
            Ok(f) => {
                *pipe = Stdio::from(f);
                true
            }
            Err(_) => {
                println!("minishell: no such file or directory: {}", value);
                false
            }
        }
    }
}

fn execute_pipes(tokens: &ParsedHead) -> (Stdio, Stdio, bool) {
    let mut pipe = Pipe::new();
    let mut error = true;
    for red in tokens.tokens.iter() {
        if red.get_type() == &ParseTypes::Word {
            if pipe.value == ">" {
                error = pipe.open_write(red.get_value());
            } else if pipe.value == "<" {
                error = pipe.open_read(red.get_value());
            } else if pipe.value == ">>" {
                error = pipe.open_write(red.get_value());
            } else if pipe.value == "<<" {
                heredoc(red.get_value());
            }
            if error == false {
                return (pipe.pipe_in, pipe.pipe_out, false);
            }
        } else {
            pipe.value = red.get_value().clone();
        }
    }
    return (pipe.pipe_in, pipe.pipe_out, true);
}

pub fn execute(tokens: ParsedHead) {
    let (mut pipe_in, pipe_out, error) = execute_pipes(tokens.get);
    if error == false {
        return;
    }
    for (u, cmd) in tokens.tokens.iter().enumerate() {
        if cmd.get_type() == &ParseTypes::Word {
            if u == tokens.tokens.len() - 1 {
                break;
            } else {
                let sed = cmd.execute(pipe_in, Stdio::piped());
                pipe_in = sed.stdout.expect("error on stdout").into();
            }
        }
    }
    let sed = tokens
        .cmds
        .back()
        .expect("errror")
        .execute(pipe_in, pipe_out);
    (sed.wait_with_output().expect("error on wait"));
}
