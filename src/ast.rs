pub trait AstNode {
    fn get_token_literal(&self) -> String;
}

pub trait Expression: AstNode {}
