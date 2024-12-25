// use crate::extract_value_from_token;
use crate::lexer;
use crate::utils::{most_generic_type, extract_value_from_token, Token};


/// Given a list of strings, sequentially iterate 
/// the list, adding data to the stack and applying 
/// operators when they are encountered
/// 
/// ```
/// let stack_str = "1 2 DUP + +";
/// let lexemes = lexer::read_stack(&stack_str);
/// let res = stack::exec_stack(&lexemes);
/// assert_eq!(Token::Int(5), res[0])
/// ```
pub fn exec_stack(lexemes: &Vec<String>) -> Vec<Token> {
    let mut stack = Vec::new();

    for val in lexemes {
        if let Some(tok) = lexer::parse_data_type(val) {
            stack.push(tok);
            continue;
        }
        perform_token_operation(&mut stack, val);
    }
    stack
}

fn perform_token_operation(mut stack: &mut Vec<Token>, val: &String) {
    match val.as_str() {
        "+" => apply_bin_op(&mut stack, add),
        "-" => apply_bin_op(&mut stack, sub),
        "*" => apply_bin_op(&mut stack, mul),
        "/" => apply_bin_op(&mut stack, div),
        "**" => apply_bin_op(&mut stack, exp),
        "%" => apply_bin_op(&mut stack, modu),

        "<<" => apply_bin_op(&mut stack, bitl),
        ">>" => apply_bin_op(&mut stack, bitr),

        "==" => apply_bin_op(&mut stack, equequ),
        "!=" => apply_bin_op(&mut stack, notequ),
        ">" => apply_bin_op(&mut stack, gt),
        "<" => apply_bin_op(&mut stack, lt),
        ">=" => apply_bin_op(&mut stack, ge),
        "<=" => apply_bin_op(&mut stack, le),

        "&" => apply_bin_op(&mut stack, and),
        "|" => apply_bin_op(&mut stack, or),
        "^" => apply_bin_op(&mut stack, xor),

        "DROP" => drop(&mut stack),
        "DUP" => dup(&mut stack),
        "SWAP" => swap(&mut stack),
        "ROT" => rot(&mut stack, 3),
        "ROLL" => rol(&mut stack),
        "ROLLD" => rold(&mut stack),
        "IFELSE" => ifelse(&mut stack),

        _ => panic!("Unsupported stack element: '{}'", val),
    }
}

///
/// VOID OPERATORS
/// 

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

pub fn rot(stack: &mut Vec<Token>, num_to_rot: usize) {
    assert!(stack.len() >= num_to_rot, "Cannot ROT without at least 3 elements!");
    let len = stack.len();
    let last = len - num_to_rot;
    let last_tok = stack[last].clone();

    for i in len - num_to_rot..len - 1 {
        stack[i] = stack[i + 1].clone()
    }
    stack[len - 1] = last_tok;
}

pub fn rol(stack: &mut Vec<Token>) {
    assert!(stack.len() >= 4, "Cannot ROLL without at least 4 elements!");
    let rot_count = extract_value_from_token::<i64>(stack.pop().expect("Unreachable!"));
    assert!(rot_count >= 0, "Cannot invoke ROLL with a neagtive arg!");

    rot(stack, rot_count as usize)
}

pub fn rold(stack: &mut Vec<Token>) {
    assert!(stack.len() >= 4, "Cannot ROT without at least 4 elements!");
    let rot_count = extract_value_from_token::<i64>(stack.pop().expect("Unreachable!"));
    assert!(rot_count >= 0, "Cannot invoke ROT with a neagtive arg!");
    
    let len = stack.len();
    let last = len - 1;
    let last_tok = stack[last].clone();

    for i in (len + 1 - rot_count as usize..len).rev() {
        stack[i] = stack[i - 1].clone()
    }
    stack[len - rot_count as usize] = last_tok;
}

pub fn ifelse(stack: &mut Vec<Token>) {
    let op = stack.pop().expect("");
    let val = extract_value_from_token::<bool>(op);
    if val {
        stack.remove(stack.len() - 1);
    } else {
        stack.remove(stack.len() - 2);
    }
}


