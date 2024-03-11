use kl_rs::{
    ast::{AstNode, BlockStatement, Expression, Statement},
    lexer::Lexer,
    parser::Parser,
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

    assert_eq!(parser.errors.len(), 0);

    match parsed_program {
        AstNode::Program { statements } => {
            assert_eq!(statements.len(), 3);

            statements.iter().enumerate().for_each(|(idx, statement)| {
                let expected_identifier = expected_identifiers.get(idx).unwrap();
                let expected_int = expected_ints.get(idx).unwrap();

                let expected_statement = AstNode::Statement(Statement::LetStatement {
                    token: Token::Let,
                    name: Expression::Identifier {
                        token: Token::Identifier(expected_identifier.to_string()),
                    },
                    value: Expression::Int {
                        token: Token::Int(expected_int.to_string()),
                    },
                });

                assert_eq!(*statement, expected_statement);
            })
        }
        _ => panic!("Unexpected AstNode!"),
    }
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

    assert_eq!(parser.errors.len(), 0);

    match parsed_program {
        AstNode::Program { statements } => {
            assert_eq!(statements.len(), 3);

            statements.iter().enumerate().for_each(|(idx, statement)| {
                let expected_int = expected_ints.get(idx).unwrap();
                let expected_expression = Expression::Int {
                    token: Token::Int(expected_int.to_string()),
                };

                let expected_statement = AstNode::Statement(Statement::ReturnStatement {
                    token: Token::Return,
                    value: expected_expression,
                });

                assert_eq!(*statement, expected_statement);
            })
        }
        _ => panic!("Unexpected AstNode!"),
    }
}

#[test]
fn given_a_variable_name_it_should_parse_correctly() {
    let code = "foo;";

    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let parsed_program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    match parsed_program {
        AstNode::Program { statements } => {
            assert_eq!(statements.len(), 1);

            match statements.first().unwrap() {
                AstNode::Expression(expression) => {
                    assert_eq!(
                        *expression,
                        Expression::Identifier {
                            token: Token::Identifier("foo".to_string())
                        }
                    );
                }
                _ => panic!("wrong statement!"),
            }
        }
        _ => panic!("Unexpected AstNode!"),
    }
}

