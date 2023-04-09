use super::commands::{ElementLine, ParseTypes, ParsedHead};

fn check_error(line: &String) -> bool {
    if line.len() >= 2 {
        if line[0..2].contains("||") || line[0..2].contains("&&") {
            println!("minishell: parse error near `{}'", &line[0..2]);
            return true;
        }
    } else if line.len() > 1 && line[0..1].contains("|") {
        println!("minishell: parse error near `|'");
        return true;
    }
    return false;
}

fn parse_pipe(
    tokens: &mut ParsedHead,
    i: &mut usize,
    last_add: &ElementLine,
) -> (ElementLine, bool) {
    let mut element = ElementLine::new();
    if last_add.get_type() != &ParseTypes::Word {
        println!(
            "minishell: syntax error near unexpected token `{}'",
            last_add.get_value()
        );
        return (element, true);
    }
    element.select_type(&String::from("|"));
    element.add_value(String::from("|"));
    tokens.add_token(element);
    *i += 1;
    return (tokens.tokens.back().unwrap().clone(), false);
}

fn parse_redirection(
    tokens: &mut ParsedHead,
    line: &String,
    i: &mut usize,
    last_add: &ElementLine,
) -> (ElementLine, bool) {
    let mut element = ElementLine::new();
    let mut word = String::new();
    if last_add.get_type() != &ParseTypes::Word {
        println!(
            "minishell: syntax error near unexpected token `{}'",
            last_add.get_value()
        );
        return (element, true);
    }
    if *i + 1 < line.len() && line[*i..*i + 1] == line[*i + 1..*i + 2] {
        word.push_str(line[*i..*i + 2].as_ref());
        *i += 2;
    } else {
        word.push_str(line[*i..*i + 1].as_ref());
        *i += 1;
    }
    element.select_type(&word);
    element.add_value(word);
    tokens.add_token(element);
    return (tokens.tokens.back().unwrap().clone(), false);
}

fn validade_quote(line: &String, i: &usize) -> (usize, bool) {
    let string = line
        .get((*i + 1)..)
        .unwrap()
        .find(line.chars().nth(*i).unwrap());
    match string {
        Some(x) => return (x, false),
        None => {
            println!("minishell: syntax error near unexpected token `newline'");
            return (0, true);
        }
    }
}

fn parse_word(
    tokens: &mut ParsedHead,
    line: &String,
    i: &mut usize,
    last_type: &ElementLine,
) -> (ElementLine, bool) {
    let mut element = ElementLine::new();
    let mut word = String::new();
    while *i < line.len() {
        if line.chars().nth(*i).unwrap() == '\"' || line.chars().nth(*i).unwrap() == '\'' {
            let (pos, error) = validade_quote(&line, i);
            word.push_str(
                line.get(*i..=(*i + pos))
                    .expect("minishell: syntax error near unexpected token `newline'"),
            );
            *i += pos + 1;
            if error == true {
                break;
            }
        } else if line.chars().nth(*i).unwrap() == '|'
            || line.chars().nth(*i).unwrap() == '>'
            || line.chars().nth(*i).unwrap() == '<'
            || line.chars().nth(*i).unwrap() == '&'
        {
            *i -= 1;
            break;
        } else {
            word.push(line.chars().nth(*i).unwrap());
            *i += 1;
        }
    }
    element.select_type(&word);
    element.add_value(word);
    if *last_type.get_type() == ParseTypes::Pipe
        || *last_type.get_type() == ParseTypes::End
        || *last_type.get_type() == ParseTypes::Redirection
    {
        tokens.add_token(element);
        return (tokens.tokens.back().unwrap().clone(), false);
    } else {
        println!("minishell: syntax error near unexpected token `newline'");
        return (element, true);
    }
}

pub fn parser(line: &String) -> (ParsedHead, bool) {
    let mut tokens = ParsedHead::new();
    let trined = String::from(line.trim());
    let mut i = 0;
    let mut last_type = ElementLine::new();
    let mut error = check_error(&trined);
    while i < trined.len() {
        match trined.chars().nth(i).unwrap() {
            '|' => (last_type, error) = parse_pipe(&mut tokens, &mut i, &last_type),
            '>' => {
                (last_type, error) = parse_redirection(&mut tokens, &trined, &mut i, &mut last_type)
            }
            '<' => {
                (last_type, error) = parse_redirection(&mut tokens, &trined, &mut i, &mut last_type)
            }
            ' ' => {}
            _ => (last_type, error) = parse_word(&mut tokens, &trined, &mut i, &mut last_type),
        }
        if error == true {
            return (tokens, error);
        }
        i += 1;
    }
    return (tokens, error);
}
