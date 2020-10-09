use crate::token::Token;

#[derive(Debug, Eq, PartialEq)]
pub struct Dependency {
    pub path: Token,
}
