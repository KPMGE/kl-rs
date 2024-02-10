#[derive(Debug, Eq, PartialEq)]
enum Token {
    Illegal,
    Eof,
    Assign,
    Plus,
    Int(String),
    Identifier(String),
    LeftParentesis,
    Comma,
    RightParentesis,
    LeftBrace,
    RightBrace,
}

#[derive(Debug)]
struct Lexer {
    input: String,
    current_position: usize,
    read_position: usize,
    current_char: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {
            input,
            current_position: 0,
            read_position: 0,
            current_char: None,
        }
    }

    fn read_char(&mut self) {
        if self.read_position > self.input.len() {
            self.current_char = None;
            return;
        }

        let ch = self.input.chars().nth(self.read_position);

        self.current_char = ch;
        self.current_position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.read_char();

        match self.current_char {
            Some('(') => Token::RightParentesis,
            Some(')') => Token::LeftParentesis,
            Some('{') => Token::RightBrace,
            Some('}') => Token::LeftBrace,
            Some('=') => Token::Assign,
            Some('+') => Token::Plus,
            Some(',') => Token::Comma,
            None => Token::Eof,
            _ => Token::Illegal,
        }
    }
}

fn main() {}

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
