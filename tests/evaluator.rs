use kl_rs::evaluator::{Evaluator, Object};
use kl_rs::{ast::AstNode, lexer::Lexer, parser::Parser};

#[test]
fn given_an_integer_expression_it_should_evaluate_to_the_right_object() {
    let test_codes = vec!["5", "10", "20"];
    let expected_objects = vec![Object::Integer(5), Object::Integer(10), Object::Integer(20)];

    test_codes.iter().enumerate().for_each(|(idx, code)| {
        let lexer = Lexer::new(code.to_string());
        let mut parser = Parser::new(lexer);
        let parsed_program = parser.parse_program();
        let node = match parsed_program {
            AstNode::Program { statements } => statements.first().unwrap().clone(),
            _ => panic!("Unexpected AstNode!"),
        };

        let evaluator = Evaluator::new();
        let evaluated_obj = evaluator.eval(node);

        assert_eq!(evaluated_obj, *expected_objects.get(idx).unwrap());
    })
}

#[test]
fn given_boolean_expressions_it_should_evaluate_to_the_right_object() {
    let test_codes = vec!["true", "false"];
    let expected_objects = vec![Object::Boolean(true), Object::Boolean(false)];

    test_codes.iter().enumerate().for_each(|(idx, code)| {
        let lexer = Lexer::new(code.to_string());
        let mut parser = Parser::new(lexer);
        let parsed_program = parser.parse_program();
        let node = match parsed_program {
            AstNode::Program { statements } => statements.first().unwrap().clone(),
            _ => panic!("Unexpected AstNode!"),
        };

        let evaluator = Evaluator::new();
        let evaluated_obj = evaluator.eval(node);

        assert_eq!(evaluated_obj, *expected_objects.get(idx).unwrap());
    })
}

#[test]
fn given_prefix_expressions_it_should_evaluate_correctly() {
    let test_codes = vec!["!true", "!false", "!!!!true", "-10", "!20"];
    let expected_objects = vec![
        Object::Boolean(false),
        Object::Boolean(true),
        Object::Boolean(true),
        Object::Integer(-10),
        Object::Null,
    ];

    test_codes.iter().enumerate().for_each(|(idx, code)| {
        let lexer = Lexer::new(code.to_string());
        let mut parser = Parser::new(lexer);
        let parsed_program = parser.parse_program();
        let node = match parsed_program {
            AstNode::Program { statements } => statements.first().unwrap().clone(),
            _ => panic!("Unexpected AstNode!"),
        };

        let evaluator = Evaluator::new();
        let evaluated_obj = evaluator.eval(node);

        assert_eq!(evaluated_obj, *expected_objects.get(idx).unwrap());
    })
}

#[test]
fn given_if_else_expressions_it_should_evaluate_correctly() {
    let test_codes = vec![
        "if (1 < 2) { 10 } else { 20 };",
        "if (true) { 10 } else { 20 };",
        "if (false) { 10 } else { 20 };",
        "if (200) { 10 } else { 20 };",
        "if (0) { 10 } else { 20 };",
        "if (false) { 10 };",
    ];
    let expected_objects = vec![
        Object::Integer(10),
        Object::Integer(10),
        Object::Integer(20),
        Object::Integer(10),
        Object::Integer(20),
        Object::Null,
    ];

    test_codes.iter().enumerate().for_each(|(idx, code)| {
        let lexer = Lexer::new(code.to_string());
        let mut parser = Parser::new(lexer);
        let parsed_program = parser.parse_program();
        let node = match parsed_program {
            AstNode::Program { statements } => statements.first().unwrap().clone(),
            _ => panic!("Unexpected AstNode!"),
        };

        let evaluator = Evaluator::new();
        let evaluated_obj = evaluator.eval(node);

        assert_eq!(evaluated_obj, *expected_objects.get(idx).unwrap());
    })
}
