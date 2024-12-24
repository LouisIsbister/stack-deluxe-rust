use crate::stack;
use crate::utils::{parse_string, Token};

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

pub fn exec_stack(lexemes: &Vec<String>) -> Vec<Token> {
    let mut stack = Vec::new();

    for val in lexemes {
        if let Ok(i) = val.parse::<i64>() {
            stack.push(Token::Int(i));
            continue;
        } else if let Ok(i) = val.parse::<f64>() {
            stack.push(Token::Float(i));
            continue;
        } else if let Ok(i) = parse_string(val) {
            stack.push(Token::Str(i));
            continue;
        }

        match val.as_str() {
            "+" => stack::apply_bin_op(&mut stack, stack::add),
            "-" => stack::apply_bin_op(&mut stack, stack::sub),
            "*" => stack::apply_bin_op(&mut stack, stack::mul),
            "/" => stack::apply_bin_op(&mut stack, stack::div),
            "**" => stack::apply_bin_op(&mut stack, stack::exp),
            "%" => stack::apply_bin_op(&mut stack, stack::modu),

            "DROP" => stack::drop(&mut stack),
            "DUP" => stack::dup(&mut stack),
            "SWAP" => stack::swap(&mut stack),

            _ => panic!("Unsupported stack element: '{}'", val),
        }
    }

    stack
}