#[test]
fn given_a_number_expression_it_should_parse_correctly() {
    let code = "5;";

    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let parsed_program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    match parsed_program {
        AstNode::Program { statements } => {
            assert_eq!(statements.len(), 1);

            let statement = statements.first().unwrap();
            match statement {
                AstNode::Expression(expression) => {
                    assert_eq!(
                        *expression,
                        Expression::Int {
                            token: Token::Int("5".to_string())
                        }
                    );
                }
                _ => panic!("wrong statement!"),
            }
        }
        _ => panic!("Unexpected AstNode!"),
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

    assert_eq!(parser.errors.len(), 0);

    match parsed_program {
        AstNode::Program { statements } => {
            assert_eq!(statements.len(), 2);

            statements
                .iter()
                .enumerate()
                .for_each(|(idx, statement)| match statement {
                    AstNode::Expression(expression) => {
                        let expected_operator = expected_operators.get(idx).unwrap();
                        let expected_right_expression = Expression::Int {
                            token: Token::Int(expected_values.get(idx).unwrap().to_string()),
                        };

                        match expression {
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
        _ => panic!("Unexpected AstNode!"),
    }
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
    let expected_expressions = vec![
        Expression::Boolean {
            token: Token::True,
            value: true,
        },
        Expression::Boolean {
            token: Token::False,
            value: false,
        },
        Expression::Prefix {
            operator: Token::Bang,
            right: Box::new(Expression::Boolean {
                token: Token::True,
                value: true,
            }),
        },
    ];

    test_cases.iter().enumerate().for_each(|(idx, case)| {
        let expression = expected_expressions.get(idx).unwrap();
        assert_boolean_expression(case, expression);
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

        let expression = match parsed_program {
            AstNode::Program { ref statements } => match statements.first().unwrap() {
                AstNode::Expression(exp) => exp,
                _ => panic!("Unexpected expression!"),
            },
            _ => panic!("Unexpected AstNode!"),
        };

        let expected_expression = match expected_parsed_program {
            AstNode::Program { ref statements } => match statements.first().unwrap() {
                AstNode::Expression(exp) => exp,
                _ => panic!("Unexpected expression!"),
            },
            _ => panic!("Unexpected AstNode!"),
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
        condition: Box::new(AstNode::Expression(Expression::Infix {
            operator: Token::LessThan,
            left: Box::new(Expression::Identifier {
                token: Token::Identifier("x".to_string()),
            }),
            right: Box::new(Expression::Identifier {
                token: Token::Identifier("y".to_string()),
            }),
        })),
        consequence: BlockStatement {
            token: Token::LeftBrace,
            statements: vec![AstNode::Expression(Expression::Identifier {
                token: Token::Identifier("x".to_string()),
            })],
        },
        alternative: None,
    };

    let parsed_program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);
    match parsed_program {
        AstNode::Program { statements } => {
            assert_eq!(statements.len(), 1);

            let statement = statements.first().unwrap();
            match statement {
                AstNode::Expression(expression) => assert_eq!(*expression, expected_expression),
                _ => panic!("Unexpected statement!"),
            }
        }
        _ => panic!("Unexpeced AstNode!"),
    }
}

#[test]
fn given_an_if_else_expression_it_should_parse_correctly() {
    let code = "if (x < y) { x } else { y }";
    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let expected_expression = Expression::IfExpression {
        token: Token::If,
        condition: Box::new(AstNode::Expression(Expression::Infix {
            operator: Token::LessThan,
            left: Box::new(Expression::Identifier {
                token: Token::Identifier("x".to_string()),
            }),
            right: Box::new(Expression::Identifier {
                token: Token::Identifier("y".to_string()),
            }),
        })),
        consequence: BlockStatement {
            token: Token::LeftBrace,
            statements: vec![AstNode::Expression(Expression::Identifier {
                token: Token::Identifier("x".to_string()),
            })],
        },
        alternative: Some(BlockStatement {
            token: Token::LeftBrace,
            statements: vec![AstNode::Expression(Expression::Identifier {
                token: Token::Identifier("y".to_string()),
            })],
        }),
    };

    let parsed_program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    match parsed_program {
        AstNode::Program { statements } => {
            assert_eq!(statements.len(), 1);

            let statement = statements.first().unwrap();
            match statement {
                AstNode::Expression(expression) => assert_eq!(*expression, expected_expression),
                _ => panic!("Unexpected statement!"),
            }
        }
        _ => panic!("Unexpected AstNode!"),
    }
}

#[test]
fn given_a_function_expression_it_should_parse_correctly() {
    let code = "fn(a, b) { a + b };";
    let expected_expression = Expression::FunctionExpression {
        token: Token::Function,
        parameters: vec![
            Token::Identifier("a".to_string()),
            Token::Identifier("b".to_string()),
        ],
        body: BlockStatement {
            token: Token::LeftBrace,
            statements: vec![AstNode::Expression(Expression::Infix {
                operator: Token::Plus,
                left: Box::new(Expression::Identifier {
                    token: Token::Identifier("a".to_string()),
                }),
                right: Box::new(Expression::Identifier {
                    token: Token::Identifier("b".to_string()),
                }),
            })],
        },
    };

    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);
    let parsed_program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    match parsed_program {
        AstNode::Program { statements } => {
            assert_eq!(statements.len(), 1);

            let statement = statements.first().unwrap();

            match statement {
                AstNode::Expression(expression) => assert_eq!(*expression, expected_expression),
                _ => panic!("Unexpected expression!"),
            }
        }
        _ => panic!("Unexpected AstNode!"),
    }
}

#[test]
fn given_a_call_expression_it_should_parse_correctly() {
    let code = "add(2 * 3, 1 + 4);";

    let expected_expression = Expression::CallExpression {
        token: Token::LeftParentesis,
        function: Box::new(Expression::Identifier {
            token: Token::Identifier("add".to_string()),
        }),
        arguments: vec![
            Expression::Infix {
                operator: Token::Asterisk,
                left: Box::new(Expression::Int {
                    token: Token::Int("2".to_string()),
                }),
                right: Box::new(Expression::Int {
                    token: Token::Int("3".to_string()),
                }),
            },
            Expression::Infix {
                operator: Token::Plus,
                left: Box::new(Expression::Int {
                    token: Token::Int("1".to_string()),
                }),
                right: Box::new(Expression::Int {
                    token: Token::Int("4".to_string()),
                }),
            },
        ],
    };

    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let parsed_program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    match parsed_program {
        AstNode::Program { statements } => {
            assert_eq!(statements.len(), 1);

            let statement = statements.first().unwrap();
            match statement {
                AstNode::Expression(expression) => assert_eq!(*expression, expected_expression),
                _ => panic!("Unexpected statement!"),
            }
        }
        _ => panic!("Unexpected AstNode!"),
    }
}

fn assert_boolean_expression(code: &str, expected_expression: &Expression) {
    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let parsed_program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    match parsed_program {
        AstNode::Program { statements } => {
            assert_eq!(statements.len(), 1);

            let statement = statements.first().unwrap();

            match statement {
                AstNode::Expression(expression) => assert_eq!(expression, expected_expression),
                _ => panic!("Unexpected statement!"),
            }
        }
        _ => panic!("Unexpected AstNode!"),
    }
}

fn assert_infix_expression(code: &str, expected_operator: Token, expected_literals: (&str, &str)) {
    let lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer);

    let parsed_program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    match parsed_program {
        AstNode::Program { statements } => {
            assert_eq!(statements.len(), 1);

            let statement = statements.first().unwrap();
            match statement {
                AstNode::Expression(expression) => {
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

                    assert_eq!(*expression, expected_expression);
                }
                _ => panic!(),
            }
        }
        _ => panic!("Unexpected AstNode!"),
    }
}
