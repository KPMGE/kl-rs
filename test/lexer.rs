#[test]
fn given_code_with_single_characters_it_should_parse_correctly() {
    let code = "{}()+=,";
    let expected_tokens = vec![
        Token::RightBrace,
        Token::LeftBrace,
        Token::RightParentesis,
        Token::LeftParentesis,
        Token::Plus,
        Token::Assign,
        Token::Comma,
        Token::Eof,
    ];

    let mut lexer = Lexer::new(code.to_string());

    (0..code.len()).for_each(|idx| {
        let token = lexer.next_token();
        let expected_token = &expected_tokens[idx];
        assert!(token == *expected_token);
    })
}
