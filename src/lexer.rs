use crate::utils::{parse_bool, parse_string, Token};

pub fn read_stack(fstr: &String) -> Vec<String> {
    let lexemes = fstr
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
        })
        .flat_map(|line| line)
        .collect::<Vec<String>>();

    lexemes
}

pub fn parse_data_type(val: &String) -> Option<Token> {
    if let Ok(i) = val.parse::<i64>() {
        Some(Token::Int(i))
    } else if let Ok(i) = val.parse::<f64>() {
        Some(Token::Float(i))
    } else if let Ok(i) = parse_string(val) {
        Some(Token::Str(i))
    } else if let Ok(i) = parse_bool(val) {
        Some(Token::Bool(i))
    } else {
        None
    }
}
