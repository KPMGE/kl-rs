use kl_rs::{
    ast::AstNode,
    lexer::Lexer,
    parser::{Expression, Parser, Statement},
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
        .for_each(|(idx, statement)| {
            let expected_identifier = expected_identifiers.get(idx).unwrap();
            let expected_int = expected_ints.get(idx).unwrap();

            let expected_statement = Statement::LetStatement {
                token: Token::Let,
                name: Expression::Identifier {
                    token: Token::Identifier(expected_identifier.to_string()),
                },
                value: Expression::Int {
                    token: Token::Int(expected_int.to_string()),
                },
            };

            assert_eq!(*statement, expected_statement);
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
        .for_each(|(idx, statement)| {
            let expected_int = expected_ints.get(idx).unwrap();
            let expected_expression = Expression::Int {
                token: Token::Int(expected_int.to_string()),
            };

            let expected_statement = Statement::ReturnStatement {
                token: Token::Return,
                value: expected_expression,
            };

            assert_eq!(*statement, expected_statement);
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

    parsed_program
        .statements
        .iter()
        .enumerate()
        .for_each(|(idx, statement)| match statement {
            Statement::ExpressionStatement { token, value } => {
                assert_eq!(*token, expected_operators[idx]);
                match value {
                    Expression::Prefix { operator, right } => {
                        assert_eq!(*operator, expected_operators[idx]);
                        assert_eq!(right.get_token_literal(), expected_values[idx]);
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        })
}

#[test]
fn given_infix_expressions_it_should_parse_correctly() {
    let infix_statements = vec![
        "5 + 6;", "10 - 5;", "2 < 3;", "2 > 3;", "4 * 5;", "5 / 7;", "8 == 9;", "4 != 2;",
    ];

    let expected_operators = vec![
        Token::Plus,
        Token::Minus,
        Token::LessThan,
        Token::GreaterThan,
        Token::Asterisk,
        Token::Slash,
        Token::Equals,
        Token::NotEquals,
    ];

    let expected_literals = vec![
        ("5", "6"),
        ("10", "5"),
        ("2", "3"),
        ("2", "3"),
        ("4", "5"),
        ("5", "7"),
        ("8", "9"),
        ("4", "2"),
    ];

    infix_statements
        .iter()
        .enumerate()
        .for_each(|(idx, statement)| {
            assert_infix_expression(
                statement,
                expected_operators.get(idx).unwrap().clone(),
                *expected_literals.get(idx).unwrap(),
            );
        });
}

fn assert_infix_expression(code: &str, expected_operator: Token, expected_literals: (&str, &str)) {
    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let parsed_program = parser.parse_program();

    assert_eq!(parsed_program.statements.len(), 1);
    assert_eq!(parser.errors.len(), 0);

    let statement = parsed_program.statements.first().unwrap();

    match statement {
        Statement::ExpressionStatement { value, .. } => {
            let expected_left_value = expected_literals.0;
            let expected_right_value = expected_literals.1;
            let expected_left_exp = Expression::Int {
                token: Token::Int(expected_left_value.to_string()),
            };
            let expected_right_exp = Expression::Int {
                token: Token::Int(expected_right_value.to_string()),
            };

            let expected_expression = Expression::Infix {
                operator: expected_operator,
                left: Box::new(expected_left_exp),
                right: Box::new(expected_right_exp),
            };

            assert_eq!(*value, expected_expression);
        }
        _ => panic!(),
    }
}
