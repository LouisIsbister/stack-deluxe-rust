use crate::value_from_token;
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
    let (fv, sv) = (value_from_token!(first, i64), value_from_token!(second, i64));
    let res = func(fv, sv);
    Token::Int(res)
}

fn bin_float_function(first: Token, second: Token, func: impl Fn(f64, f64) -> f64) -> Token {
    let (fv, sv) = (value_from_token!(first, f64), value_from_token!(second, f64));
    let res = func(fv, sv);
    Token::Float(res)
}

fn bin_string_function(first: Token, second: Token, func: impl Fn(String, String) -> String) -> Token {
    let (fv, sv) = (value_from_token!(first, String), value_from_token!(second, String));
    let res = func(fv, sv);
    Token::Str(res)
}

/// OPERATOR implementations

pub fn add(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a + b),
        Token::Float(_) => bin_float_function(first, second, |a, b| a + b),
        Token::Str(_) => bin_string_function(first, second, |mut a, b| { a.push_str(b.as_str()); a }),
    }
}

pub fn sub(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a - b),
        Token::Float(_) => bin_float_function(first, second, |a, b| a - b),
        Token::Str(_) => panic!("Cannot perform subtraction on Strings!"),
    }
}

pub fn mul(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a * b),
        Token::Float(_) => bin_float_function(first, second, |a, b| a * b),
        Token::Str(fv) => {
            let sv = value_from_token!(second, i64);
            if sv < 0 {
                panic!("Cannot multiply Strings by negative int!")
            }
            return Token::Str(fv.repeat(sv as usize))
        },
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
    }
}

pub fn exp(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a.pow(b as u32)),
        Token::Float(_) => bin_float_function(first, second, |a, b| a.powf(b)),
        Token::Str(_) => panic!("Cannot perform expontiation on Strings!"),
    }
}

pub fn modu(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a % b),
        Token::Float(_) => bin_float_function(first, second, |a, b| a % b),
        Token::Str(_) => panic!("Cannot perform modular operation on Strings!"),
    }
}




    // let ret_type = most_generic_type(&first, &second);
    // let (farg, sarg) = match ret_type {
    //     Token::Int(_) => get_binary_args::<i64>(ret_type, first, second),
    //     Token::Float(_) => get_binary_args::<f64>(ret_type, first, second),
    //     Token::Str(_) => get_binary_args::<String>(ret_type, first, second),
    // };

// fn get_binary_args<T>(ret_type: Token, first: Token, second: Token) -> (Token, Token) {
//     match ret_type {
//         Token::Int(_) => {
//             (Token::Int(value_from_token!(first, i64)), Token::Int(value_from_token!(second, i64)))
//         },
//         Token::Float(_) => {
//             (Token::Int(value_from_token!(first, f64)), Token::Int(value_from_token!(second, f64)))
//         },
//         Token::Str(_) => {
//             (Token::Int(value_from_token!(first, String)), Token::Int(value_from_token!(second, String)))
//         },
//     }
// }