///
/// BINARY OPERATIONS
///


/// 
/// NUMERIC operators
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
        extract_value_from_token::<i64>(first), 
        extract_value_from_token::<i64>(second)
    );
    let res = func(fv, sv);
    Token::Int(res)
}

fn bin_float_function(first: Token, second: Token, func: impl Fn(f64, f64) -> f64) -> Token {
    let (fv, sv) = (
        extract_value_from_token::<f64>(first), 
        extract_value_from_token::<f64>(second)
    );
    let res = func(fv, sv);
    Token::Float(res)
}

fn bin_string_function(first: Token, second: Token, func: impl Fn(String, String) -> String) -> Token {
    let (fv, sv) = (
        extract_value_from_token::<String>(first), 
        extract_value_from_token::<String>(second)
    );
    let res = func(fv, sv);
    Token::Str(res)
}

fn bin_bool_function<T>(
    first: Token, 
    second: Token, 
    func: impl Fn(T, T) -> bool
) -> Token 
where 
    T: From<Token>
{
    let (fv, sv) = (
        extract_value_from_token::<T>(first),
        extract_value_from_token::<T>(second)
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
            let sv = extract_value_from_token::<i64>(second);
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

pub fn bitl(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a << b),
        _ => panic!("Cannot perform bitwse operation on non ints!"),
    }
}

pub fn bitr(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a >> b),
        _ => panic!("Cannot perform bitwse operation on non ints!"),
    }
}


/// 
/// BOOLEAN operators
/// 
pub fn equequ(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_bool_function::<i64>(first, second, |a, b| a == b),
        Token::Float(_) => bin_bool_function::<f64>(first, second, |a, b| a == b),
        Token::Str(_) => bin_bool_function::<String>(first, second, |a, b| a == b),
        Token::Bool(_) => bin_bool_function::<bool>(first, second, |a, b| a == b),
    }
}

pub fn notequ(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_bool_function::<i64>(first, second, |a, b| a != b),
        Token::Float(_) => bin_bool_function::<f64>(first, second, |a, b| a != b),
        Token::Str(_) => bin_bool_function::<String>(first, second, |a, b| a != b),
        Token::Bool(_) => bin_bool_function::<bool>(first, second, |a, b| a != b),
    }
}

pub fn gt(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_bool_function::<i64>(first, second, |a, b| a > b),
        Token::Float(_) => bin_bool_function::<f64>(first, second, |a, b| a > b),
        _ => panic!("Cannot perform > on non numeric types!")
    }
}

pub fn lt(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_bool_function::<i64>(first, second, |a, b| a < b),
        Token::Float(_) => bin_bool_function::<f64>(first, second, |a, b| a < b),
        _ => panic!("Cannot perform < on non numeric types!")
    }
}

pub fn ge(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_bool_function::<i64>(first, second, |a, b| a >= b),
        Token::Float(_) => bin_bool_function::<f64>(first, second, |a, b| a >= b),
        _ => panic!("Cannot perform >= on non numeric types!")
    }
}

pub fn le(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_bool_function::<i64>(first, second, |a, b| a <= b),
        Token::Float(_) => bin_bool_function::<f64>(first, second, |a, b| a <= b),
        _ => panic!("Cannot perform <= on non numeric types!")
    }
}

pub fn and(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Bool(_) => bin_bool_function::<bool>(first, second, |a, b| a && b),
        _ => panic!("Cannot perform & on non bool types!")
    }
}

pub fn or(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Bool(_) => bin_bool_function::<bool>(first, second, |a, b| a || b),
        _ => panic!("Cannot perform | on non bool types!")
    }
}

pub fn xor(first: Token, second: Token) -> Token {
    match most_generic_type(&first, &second) {
        Token::Int(_) => bin_int_function(first, second, |a, b| a ^ b),
        Token::Bool(_) => bin_bool_function::<bool>(first, second, |a, b| a ^ b),
        _ => panic!("Cannot perform ^ on non bool or integer types!")
    }
}