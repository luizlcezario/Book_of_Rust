use super::commands::{ParsedHead, ElementLine, ParseTypes};

fn check_error(line: &String) {
	if line.len() >= 2 { 
		if line[0..2].contains("||") || line[0..2].contains("&&") {
			panic!("minishell: syntax error near unexpected token `{}'", &line[0..2]);
		}
	}
	else if line.len() > 1 && line[0..1].contains("|") {
		panic!("minishell: syntax error near unexpected token `|'");
	}
}

fn parse_pipe(tokens: & mut ParsedHead, line: &String, i: & mut usize) -> ParseTypes{
	let mut element = ElementLine::new();
	let next = line.chars().nth(*i + 1).unwrap();
	if next == '|' {
		element.select_type(&String::from("||"));
		element.add_value(String::from("||"));
		tokens.add_cmd(element);
		*i += 2;
	} else  {
		element.select_type(&String::from("|"));
		element.add_value(String::from("|"));
		tokens.add_cmd(element);
		*i += 1;
	}
	return ParseTypes::Pipe;
}

fn validade_quote(line: &String, i: & usize)-> usize{
	let string = line.get((*i + 1)..).expect("minishell: syntax error near unexpected token `newline'");
	return string.find(line.chars().nth(*i).unwrap()).expect("minishell: syntax error near unexpected token `newline'") + 1;
}

fn parse_word(tokens: & mut ParsedHead, line: &String, i: & mut usize, last_type: &ParseTypes) -> ParseTypes {
	let mut element = ElementLine::new();
	let mut word = String::new();
	while *i < line.len() {
		if line.chars().nth(*i).unwrap() == '\"' || line.chars().nth(*i).unwrap() == '\'' {
			let pos = validade_quote(&line, i);
			word.push_str(line.get(*i..=(*i + pos)).expect("minishell: syntax error near unexpected token `newline'"));
			*i += pos + 1;
		}
		else if line.chars().nth(*i).unwrap() == '|' || line.chars().nth(*i).unwrap() == '>' || line.chars().nth(*i).unwrap() == '<'  || line.chars().nth(*i).unwrap() == '&' {
			*i -= 1;
			break;
		}else {
			word.push(line.chars().nth(*i).unwrap());
			*i += 1;
		}
	}
	element.select_type(&word);
	element.add_value(word);
	if *last_type == ParseTypes::Pipe || *last_type == ParseTypes::End {
		tokens.add_cmd(element);
	} else if *last_type == ParseTypes::Redirection {
		tokens.add_redirection(element);
	} else if *last_type == ParseTypes::Word {
		panic!("minishell: syntax error near unexpected token `newline'")
	}
	return ParseTypes::Word;
}

pub fn parser(line: &String) -> ParsedHead {
	let mut tokens = ParsedHead::new();
	let trined = String::from(line.trim());
	let mut i = 0;
	let mut last_type = ParseTypes::End;
	check_error(&trined);
	while i < trined.len() {
		match trined.chars().nth(i).unwrap() {
			'|' => last_type = parse_pipe(& mut tokens, &trined, & mut i),
			'&' => println!("Pipe"),
			'>' => println!("Redirection"),
			'<' => println!("Redirection"),
			' ' => println!("Space"),
			_ => last_type = parse_word(& mut tokens, &trined, & mut i, & mut last_type),
		}
		i+= 1;
	}
	return tokens;
}