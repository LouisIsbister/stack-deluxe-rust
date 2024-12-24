mod test;

mod utils;
mod lexer;
mod stack;

use lexer::{exec_stack, read_stack};
use utils::Token;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    // if two args are provided, the second is a 
    // 3 digit number that is the test case!
    
    match args.len() {
        1 => (), // exec normally
        2 => {   // file
            let fnum = &args[1];
            let input_path = format!("input/input-{}.txt", fnum);
            let expected_path = format!("expected/expected-{}.txt", fnum);

            // read the files in
            let (input_content, _) = get_file_contents(&input_path);
            let (_, expected_stack_format) = get_file_contents(&expected_path);

            // execute the input file
            let input_res = exec(&input_content);

            println!("Expected: {:?}\nGot: {:?}", expected_stack_format, input_res);
            println!("Result: {}", expected_stack_format == input_res)
        }, 
        _ => (),
    }

}

fn exec(stack_str: &String) -> Vec<String> {
    let lexemes = read_stack(stack_str);
    let res = exec_stack(&lexemes);
    
    res.iter()
        .map(|val| extract_value_from_token!(val, String))
        .collect::<Vec<String>>()
}

fn get_file_contents(path: &str) -> (String, Vec<String>) {
    let fstr = std::fs::read_to_string(path);
    match fstr {
        Ok(contents) => (
            contents.clone(), 
            contents.split("\r\n")
                .filter(|val| !val.is_empty())
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
        ),
        Err(_) => panic!("Could not read the file '{}'", path),
    }
}



