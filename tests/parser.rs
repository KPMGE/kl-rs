use kl_rs::{
    ast::AstNode,
    lexer::Lexer,
    parser::{BlockStatement, Expression, Parser, Statement},
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

    let expected_operators = vec![Token::Minus, Token::Bang];
    let expected_values = vec!["5", "20"];

    let parsed_program = parser.parse_program();

    assert_eq!(parsed_program.statements.len(), 2);
    assert_eq!(parser.errors.len(), 0);

    parsed_program
        .statements
        .iter()
        .enumerate()
        .for_each(|(idx, statement)| match statement {
            Statement::ExpressionStatement { token, value } => {
                let expected_operator = expected_operators.get(idx).unwrap();
                let expected_right_expression = Expression::Int {
                    token: Token::Int(expected_values.get(idx).unwrap().to_string()),
                };

                assert_eq!(token, expected_operators.get(idx).unwrap());
                match value {
                    Expression::Prefix { operator, right } => {
                        assert_eq!(operator, expected_operator);
                        assert_eq!(*right, Box::new(expected_right_expression));
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

#[test]
fn given_boolean_expression_it_should_parse_correctly() {
    let test_cases = vec!["true;", "false;", "!true;"];
    let expected_tokens = vec![Token::True, Token::False, Token::Bang];
    let expected_expressions = vec![
        Expression::Boolean { token: Token::True },
        Expression::Boolean {
            token: Token::False,
        },
        Expression::Prefix {
            operator: Token::Bang,
            right: Box::new(Expression::Boolean { token: Token::True }),
        },
    ];

    test_cases.iter().enumerate().for_each(|(idx, case)| {
        let token = expected_tokens.get(idx).unwrap().clone();
        let expression = expected_expressions.get(idx).unwrap();
        assert_boolean_expression(case, token, expression);
    });
}

#[test]
fn given_a_grouped_expression_it_should_parse_correctly() {
    let test_cases = vec![
        ("(1 + (2 + 3)) + 4", "((1 + (2 + 3)) + 4)"),
        ("-(5 + 5)", "(-(5 + 5))"),
    ];

    test_cases.iter().for_each(|(case, expected)| {
        let lexer = Lexer::new(case.to_string());
        let mut parser = Parser::new(lexer);

        let parsed_program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);

        let lexer = Lexer::new(expected.to_string());
        let mut parser = Parser::new(lexer);

        let expected_parsed_program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);

        let expression = match parsed_program.statements.first().unwrap() {
            Statement::ExpressionStatement { value, .. } => value,
            _ => panic!("Unexpected expression!"),
        };

        let expected_expression = match expected_parsed_program.statements.first().unwrap() {
            Statement::ExpressionStatement { value, .. } => value,
            _ => panic!("Unexpected expression!"),
        };

        assert_eq!(expression, expected_expression);
    });
}

#[test]
fn given_an_if_expression_it_should_parse_correctly() {
    let code = "if (x < y) { x }";
    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let expected_expression = Expression::IfExpression {
        token: Token::If,
        condition: Box::new(Expression::Infix {
            operator: Token::LessThan,
            left: Box::new(Expression::Identifier {
                token: Token::Identifier("x".to_string()),
            }),
            right: Box::new(Expression::Identifier {
                token: Token::Identifier("y".to_string()),
            }),
        }),
        consequence: BlockStatement {
            token: Token::LeftBrace,
            statements: vec![Statement::ExpressionStatement {
                token: Token::Identifier("x".to_string()),
                value: Expression::Identifier {
                    token: Token::Identifier("x".to_string()),
                },
            }],
        },
        alternative: None,
    };

    let parsed_program = parser.parse_program();

    assert_eq!(parsed_program.statements.len(), 1);
    assert_eq!(parser.errors.len(), 0);

    let statement = parsed_program.statements.first().unwrap();

    match statement {
        Statement::ExpressionStatement { token, value } => {
            assert_eq!(*token, Token::If);
            assert_eq!(*value, expected_expression);
        }
        _ => panic!("Unexpected statement!"),
    }
}

#[test]
fn given_an_if_else_expression_it_should_parse_correctly() {
    let code = "if (x < y) { x } else { y }";
    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let expected_expression = Expression::IfExpression {
        token: Token::If,
        condition: Box::new(Expression::Infix {
            operator: Token::LessThan,
            left: Box::new(Expression::Identifier {
                token: Token::Identifier("x".to_string()),
            }),
            right: Box::new(Expression::Identifier {
                token: Token::Identifier("y".to_string()),
            }),
        }),
        consequence: BlockStatement {
            token: Token::LeftBrace,
            statements: vec![Statement::ExpressionStatement {
                token: Token::Identifier("x".to_string()),
                value: Expression::Identifier {
                    token: Token::Identifier("x".to_string()),
                },
            }],
        },
        alternative: Some(BlockStatement {
            token: Token::LeftBrace,
            statements: vec![Statement::ExpressionStatement {
                token: Token::Identifier("y".to_string()),
                value: Expression::Identifier {
                    token: Token::Identifier("y".to_string()),
                },
            }],
        }),
    };

    let parsed_program = parser.parse_program();

    assert_eq!(parsed_program.statements.len(), 1);
    assert_eq!(parser.errors.len(), 0);

    let statement = parsed_program.statements.first().unwrap();

    match statement {
        Statement::ExpressionStatement { token, value } => {
            assert_eq!(*token, Token::If);
            assert_eq!(*value, expected_expression);
        }
        _ => panic!("Unexpected statement!"),
    }
}

fn assert_boolean_expression(code: &str, expected_token: Token, expected_expression: &Expression) {
    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let parsed_program = parser.parse_program();

    assert_eq!(parsed_program.statements.len(), 1);
    assert_eq!(parser.errors.len(), 0);

    let statement = parsed_program.statements.first().unwrap();

    match statement {
        Statement::ExpressionStatement { token, value } => {
            assert_eq!(*token, expected_token);
            assert_eq!(value, expected_expression);
        }
        _ => panic!(),
    }
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
