use kl_rs::{parser::Parser, lexer::Lexer, ast::AstNode};

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
    (0..parsed_program.statements.len()).for_each(|idx| {
        let identifier = parsed_program.statements[idx].name.get_token_literal();
        let value = parsed_program.statements[idx].value.get_token_literal();

        let expected_identifier = expected_identifiers[idx];
        let expected_int = expected_ints[idx];

        assert_eq!(expected_int, value);
        assert_eq!(expected_identifier, identifier);
    })
}
