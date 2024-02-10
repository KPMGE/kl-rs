mod lexer;
mod token;

use crate::lexer::Lexer;

fn main() {
    let code = "(),{}=";
    let mut lexer = Lexer::new(code.to_string());

    (0..code.len()).for_each(|_| {
        let token = lexer.next_token();
        println!("token: {:?}", token);
    })
}
