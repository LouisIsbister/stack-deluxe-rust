/// Given a token and a type, extract the value from the
/// token and convert it to that type!
pub fn extract_value_from_token<T>(token: Token) -> T
where
    T: From<Token>,
{
    T::from(token)
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
            &Token::Bool(_) => 4,
        }
    }
}

impl From<Token> for bool {
    fn from(token: Token) -> Self {
        if let Token::Bool(val) = token {
            val
        } else {
            panic!("Cannot convert {:?} to bool!", token)
        }
    }
}

impl From<Token> for i64 {
    fn from(token: Token) -> Self {
        if let Token::Int(val) = token {
            val
        } else {
            panic!("Cannot convert {:?} to i64!", token)
        }
    }
}

impl From<Token> for f64 {
    fn from(token: Token) -> Self {
        match token {
            Token::Int(val) => val as f64,
            Token::Float(val) => val,
            _ => panic!("Cannot convert {:?} to f64!", token),
        }
    }
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        match token {
            Token::Str(val) => val,
            Token::Int(val) => val.to_string(),
            Token::Float(val) => val.to_string(),
            Token::Bool(val) => val.to_string(),
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
        Ok(val.to_string())
    }
}


/// given a string from the user, i.e. a value enclosed in quote,
/// remove the quote characters and convert back to string
pub fn parse_bool(val: &str) -> Result<bool, ()> {
    match val {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(()),
    }
}