#[cfg(test)]
mod binary_operator_tests {

    use std::panic::catch_unwind;
    use crate::{stack::{self, apply_bin_op}, utils::Token};

    #[test]
    fn test_binary_add() {
        // test 5 + 5.0 = 10.0
        let mut stack = vec![
            Token::Int(5), Token::Float(5.0)
        ];
        apply_bin_op(&mut stack, stack::add);
        assert!(stack[0] == Token::Float(10.0));

        // test numbers can be concatenated as strings
        stack = vec![
            Token::Str("Hello".to_string()), Token::Int(5), Token::Str("World".to_string())
        ];
        apply_bin_op(&mut stack, stack::add);
        apply_bin_op(&mut stack, stack::add);

        assert!(stack[0] == Token::Str("Hello5World".to_string()))
    } 

    #[test]
    fn test_binary_mul() {
        // test 5 + 5.0 = 10.0
        let mut stack = vec![
            Token::Str("HI".to_string()), Token::Int(5)
        ];
        apply_bin_op(&mut stack, stack::mul);
        assert!(stack[0] == Token::Str("HIHIHIHIHI".to_string()));

        // test that muliplying a string by a float throws an err
        let re = catch_unwind(|| {
            apply_bin_op(
                &mut vec![Token::Str("HI".to_string()), Token::Float(1.0)], 
                stack::mul
            )
        });
        assert!(re.is_err());

        // test numbers can be concatenated as strings
        stack = vec![
            Token::Float(3.0), Token::Int(5), Token::Float(1.5)
        ];
        apply_bin_op(&mut stack, stack::mul);
        apply_bin_op(&mut stack, stack::mul);

        assert!(stack[0] == Token::Float(22.5))
    } 

    #[test]
    fn test_binary_div() {
        // test that dividing by 0 throws an err
        let re = catch_unwind(|| {
            apply_bin_op(
                &mut vec![Token::Int(1), Token::Int(0)], 
                stack::div
            )
        });
        assert!(re.is_err());

        // test numbers can be concatenated as strings
        let mut stack = vec![
            Token::Float(10.0), Token::Int(5), Token::Int(1)
        ];
        apply_bin_op(&mut stack, stack::div);
        assert_eq!(Token::Int(5), stack[stack.len() - 1]);
        
        apply_bin_op(&mut stack, stack::div);
        assert_eq!(Token::Float(2.0), stack[stack.len() - 1]);
    }

    #[test]
    fn test_binary_pow() {
        // test numbers can be concatenated as strings
        let mut stack = vec![Token::Float(10.0), Token::Int(3)];
        apply_bin_op(&mut stack, stack::exp);
        assert_eq!(Token::Float(1000.0), stack[0]);
        
        let mut stack = vec![Token::Int(7), Token::Int(2)];
        apply_bin_op(&mut stack, stack::exp);
        assert_eq!(Token::Int(49), stack[0]);
    }

}

#[cfg(test)]
mod stack_exec_tests {

    use crate::{
        lexer::read_stack, 
        stack::exec_stack,
        utils::Token
    };

    #[test]
    fn test_exec_drop_dup() {
        let input = "1 2 3 DROP DUP".to_string();
        let lexemes = read_stack(&input);

        let res = exec_stack(&lexemes);
        assert_eq!(res, vec![
            Token::Int(1), Token::Int(2), Token::Int(2)
        ])
    }

    #[test]
    fn test_exec_full_expr() {
        let input = "1.0 2 3 9 - + 2 * + 3 *".to_string();
        let lexemes = read_stack(&input);

        let res = exec_stack(&lexemes);
        assert_eq!(res, vec![Token::Float(-21.0)])
    }

    #[test]
    fn test_exec_rolld() {
        let input = "0 1 2 3 4 4 ROLLD".to_string();
        let lexemes = read_stack(&input);

        let res = exec_stack(&lexemes);
        assert_eq!(res, vec![
            Token::Int(0), Token::Int(4), Token::Int(1), Token::Int(2), Token::Int(3)
        ])
    }

    #[test]
    fn test_exec_equ() {
        let input = "1.0 1 ==".to_string();
        let lexemes = read_stack(&input);

        let res = exec_stack(&lexemes);
        assert_eq!(res[0], Token::Bool(true))
    }
    
}
