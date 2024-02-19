use downcast_rs::{Downcast, impl_downcast};

pub trait AstNode {
    fn get_token_literal(&self) -> String;
}

pub trait Expression: AstNode + Downcast {}
impl_downcast!(Expression);
