use kl_rs::{lexer::Lexer, token::Token};

#[test]
fn given_code_with_single_characters_it_should_parse_correctly() {
    let code = "{}()+=,-!*/><;";
    let expected_tokens = vec![
        Token::RightBrace,
        Token::LeftBrace,
        Token::RightParentesis,
        Token::LeftParentesis,
        Token::Plus,
        Token::Assign,
        Token::Comma,
        Token::Minus,
        Token::Bang,
        Token::Asterisk,
        Token::Slash,
        Token::GreaterThan,
        Token::LessThan,
        Token::Semicolon,
        Token::Eof,
    ];

    let mut lexer = Lexer::new(code.to_string());

    (0..code.len()).for_each(|idx| {
        let token = lexer.next_token();
        let expected_token = &expected_tokens[idx];
        assert!(token == *expected_token);
    })
}

#[test]
fn given_code_with_keywords_it_should_parse_correctly() {
    let code = "fn let if else true false return";

    let expected_tokens = vec![
        Token::Function,
        Token::Let,
        Token::If,
        Token::Else,
        Token::True,
        Token::False,
        Token::Return,
    ];

    let mut lexer = Lexer::new(code.to_string());

    (0..expected_tokens.len()).for_each(|idx| {
        let token = lexer.next_token();
        let expected_token = &expected_tokens[idx];
        assert!(token == *expected_token);
    });
}