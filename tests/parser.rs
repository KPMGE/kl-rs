use kl_rs::{
    ast::AstNode,
    lexer::Lexer,
    parser::{Parser, Statement, Expression},
    token::Token,
};

#[test]
fn given_let_statements_with_single_integers_shold_parse_correctly() {
    let code = "
        let foo = 10;\
        let bar = 20;\
        let baz = 30;\
    ";

    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);
    let expected_identifiers = &["foo", "bar", "baz"];
    let expected_ints = &["10", "20", "30"];

    let parsed_program = parser.parse_program();

    assert_eq!(parsed_program.statements.len(), 3);
    assert_eq!(parser.errors.len(), 0);

    parsed_program
        .statements
        .iter()
        .enumerate()
        .for_each(|(idx, statement)| match statement {
            Statement::LetStatement { value, name, .. } => {
                let expected_identifier = expected_identifiers[idx];
                let expected_int = expected_ints[idx];

                assert_eq!(expected_identifier, name.get_token_literal());
                assert_eq!(expected_int, value.get_token_literal());
            }
            _ => panic!("wrong statement!"),
        })
}

#[test]
fn given_return_statements_with_single_integers_shold_parse_correctly() {
    let code = "
        return 10;\
        return 20;\
        return 30;\
    ";

    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);
    let expected_ints = &["10", "20", "30"];

    let parsed_program = parser.parse_program();

    assert_eq!(parsed_program.statements.len(), 3);
    assert_eq!(parser.errors.len(), 0);

    parsed_program
        .statements
        .iter()
        .enumerate()
        .for_each(|(idx, statement)| match statement {
            Statement::ReturnStatement { value, .. } => {
                let expected_int = expected_ints[idx];

                assert_eq!(expected_int, value.get_token_literal());
            }
            _ => panic!("wrong statement!"),
        })
}

#[test]
fn given_a_variable_name_it_should_parse_correctly() {
    let code = "foo;";

    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let parsed_program = parser.parse_program();

    assert_eq!(parsed_program.statements.len(), 1);
    assert_eq!(parser.errors.len(), 0);

    let statement = parsed_program.statements.first().unwrap();
    match statement {
        Statement::ExpressionStatement { token, value } => {
            assert_eq!(*token, Token::Identifier("foo".to_string()));
            assert_eq!(value.get_token_literal(), "foo");
        }
        _ => panic!("wrong statement!"),
    }
}

#[test]
fn given_a_number_expression_it_should_parse_correctly() {
    let code = "5;";

    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let parsed_program = parser.parse_program();

    assert_eq!(parsed_program.statements.len(), 1);
    assert_eq!(parser.errors.len(), 0);

    let statement = parsed_program.statements.first().unwrap();
    match statement {
        Statement::ExpressionStatement { token, value } => {
            assert_eq!(*token, Token::Int("5".to_string()));
            assert_eq!(value.get_token_literal(), "5");
        }
        _ => panic!("wrong statement!"),
    }
}

#[test]
fn given_a_prefix_expression_it_should_parse_correctly() {
    let code = "
        -5;\
        !20;
    ";

    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let expected_operators = [Token::Minus, Token::Bang];
    let expected_values = ["5", "20"];

    let parsed_program = parser.parse_program();

    assert_eq!(parsed_program.statements.len(), 2);
    assert_eq!(parser.errors.len(), 0);

    parsed_program.statements
        .iter()
        .enumerate()
        .for_each(|(idx, statement)| {
            match statement {
                Statement::ExpressionStatement { token, value } => {
                    assert_eq!(*token, expected_operators[idx]);
                    match value {
                        Expression::Prefix { operator, right } => {
                            assert_eq!(*operator, expected_operators[idx]);
                            assert_eq!(right.get_token_literal(), expected_values[idx]);
                        }
                        _ => panic!()
                    }
                }
                _ => panic!()
            }
        })
}
