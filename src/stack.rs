use crate::extract_value_from_token;
use crate::utils::{most_generic_type, Token};

pub fn drop(stack: &mut Vec<Token>) {
    assert!(stack.len() >= 1, "Cannot DROP nothing!");
    stack.pop().expect("Unreachable error!");
}

pub fn dup(stack: &mut Vec<Token>) {
    assert!(stack.len() >= 1, "Cannot DUP nothing!");
    stack.push(stack[stack.len() - 1].clone())
}

pub fn swap(stack: &mut Vec<Token>) {
    assert!(stack.len() >= 2, "Cannot SWAP without at least 2 elements!");
    let top = stack.len() - 1;
    stack.swap(top, top - 1)
}

///
/// BINARY OPERATIONS
///

/// Function that pops the first two elements off the stack
/// and passes them to the binary function passed as an argument! 
pub fn apply_bin_op(
    stack: &mut Vec<Token>, 
    func: impl Fn(Token, Token) -> Token
) {
    assert!(stack.len() >= 2, "Cannot + without at least 2 elements!");
    let second = stack.pop().expect("Unreachable err.");
    let first = stack.pop().expect("Unreachable err.");
   
    let res = func(first, second);
    stack.push(res.clone())
}

fn bin_int_function(first: Token, second: Token, func: impl Fn(i64, i64) -> i64) -> Token {
    let (fv, sv) = (
        extract_value_from_token!(first, i64), 
        extract_value_from_token!(second, i64)
    );
    let res = func(fv, sv);
    Token::Int(res)
}

fn bin_float_function(first: Token, second: Token, func: impl Fn(f64, f64) -> f64) -> Token {
    let (fv, sv) = (
        extract_value_from_token!(first, f64), 
        extract_value_from_token!(second, f64)
    );
    let res = func(fv, sv);
    Token::Float(res)
}

fn bin_string_function(first: Token, second: Token, func: impl Fn(String, String) -> String) -> Token {
    let (fv, sv) = (
        extract_value_from_token!(first, String), 
        extract_value_from_token!(second, String)
    );
    let res = func(fv, sv);
    Token::Str(res)
}

fn bin_bool_function(first: Token, second: Token, func: impl Fn(bool, bool) -> bool) -> Token {
    let (fv, sv) = (
        extract_value_from_token!(first, bool), 
        extract_value_from_token!(second, bool)
    );
    let res = func(fv, sv);
    Token::Bool(res)
}

/// OPERATOR implementations

pub fn add(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a + b),
        Token::Float(_) => bin_float_function(first, second, |a, b| a + b),
        Token::Str(_) => bin_string_function(first, second, |mut a, b| { a.push_str(b.as_str()); a }),
        Token::Bool(_) => panic!("Cannot add booleans!"),
    }
}

pub fn sub(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a - b),
        Token::Float(_) => bin_float_function(first, second, |a, b| a - b),
        Token::Str(_) => panic!("Cannot perform subtraction on Strings!"),
        Token::Bool(_) => panic!("Cannot subtract booleans!"),
    }
}

pub fn mul(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a * b),
        Token::Float(_) => bin_float_function(first, second, |a, b| a * b),
        Token::Str(fv) => {
            let sv = extract_value_from_token!(second, i64);
            if sv < 0 {
                panic!("Cannot multiply Strings by negative int!")
            }
            return Token::Str(fv.repeat(sv as usize))
        },
        Token::Bool(_) => panic!("Cannot multiply booleans!"),
    }
}

pub fn div(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| {
            if b != 0 { 
                a / b 
            } else { 
                panic!("Cannot divide by 0!") 
            }
        }),
        Token::Float(_) => bin_float_function(first, second, |a, b| {
            if b != 0.0 { 
                a / b 
            } else { 
                panic!("Cannot divide by 0!") 
            }
        }),
        Token::Str(_) => panic!("Cannot perform division on Strings!"),
        Token::Bool(_) => panic!("Cannot divide booleans!"),
    }
}

pub fn exp(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a.pow(b as u32)),
        Token::Float(_) => bin_float_function(first, second, |a, b| a.powf(b)),
        Token::Str(_) => panic!("Cannot perform expontiation on Strings!"),
        Token::Bool(_) => panic!("Cannot perform exponentiation on  booleans!"),
    }
}

pub fn modu(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a % b),
        Token::Float(_) => bin_float_function(first, second, |a, b| a % b),
        Token::Str(_) => panic!("Cannot perform modular operation on Strings!"),
        Token::Bool(_) => panic!("Cannot perform modular operation on booleans!"),
    }
}
