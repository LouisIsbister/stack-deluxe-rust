/// Given a token and a type, extract the value from the
/// token and convert it to that type!
#[macro_export]
macro_rules! extract_value_from_token {
    ($token: expr, bool) => {{
        match $token {
            Token::Bool(val) => val, 
            _ => panic!("Cannot convert {:?} to bool!", $token)
        }}
    };
    ($token: expr, i64) => {{
        match $token {
            Token::Int(val) => val, 
            _ => panic!("Cannot convert {:?} to int!", $token)
        }}
    };
    ($token: expr, f64) => {{
        match $token {
            Token::Int(val) => val as f64, 
            Token::Float(val) => val,
            _ => panic!("Cannot convert {:?} to float!", $token)
        }}
    };
    ($token: expr, String) => {{
        match $token {
            Token::Int(val) => val.to_string(),
            Token::Float(val) => val.to_string(),
            Token::Str(val) => val.clone(),
            Token::Bool(val) => val.to_string(),
        }}
    };
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Int(i64), Float(f64), Str(String), Bool(bool)
}

impl Token {
    fn ord(&self) -> i8 {
        match self {
            &Token::Int(_) => 1,
            &Token::Float(_) => 2,
            &Token::Str(_) => 3,
            Token::Bool(_) => 4,
        }
    }
}


/// Given two tokens return the token with the greatest precedence 
/// in the type hierachy 
pub fn most_generic_type(first: &Token, second: &Token) -> Token {
    if first.ord() >= second.ord() {
        first.clone()
    } else {
        second.clone()
    }
}


/// given a string from the user, i.e. a value enclosed in quote,
/// remove the quote characters and convert back to string
pub fn parse_string(val: &str) -> Result<String, ()> {
    let chs = val.chars().collect::<Vec<char>>();
    if val.len() < 2 || chs[0] != '"' || chs[chs.len() - 1] != '"' {
        Err(())
    } else {
        Ok(val[1..val.len() - 1].to_string())
    }
}